use gloo_net::http::Request;
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
        <div style="max-width:600px;margin:2em auto;padding:2em;border:1px solid #ccc;border-radius:8px;">
            <h1>{"Run API テスト"}</h1>
            <NavBar />
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
            <div style="margin-top:2em;">
                <h2>{"結果"}</h2>
                <pre>{ result.as_ref().unwrap_or(&"".to_string()) }</pre>
            </div>
        </div>
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
        <div style="max-width:600px;margin:2em auto;">
            <h1>{"Submissions 一覧"}</h1>
            <NavBar />
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
        </div>
    }
}

#[function_component(TestCasesPage)]
fn test_cases_page() -> Html {
    let cases = vec![(1, "case_1.in"), (2, "case_2.in"), (3, "case_3.in")];

    html! {
        <div style="max-width:600px;margin:2em auto;">
            <h1>{"TestCases 一覧"}</h1>
            <NavBar />
            <ul>
                { for cases.iter().cloned().map(|(id,name)| html! {<li>{format!("{} - {}", id, name)}</li>}) }
            </ul>
        </div>
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
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
