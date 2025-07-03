use crate::pages::{
    not_found::NotFound, submission_detail::SubmissionDetail, submissions::Submissions,
    submit::SubmitPage, test_cases::TestCasesPage, visualize::VisualizePage,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/submit")]
    Submit,
    #[at("/submissions")]
    Submissions,
    #[at("/submissions/:id")]
    SubmissionDetail { id: i32 },
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
        Route::Submissions => html! { <Submissions /> },
        Route::SubmissionDetail { id } => html! { <SubmissionDetail id={id} /> },
        Route::TestCases => html! { <TestCasesPage /> },
        Route::Visualize => html! { <VisualizePage /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
