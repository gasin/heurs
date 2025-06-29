use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

// ---------- Routing ----------

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Submit,
    #[at("/submissions")]
    Submissions,
    #[at("/test_cases")]
    TestCases,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Submit => html! { <SubmitPage /> },
        Route::Submissions => html! { <SubmissionsPage /> },
        Route::TestCases => html! { <TestCasesPage /> },
        Route::NotFound => html! { <h1>{"ページが見つかりません"}</h1> },
    }
}

// ---------- Pages ----------

#[function_component(SubmitPage)]
fn submit_page() -> Html {
    let source_code = use_state(|| String::new());
    let cases = use_state(|| 10u32);
    let parallel = use_state(|| 1u32);
    let result = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let on_source_code = {
        let source_code = source_code.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            source_code.set(input.value());
        })
    };
    let on_cases = {
        let cases = cases.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse() {
                cases.set(val);
            }
        })
    };
    let on_parallel = {
        let parallel = parallel.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse() {
                parallel.set(val);
            }
        })
    };

    let on_submit = {
        let source_code = source_code.clone();
        let cases = cases.clone();
        let parallel = parallel.clone();
        let result = result.clone();
        let loading = loading.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            loading.set(true);
            let source_code = (*source_code).clone();
            let cases = *cases;
            let parallel = *parallel;
            let result = result.clone();
            let loading = loading.clone();
            spawn_local(async move {
                let body = serde_json::json!({
                    "source_code": source_code,
                    "cases": cases,
                    "parallel": parallel,
                    "timeout": 10
                });
                let resp = Request::post("http://localhost:3000/api/run")
                    .header("Content-Type", "application/json")
                    .body(body.to_string())
                    .expect("body作成失敗")
                    .send()
                    .await;
                match resp {
                    Ok(r) => {
                        let text = r
                            .text()
                            .await
                            .unwrap_or_else(|_| "レスポンス取得失敗".to_string());
                        result.set(Some(text));
                    }
                    Err(e) => {
                        result.set(Some(format!("リクエスト失敗: {}", e)));
                    }
                }
                loading.set(false);
            });
        })
    };

    html! {
        <>
            <h1>{"Run API テスト"}</h1>
            <form onsubmit={on_submit}>
                <div style="margin-bottom:1em;">
                    <label>{"ソースコード"}</label><br/>
                    <textarea rows=10 cols=60 value={(*source_code).clone()} oninput={on_source_code} required=true />
                </div>
                <div style="margin-bottom:1em;">
                    <label>{"テストケース数"}</label><br/>
                    <input type="number" min=1 value={cases.to_string()} oninput={on_cases} required=true />
                </div>
                <div style="margin-bottom:1em;">
                    <label>{"並列数"}</label><br/>
                    <input type="number" min=1 value={parallel.to_string()} oninput={on_parallel} required=true />
                </div>
                <button type="submit" disabled={*loading}>{ if *loading { "送信中..." } else { "実行" } }</button>
            </form>
            <h2>{"結果"}</h2>
            <pre>{ result.as_ref().unwrap_or(&"".to_string()) }</pre>
        </>
    }
}

#[function_component(SubmissionsPage)]
fn submissions_page() -> Html {
    let submissions = vec![
        (1, "main.cpp", 95),
        (2, "solution.py", 100),
        (3, "algo.rs", 87),
    ];

    html! {
        <>
            <h1>{"Submissions 一覧"}</h1>
            <table style="width:100%;border-collapse:collapse;">
                <thead>
                    <tr><th>{"ID"}</th><th>{"ファイル名"}</th><th>{"スコア"}</th></tr>
                </thead>
                <tbody>
                    { for submissions.iter().cloned().map(|(id,f,score)| html! {
                        <tr>
                            <td style="border:1px solid #ccc;padding:4px;">{id}</td>
                            <td style="border:1px solid #ccc;padding:4px;">{f}</td>
                            <td style="border:1px solid #ccc;padding:4px;">{score}</td>
                        </tr>
                    }) }
                </tbody>
            </table>
        </>
    }
}

#[function_component(TestCasesPage)]
fn test_cases_page() -> Html {
    let metas = use_state(|| Vec::<TestCaseMeta>::new());
    let selected = use_state(|| Option::<TestCase>::None);

    // 初回ロードで一覧取得
    {
        let metas = metas.clone();
        use_effect(move || {
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
            || ()
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

// ---------- 共通ナビ ----------

#[function_component(NavBar)]
fn nav_bar() -> Html {
    html! {
        <nav style="margin-bottom:1em;">
            <Link<Route> to={Route::Submit} >{"[Submit]"}</Link<Route>>{" | "}
            <Link<Route> to={Route::Submissions} >{"[Submissions]"}</Link<Route>>{" | "}
            <Link<Route> to={Route::TestCases} >{"[TestCases]"}</Link<Route>>
        </nav>
    }
}

// ---------- Root Component ----------

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

#[derive(Clone, Deserialize, PartialEq)]
struct TestCaseMeta {
    id: i32,
    filename: String,
    created_at: String,
}

#[derive(Clone, Deserialize, PartialEq)]
struct TestCasesResponse {
    test_cases: Vec<TestCaseMeta>,
}

#[derive(Clone, Deserialize, PartialEq)]
struct TestCase {
    id: i32,
    filename: String,
    content: String,
    created_at: String,
}

#[derive(Deserialize)]
struct TestCaseResponse {
    test_case: TestCase,
}

fn main() {
    yew::Renderer::<App>::new().render();
}
