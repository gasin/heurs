use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav style="margin-bottom:1em;">
            <Link<Route> to={Route::Submit} >{"[Submit]"}</Link<Route>>{" | "}
            <Link<Route> to={Route::Submissions} >{"[Submissions]"}</Link<Route>>{" | "}
            <Link<Route> to={Route::TestCases} >{"[TestCases]"}</Link<Route>>{" | "}
            <Link<Route> to={Route::Visualize} >{"[Visualize]"}</Link<Route>>
        </nav>
    }
}
