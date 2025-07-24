use crate::components::item_list_panel::ItemListPanel;
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
                if let Ok(resp) = Request::get("/api/test_cases").send().await {
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
                let url = format!("/api/test_cases/{}", id);
                if let Ok(resp) = Request::get(&url).send().await {
                    if let Ok(json) = resp.json::<TestCaseResponse>().await {
                        selected.set(Some(json.test_case));
                    }
                }
            });
        })
    };

    let render_item_row = Callback::from(|meta: TestCaseMeta| {
        html! {
            <>
                <td style="padding:4px;">{meta.id}</td>
                <td style="padding:4px;">{meta.filename}</td>
                <td style="padding:4px;">{crate::types::format_datetime_minute(&meta.created_at)}</td>
            </>
        }
    });

    let list_panel = html! {
        <ItemListPanel<TestCaseMeta>
            title="一覧"
            items={(*metas).clone()}
            selected_id={selected.as_ref().map(|s| s.id)}
            on_select={on_select}
            headers={vec!["ID".to_string(), "ファイル名".to_string(), "作成日時".to_string()]}
            render_item_row={render_item_row}
        />
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
            <h1>{"TestCases"}</h1>
            <div style="display:flex; justify-content:space-between; align-items:flex-start;">
                {list_panel}
                {detail_panel}
            </div>
        </>
    }
}
