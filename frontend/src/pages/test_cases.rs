use crate::types::{TestCase, TestCaseMeta, TestCaseResponse, TestCasesResponse};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(TestCasesPage)]
pub fn test_cases_page() -> Html {
    let metas = use_state(|| Vec::<TestCaseMeta>::new());
    let selected = use_state(|| Option::<TestCase>::None);

    // 初回ロードで一覧取得
    {
        let metas = metas.clone();
        use_effect_with((), move |_| {
            let metas = metas.clone();
            spawn_local(async move {
                if let Ok(resp) = Request::get("http://localhost:3000/api/test_cases")
                    .send()
                    .await
                {
                    if let Ok(json) = resp.json::<TestCasesResponse>().await {
                        metas.set(json.test_cases);
                    }
                }
            });
        });
    }

    // 行クリックハンドラ
    let on_select = {
        let selected = selected.clone();
        Callback::from(move |id: i32| {
            let selected = selected.clone();
            spawn_local(async move {
                let url = format!("http://localhost:3000/api/test_cases/{}", id);
                if let Ok(resp) = Request::get(&url).send().await {
                    if let Ok(json) = resp.json::<TestCaseResponse>().await {
                        selected.set(Some(json.test_case));
                    }
                }
            });
        })
    };

    let list_panel = html! {
        <div style="width:45%;">
            <h2>{"一覧"}</h2>
            <div style="max-height:400px; overflow-y:auto; border:1px solid #ccc;">
                <table style="width:100%;border-collapse:collapse;">
                    <thead>
                        <tr>
                            <th style="border-bottom:1px solid #ccc;padding:4px;text-align:left;">{"ID"}</th>
                            <th style="border-bottom:1px solid #ccc;padding:4px;text-align:left;">{"ファイル名"}</th>
                            <th style="border-bottom:1px solid #ccc;padding:4px;text-align:left;">{"作成日時"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for metas.iter().cloned().map(|m| {
                            let id = m.id;
                            let onclick = {
                                let on_select = on_select.clone();
                                Callback::from(move |_| on_select.emit(id))
                            };
                            let selected_id = selected.as_ref().map(|s| s.id);
                            let is_selected = selected_id == Some(m.id);
                            let base_style = "cursor:pointer; transition:background-color 0.2s ease;";
                            let style = if is_selected {
                                format!("{} {}", base_style, "background-color:#d0e0ff;")
                            } else {
                                base_style.to_string()
                            };

                            html! {
                                <tr class="test-case-row" {onclick} {style}>
                                    <td style="padding:4px;">{m.id}</td>
                                    <td style="padding:4px;">{m.filename}</td>
                                    <td style="padding:4px;">{m.created_at}</td>
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            </div>
        </div>
    };

    let detail_panel = if let Some(detail) = (*selected).clone() {
        html! {
            <div style="width:50%;border-left:1px solid #ccc;padding-left:1em;">
                <h2>{"詳細"}</h2>
                <p>{format!("ID: {}", detail.id)}</p>
                <p>{format!("File: {}", detail.filename)}</p>
                <pre style="white-space:pre-wrap;border:1px solid #ccc;padding:4px;max-height:400px;overflow:auto;">{detail.content.clone()}</pre>
            </div>
        }
    } else {
        html! { <div style="width:50%; padding-left:1em;">{"クリックして詳細を表示"}</div> }
    };

    html! {
        <>
            <h1>{"TestCases 一覧"}</h1>
            <div style="display:flex; justify-content:space-between; align-items:flex-start;">
                {list_panel}
                {detail_panel}
            </div>
        </>
    }
}
