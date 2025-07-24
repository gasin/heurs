use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="nav-bar">
            <Link<Route> classes="nav-link" to={Route::Submit}>{"Submit"}</Link<Route>>
            <Link<Route> classes="nav-link" to={Route::Submissions}>{"Submissions"}</Link<Route>>
            <Link<Route> classes="nav-link" to={Route::TestCases}>{"TestCases"}</Link<Route>>
            <Link<Route> classes="nav-link" to={Route::Visualize}>{"Visualize"}</Link<Route>>
        </nav>
    }
}
