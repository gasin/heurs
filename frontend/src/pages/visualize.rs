use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::visualizer_host::{ParseFn, VisProblem, VisualizerHost};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Grid cell type
#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Start,
    Goal,
}

/// 問題固有の状態
struct GridPathState {
    n: usize,
    m: usize,
    grid: Vec<Cell>,  // n*m
    moves: Vec<char>, // 出力の経路
    start: (i32, i32),
    goal: (i32, i32),
}

impl VisProblem for GridPathState {
    fn max_step(&self) -> usize {
        self.moves.len()
    }

    fn draw(&self, step: usize, ctx: &CanvasRenderingContext2d) {
        // canvas サイズ調整
        let cell = 30.0;
        let width = self.m as f64 * cell;
        let height = self.n as f64 * cell;
        let canvas: HtmlCanvasElement = ctx.canvas().unwrap().dyn_into().unwrap();
        if canvas.width() as f64 != width || canvas.height() as f64 != height {
            canvas.set_width(width as u32);
            canvas.set_height(height as u32);
        }

        // 背景クリア
        ctx.set_fill_style_str("#ffffff");
        ctx.fill_rect(0.0, 0.0, width, height);

        // グリッド描画
        for y in 0..self.n {
            for x in 0..self.m {
                let idx = y * self.m + x;
                match self.grid[idx] {
                    Cell::Wall => {
                        ctx.set_fill_style_str("#606060");
                        ctx.fill_rect(x as f64 * cell, y as f64 * cell, cell, cell);
                    }
                    Cell::Start => {
                        ctx.set_fill_style_str("#6ab04c");
                        ctx.fill_rect(x as f64 * cell, y as f64 * cell, cell, cell);
                    }
                    Cell::Goal => {
                        ctx.set_fill_style_str("#eb4d4b");
                        ctx.fill_rect(x as f64 * cell, y as f64 * cell, cell, cell);
                    }
                    _ => {}
                }
                // grid lines
                ctx.set_stroke_style_str("#cccccc");
                ctx.stroke_rect(x as f64 * cell, y as f64 * cell, cell, cell);
            }
        }

        // パス描画
        let mut path: Vec<(i32, i32)> = Vec::new();
        let (mut x, mut y) = self.start;
        path.push((x, y));
        for &c in &self.moves[..step.min(self.moves.len())] {
            match c {
                'U' => y -= 1,
                'D' => y += 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => {}
            }
            path.push((x, y));
        }
        // 塗りつぶし
        ctx.set_fill_style_str("rgba(46, 134, 222,0.3)");
        for &(px, py) in &path {
            ctx.fill_rect(px as f64 * cell, py as f64 * cell, cell, cell);
        }
        // 現在位置
        if let Some(&(cx, cy)) = path.last() {
            ctx.set_stroke_style_str("#1e3799");
            ctx.set_line_width(3.0);
            ctx.stroke_rect(cx as f64 * cell, cy as f64 * cell, cell, cell);
        }
    }

    fn info(&self, step: usize) -> Option<String> {
        Some(format!("Step: {}/{}", step, self.moves.len()))
    }
}

// ========== パース関数 ==========
fn parse_input(text: &str) -> Option<(usize, usize, Vec<Cell>, (i32, i32), (i32, i32))> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header = lines.next()?;
    let mut it = header.split_whitespace();
    let n: usize = it.next()?.parse().ok()?;
    let m: usize = it.next()?.parse().ok()?;
    let mut grid: Vec<Cell> = Vec::with_capacity(n * m);
    let mut start = None;
    let mut goal = None;
    for (y, line) in lines.take(n).enumerate() {
        for (x, ch) in line.chars().take(m).enumerate() {
            let cell = match ch {
                '#' => Cell::Wall,
                'S' => {
                    start = Some((x as i32, y as i32));
                    Cell::Start
                }
                'G' => {
                    goal = Some((x as i32, y as i32));
                    Cell::Goal
                }
                _ => Cell::Empty,
            };
            grid.push(cell);
        }
    }
    Some((n, m, grid, start?, goal?))
}

fn parse_output(text: &str) -> Vec<char> {
    text.chars()
        .filter(|c| "UDLRudlr".contains(*c))
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

fn build_parser() -> ParseFn {
    Rc::new(|input, output| {
        let (n, m, grid, start, goal) = parse_input(input)?;
        let moves = parse_output(output);
        Some(Rc::new(GridPathState {
            n,
            m,
            grid,
            moves,
            start,
            goal,
        }) as Rc<dyn VisProblem>)
    })
}

#[function_component(VisualizePage)]
pub fn visualize_page() -> Html {
    let parser = use_memo((), |_| build_parser());
    html! {
        <div>
            <h1>{"Grid Path Visualizer"}</h1>
            <p>{"S: スタート, G: ゴール, #: 壁"}</p>
            <VisualizerHost
                parser={(*parser).clone()}
                initial_input={Some("5 5\nS....\n.....\n.....\n.....\n....G\n".to_string())}
                initial_output={Some("RRRRDDDD".to_string())}
            />
        </div>
    }
}
