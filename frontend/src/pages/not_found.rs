use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div>
            <h1>{ "404 - Page Not Found" }</h1>
            <p>{ "The page you are looking for does not exist." }</p>
        </div>
    }
}
