mod components;
mod pages;
mod router;
mod types;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav_bar::NavBar;
use crate::router::{Route, switch};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div style="max-width:960px; margin:2em auto; padding:2em; border:1px solid #ccc; border-radius:8px;">
                <NavBar />
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
