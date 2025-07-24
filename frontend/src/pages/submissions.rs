// use crate::components::code_block::CodeBlock; // No longer needed
use crate::components::item_list_panel::ItemListPanel;
use crate::types::{SubmissionDetail, SubmissionMeta};
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

// APIレスポンス用のラッパー構造体
#[derive(Deserialize)]
struct SubmissionsListResponse {
    submissions: Vec<SubmissionMeta>,
}

#[derive(Deserialize)]
struct SubmissionDetailResponse {
    submission: SubmissionDetail,
}

#[function_component(Submissions)]
pub fn submissions() -> Html {
    // 状態: 提出リストと選択された提出詳細
    let submission_metas = use_state(Vec::<SubmissionMeta>::new);
    let selected_submission = use_state(|| None::<SubmissionDetail>);
    let error = use_state(|| None::<String>);

    // 初回ロードでリストを取得
    {
        let submission_metas = submission_metas.clone();
        let selected_submission_state = selected_submission.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match Request::get("/api/submissions").send().await {
                    Ok(response) if response.ok() => {
                        match response.json::<SubmissionsListResponse>().await {
                            Ok(mut resp) => {
                                // Created At 降順にソート
                                resp.submissions
                                    .sort_by(|a, b| b.created_at.cmp(&a.created_at));

                                // state 更新
                                let first_id_opt = resp.submissions.first().map(|m| m.id);
                                submission_metas.set(resp.submissions);

                                // 先頭を自動選択
                                if let Some(first_id) = first_id_opt {
                                    // Fetch detail of first submission
                                    match Request::get(&format!("/api/submissions/{}", first_id))
                                        .send()
                                        .await
                                    {
                                        Ok(detail_resp) if detail_resp.ok() => {
                                            if let Ok(detail_json) =
                                                detail_resp.json::<SubmissionDetailResponse>().await
                                            {
                                                selected_submission_state
                                                    .set(Some(detail_json.submission));
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Err(e) => error.set(Some(format!("Parse error: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!("API error: {}", response.status()))),
                    Err(e) => error.set(Some(format!("Request error: {}", e))),
                }
            });
        });
    }

    // クリックハンドラ: IDを受け取り詳細を取得
    let on_select = {
        let selected_submission = selected_submission.clone();
        let error = error.clone();
        Callback::from(move |id: i32| {
            let selected_submission = selected_submission.clone();
            let error = error.clone();
            spawn_local(async move {
                let url = format!("/api/submissions/{}", id);
                match Request::get(&url).send().await {
                    Ok(response) if response.ok() => {
                        match response.json::<SubmissionDetailResponse>().await {
                            Ok(resp) => selected_submission.set(Some(resp.submission)),
                            Err(e) => error.set(Some(format!("Parse error: {}", e))),
                        }
                    }
                    Ok(response) => error.set(Some(format!("API error: {}", response.status()))),
                    Err(e) => error.set(Some(format!("Request error: {}", e))),
                }
            });
        })
    };

    // --- レンダリング ---

    if let Some(err_msg) = &*error {
        return html! { <div class="alert alert-danger">{ err_msg }</div> };
    }

    let render_item_row = Callback::from(|meta: SubmissionMeta| {
        html! {
            <>
                <td style="padding:4px;">{meta.id}</td>
                <td style="padding:4px;">{format!("{:.2}", meta.average_score)}</td>
                <td style="padding:4px;">{meta.number_of_test_cases}</td>
                <td style="padding:4px;">{crate::types::format_datetime_minute(&meta.created_at)}</td>
            </>
        }
    });

    // 左パネル: 提出リスト
    let list_panel = html! {
        <ItemListPanel<SubmissionMeta>
            title="Submissions"
            items={(*submission_metas).clone()}
            selected_id={selected_submission.as_ref().map(|s| s.id)}
            on_select={on_select}
            headers={vec![
                "ID".to_string(),
                "Avg Score".to_string(),
                "Test Cases".to_string(),
                "Created At".to_string(),
            ]}
            render_item_row={render_item_row}
        />
    };

    // 右パネル: 提出詳細
    let detail_panel = match selected_submission.as_ref() {
        Some(s) => html! {
            <div style="width:55%; padding-left: 1em;">
                <h2>{ "Details" }</h2>
                <div class="card">
                    <div class="card-header">{format!("#{} @ {}", s.id, crate::types::format_datetime_minute(&s.created_at))}</div>
                    <div class="card-body">
                        <h5 class="card-title">{ "Source Code" }</h5>
                        <div style="max-height: 400px; overflow-y: auto; background-color: #f8f9fa;">
                            <pre><code>{ &s.source_code }</code></pre>
                        </div>
                        <hr/>
                        <h5 class="card-title">{ "Execution Results" }</h5>
                        <div style="max-height: 300px; overflow-y: auto;">
                            <table class="table">
                                <thead>
                                    <tr>
                                        <th>{ "Test Case ID" }</th>
                                        <th>{ "Score" }</th>
                                        <th>{ "Execution Time (ms)" }</th>
                                    </tr>
                                </thead>
                                <tbody>
                                { for s.execution_results.iter().map(|res| html!{
                                    <tr>
                                        <td>{res.test_case_id}</td>
                                        <td>{res.score}</td>
                                        <td>{res.execution_time_ms}</td>
                                    </tr>
                                })}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>
        },
        None => html! {
            <div style="width:55%; display:flex; align-items:center; justify-content:center; color: #888;">
                <p>{"Select a submission to view details"}</p>
            </div>
        },
    };

    html! {
        <div>
            <h1>{ "Submissions" }</h1>
            <div style="display:flex; justify-content:space-between; align-items:flex-start;">
                { list_panel }
                { detail_panel }
            </div>
        </div>
    }
}
