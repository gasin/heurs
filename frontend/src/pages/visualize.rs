use wasm_bindgen::JsCast;
use yew::prelude::*;

#[function_component(VisualizePage)]
pub fn visualize_page() -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();
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

                // Draw tile pattern
                let grid_spacing = 40.0;
                let num_x = (width / grid_spacing as f64).ceil() as i32;
                let num_y = (height / grid_spacing as f64).ceil() as i32;

                let color_light = "#f8f9fa"; // Very light gray
                let color_dark = "#e9ecef"; // Slightly darker gray

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

                // Draw circle
                context.begin_path();
                context
                    .arc(
                        width / 2.0,
                        height / 2.0,
                        50.0,
                        0.0,
                        std::f64::consts::PI * 2.0,
                    )
                    .unwrap();
                context.set_fill_style_str("rgba(25, 135, 84, 0.8)");
                context.fill();
                context.set_stroke_style_str("#0f5132");
                context.set_line_width(2.0);
                context.stroke();
            }
        });
    }

    html! {
        <>
            <h1>{"Visualize"}</h1>
            <p>{"web-sysを使って描画したサンプルです。"}</p>
            <canvas ref={canvas_ref} style="border: 1px solid #ccc;"></canvas>
        </>
    }
}
