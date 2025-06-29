use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::MouseEvent;
use yew::prelude::*;

#[function_component(VisualizePage)]
pub fn visualize_page() -> Html {
    let canvas_ref = use_node_ref();

    // アニメーション進捗（0.0〜100.0）を保持
    let progress = use_state(|| 0.0f64);
    let progress_ref = use_mut_ref(|| 0.0f64);

    // 実行状態
    let running = use_state(|| true);
    let running_ref = use_mut_ref(|| true);

    // スライダー操作ハンドラ
    let on_slider_change = {
        let progress = progress.clone();
        let progress_ref = progress_ref.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<f64>() {
                progress.set(val);
                *progress_ref.borrow_mut() = val;
            }
        })
    };

    // Stop / Restart ボタンハンドラ
    let on_stop = {
        let running = running.clone();
        let running_ref = running_ref.clone();
        Callback::from(move |_e: MouseEvent| {
            running.set(false);
            *running_ref.borrow_mut() = false;
        })
    };

    let on_restart = {
        let running = running.clone();
        let running_ref = running_ref.clone();
        Callback::from(move |_e: MouseEvent| {
            running.set(true);
            *running_ref.borrow_mut() = true;
        })
    };

    {
        let canvas_ref = canvas_ref.clone();
        let progress = progress.clone();
        let progress_ref = progress_ref.clone();
        let running_ref = running_ref.clone();
        use_effect_with((), move |_| {
            if let Some(canvas) = canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

                let width = 800.0;
                let height = 600.0;
                canvas.set_width(width as u32);
                canvas.set_height(height as u32);

                // 描画処理クロージャを16ms毎に実行（およそ60fps）
                let closure = Closure::wrap(Box::new(move || {
                    // 実行中のみ進捗を更新
                    let mut p = *progress_ref.borrow();
                    if *running_ref.borrow() {
                        p = (p + 0.5) % 100.0;
                        *progress_ref.borrow_mut() = p;
                        progress.set(p);
                    }

                    // 背景クリア
                    context.clear_rect(0.0, 0.0, width, height);

                    // タイルパターン描画
                    let grid_spacing = 40.0;
                    let num_x = (width / grid_spacing as f64).ceil() as i32;
                    let num_y = (height / grid_spacing as f64).ceil() as i32;
                    let color_light = "#f8f9fa";
                    let color_dark = "#e9ecef";
                    for i in 0..num_x {
                        for j in 0..num_y {
                            if (i + j) % 2 == 0 {
                                context.set_fill_style_str(color_light);
                            } else {
                                context.set_fill_style_str(color_dark);
                            }
                            context.fill_rect(
                                i as f64 * grid_spacing,
                                j as f64 * grid_spacing,
                                grid_spacing,
                                grid_spacing,
                            );
                        }
                    }

                    // 円描画（progressに応じて左右に移動）
                    let radius = 50.0;
                    let x = radius + (width - radius * 2.0) * (p / 100.0);
                    let y = height / 2.0;
                    context.begin_path();
                    context
                        .arc(x, y, radius, 0.0, std::f64::consts::PI * 2.0)
                        .unwrap();
                    context.set_fill_style_str("rgba(25, 135, 84, 0.8)");
                    context.fill();
                    context.set_stroke_style_str("#0f5132");
                    context.set_line_width(2.0);
                    context.stroke();
                }) as Box<dyn FnMut()>);

                let window = web_sys::window().unwrap();
                // 16ms (≒60fps) ごとに実行
                window
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        16,
                    )
                    .expect("setInterval failed");

                // メモリリーク防止を気にせずアプリ全体の寿命に任せる
                closure.forget();
            }
            || ()
        });
    }

    html! {
        <>
            <h1>{"Visualize"}</h1>
            <p>{"web-sysを使って描画したサンプルです。"}</p>
            <input type="range" min="0" max="100" value={progress.to_string()} oninput={on_slider_change} />
            <div style="margin-bottom:0.5em;">
                <button onclick={on_stop} disabled={!*running}>{"Stop"}</button>
                <button onclick={on_restart} disabled={*running}>{"Restart"}</button>
            </div>
            <canvas ref={canvas_ref} style="border: 1px solid #ccc;"></canvas>
        </>
    }
}
