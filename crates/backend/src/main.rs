mod handlers;
mod models;

use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(handlers::run::run_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
