// This is a placeholder for the submission detail page.
// We will implement this in the next step.

use crate::types::SubmissionDetail as SubmissionDetailData; // Rename to avoid conflict
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

// Wrapper struct to match the API response { "submission": {...} }
#[derive(Deserialize)]
struct SubmissionResponse {
    submission: SubmissionDetailData,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(SubmissionDetail)]
pub fn submission_detail(props: &Props) -> Html {
    let submission = use_state(|| None);
    let error = use_state(|| None::<String>);

    {
        let submission = submission.clone();
        let error = error.clone();
        let id = props.id;

        use_effect_with((id,), move |_| {
            let url = format!("/api/submissions/{}", id);
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get(&url).send().await {
                    Ok(response) => {
                        if response.ok() {
                            match response.json::<SubmissionResponse>().await {
                                Ok(fetched_response) => {
                                    submission.set(Some(fetched_response.submission));
                                }
                                Err(e) => {
                                    error.set(Some(format!("Failed to parse submission: {}", e)))
                                }
                            }
                        } else {
                            error.set(Some(format!(
                                "API request failed: {}",
                                response.status_text()
                            )));
                        }
                    }
                    Err(e) => error.set(Some(format!("Failed to send request: {}", e))),
                }
            });
            || ()
        });
    }

    if let Some(err_msg) = &*error {
        return html! { <div class="alert alert-danger">{ err_msg }</div> };
    }

    match (*submission).as_ref() {
        Some(s) => html! {
            <div>
                <div class="card mb-4">
                    <div class="card-header">
                        { format!("Submission #{}", s.id) }
                    </div>
                    <div class="card-body">
                        <h5 class="card-title">{ "Summary" }</h5>
                        <p class="card-text">{ format!("Avg Score: {:.2}", s.average_score) }</p>
                        <p class="card-text">{ format!("Avg Time: {:.2} ms", s.average_execution_time_ms) }</p>
                        <p class="card-text">{ format!("Test Cases: {}", s.number_of_test_cases) }</p>
                        <hr />
                        <h5 class="card-title">{ "Source Code" }</h5>
                        <pre><code>{ &s.source_code }</code></pre>
                    </div>
                    <div class="card-footer text-muted">
                        { format!("Submitted at: {}", crate::types::format_datetime_minute(&s.created_at)) }
                    </div>
                </div>

                <h2>{ "Execution Results" }</h2>
                <table class="table table-striped">
                    <thead>
                        <tr>
                            <th>{ "Test Case ID" }</th>
                            <th>{ "Score" }</th>
                            <th>{ "Execution Time (ms)" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for s.execution_results.iter().map(|res| html! {
                            <tr>
                                <td>{ res.test_case_id }</td>
                                <td>{ res.score }</td>
                                <td>{ res.execution_time_ms }</td>
                            </tr>
                        }) }
                    </tbody>
                </table>
            </div>
        },
        None => html! { <p>{ "Loading submission details..." }</p> },
    }
}
