use std::rc::Rc;
use wasm_bindgen::JsCast;

use gloo_timers::callback::Interval;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlTextAreaElement, InputEvent, MouseEvent,
};
use yew::{functional::*, prelude::*};

/// 問題ごとに実装する描画用トレイト
pub trait VisProblem {
    /// 描画可能な最大ステップ数 (output の長さなど)
    fn max_step(&self) -> usize;
    /// `step` 時点での描画を行う
    fn draw(&self, step: usize, ctx: &CanvasRenderingContext2d);
    /// 追加情報(スコアなど)を返す。Noneのときは表示しない。
    fn info(&self, _step: usize) -> Option<String> {
        None
    }
}

/// parse(input, output) -> 問題固有 State を返す関数型
pub type ParseFn = Rc<dyn Fn(&str, &str) -> Option<Rc<dyn VisProblem>>>;

#[derive(Properties)]
pub struct VisualizerHostProps {
    /// Input/Output をパースして問題固有の状態を生成する関数
    pub parser: ParseFn,
    /// デフォルト入力
    #[prop_or_default]
    pub initial_input: Option<String>,
    /// デフォルト出力
    #[prop_or_default]
    pub initial_output: Option<String>,
}

impl PartialEq for VisualizerHostProps {
    fn eq(&self, _other: &Self) -> bool {
        // 関数ポインタ同士の比較は出来ないため常に true を返す
        true
    }
}

/// このコンポーネントの状態
struct State {
    step: usize,
    playing: bool,
    problem: Option<Rc<dyn VisProblem>>,
}

/// 状態を更新するためのアクション
enum Action {
    SetProblem(Rc<dyn VisProblem>),
    SetStep(usize),
    TogglePlay,
    NextStep,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        let max_step = next_state.problem.as_ref().map_or(0, |p| p.max_step());

        match action {
            Action::SetProblem(problem) => {
                next_state.problem = Some(problem);
                next_state.step = 0;
                next_state.playing = false;
            }
            Action::SetStep(step) => {
                next_state.step = step;
                next_state.playing = false; // 手動操作で停止
            }
            Action::TogglePlay => {
                if next_state.problem.is_some() {
                    next_state.playing = !next_state.playing;
                }
            }
            Action::NextStep => {
                if next_state.playing {
                    next_state.step = if next_state.step < max_step {
                        next_state.step + 1
                    } else {
                        0
                    }
                }
            }
        }
        next_state.into()
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            step: self.step,
            playing: self.playing,
            problem: self.problem.clone(),
        }
    }
}

/// 可視化共通 UI を提供するホストコンポーネント
#[function_component(VisualizerHost)]
pub fn visualizer_host(props: &VisualizerHostProps) -> Html {
    // 入力 / 出力 テキストエリア
    let input_text = {
        let init = props.initial_input.clone().unwrap_or_default();
        use_state(|| init)
    };
    let output_text = {
        let init = props.initial_output.clone().unwrap_or_default();
        use_state(|| init)
    };

    // 状態管理
    let state = use_reducer(|| State {
        step: 0,
        playing: false,
        problem: None,
    });

    let canvas_ref = use_node_ref();

    // テキストエリア oninput
    let on_input_change = {
        let input_text = input_text.clone();
        Callback::from(move |e: InputEvent| {
            let el: HtmlTextAreaElement = e.target_unchecked_into();
            input_text.set(el.value());
        })
    };
    let on_output_change = {
        let output_text = output_text.clone();
        Callback::from(move |e: InputEvent| {
            let el: HtmlTextAreaElement = e.target_unchecked_into();
            output_text.set(el.value());
        })
    };

    // Visualize ボタン
    let on_visualize = {
        let parse = props.parser.clone();
        let input_text = input_text.clone();
        let output_text = output_text.clone();
        let dispatch = state.dispatcher();
        Callback::from(move |_e: MouseEvent| {
            if let Some(state) = (parse)(&input_text, &output_text) {
                dispatch.dispatch(Action::SetProblem(state.clone()));
            }
        })
    };

    // スライダー変更
    let on_slider_change = {
        let dispatch = state.dispatcher();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<usize>() {
                dispatch.dispatch(Action::SetStep(v));
            }
        })
    };

    // 再生/停止ボタン
    let on_toggle_play = {
        let dispatch = state.dispatcher();
        Callback::from(move |_e: MouseEvent| {
            dispatch.dispatch(Action::TogglePlay);
        })
    };

    // 描画 side-effect
    {
        let canvas_ref = canvas_ref.clone();
        let problem = state.problem.clone();
        let step = state.step;

        let state_key = problem
            .as_ref()
            .map(|rc| Rc::as_ptr(rc) as *const () as usize);

        use_effect_with((step, state_key), move |_| {
            if let Some(p) = problem {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    let ctx = canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();
                    if canvas.width() == 0 {
                        canvas.set_width(800);
                        canvas.set_height(600);
                    }
                    let w = canvas.width() as f64;
                    let h = canvas.height() as f64;
                    ctx.set_fill_style_str("#ffffff");
                    ctx.fill_rect(0.0, 0.0, w, h);

                    p.draw(step, &ctx);
                }
            }
            || ()
        });
    }

    // 自動再生 effect
    {
        let dispatch = state.dispatcher();
        use_effect_with(state.playing, move |playing| {
            let handle = if *playing {
                Some(Interval::new(250, move || {
                    dispatch.dispatch(Action::NextStep);
                }))
            } else {
                None
            };

            move || drop(handle)
        });
    }

    let max_step = state.problem.as_ref().map_or(0, |p| p.max_step());
    let info_text = state.problem.as_ref().and_then(|p| p.info(state.step));

    html! {
        <div>
            <div style="display:flex; gap:1em; flex-wrap:wrap;">
                <div style="flex:1; min-width:300px;">
                    <label>{"Input"}</label><br/>
                    <textarea rows="8" cols="48" value={(*input_text).clone()} oninput={on_input_change} />
                </div>
                <div style="flex:1; min-width:300px;">
                    <label>{"Output"}</label><br/>
                    <textarea rows="8" cols="48" value={(*output_text).clone()} oninput={on_output_change} />
                </div>
            </div>
            <button onclick={on_visualize} style="margin-top:0.5em;">{"Visualize"}</button>
            { if max_step > 0 {
                html! {
                    <>
                    <input type="range" min="0" max={max_step.to_string()} value={state.step.to_string()} oninput={on_slider_change} style="width:70%; margin-top:1em;" />
                    <button onclick={on_toggle_play} style="margin-left:0.5em;">
                        { if state.playing { "Stop" } else { "Play" } }
                    </button>
                    </>
                }
            } else { html!{} } }
            <div style="margin-top:1em; display:flex; gap:1em; align-items:flex-start;">
                <canvas ref={canvas_ref} style="border:1px solid #ccc;" />
                { if let Some(txt) = info_text {
                    html! { <pre style="background:#f8f9fa; padding:0.5em; min-width:150px;">{txt}</pre> }
                } else { html!{} } }
            </div>
        </div>
    }
}
