use crate::pages::{
    submissions::SubmissionsPage, submit::SubmitPage, test_cases::TestCasesPage,
    visualize::VisualizePage,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Submit,
    #[at("/submissions")]
    Submissions,
    #[at("/test_cases")]
    TestCases,
    #[at("/visualize")]
    Visualize,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Submit => html! { <SubmitPage /> },
        Route::Submissions => html! { <SubmissionsPage /> },
        Route::TestCases => html! { <TestCasesPage /> },
        Route::Visualize => html! { <VisualizePage /> },
        Route::NotFound => html! { <h1>{"ページが見つかりません"}</h1> },
    }
}
