use axum::extract::Path;
use axum::Router;
use axum::routing::{delete, get, post};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", get(get_todos))
        .route("/todos/:todo", get(get_todo))
        .route("/todos/:todo", post(save_todo))
        .route("/todos/:todo", delete(delete_todo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_todos() {}

async fn get_todo(Path(todo): Path<String>) -> String { todo }

async fn save_todo() {}

async fn delete_todo() {}
