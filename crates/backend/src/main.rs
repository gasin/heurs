use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

async fn get_users() -> Json<Vec<User>> {
    let users = vec![User {
        id: 1,
        name: "田中太郎".to_string(),
        email: "tanaka@example.com".to_string(),
    }];
    Json(users)
}

async fn create_user(Json(user): Json<User>) -> (StatusCode, Json<User>) {
    // データベースに保存する処理
    (StatusCode::CREATED, Json(user))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
