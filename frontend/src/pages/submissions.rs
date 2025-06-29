use yew::prelude::*;

#[function_component(SubmissionsPage)]
pub fn submissions_page() -> Html {
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
