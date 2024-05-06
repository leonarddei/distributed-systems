use std::error::Error;
use std::time::Duration;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("postgres://admin:secret@localhost/todo").await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", get(get_todos))
        .route("/todos/:todo", get(get_todo))
        .route("/todos/:todo", post(save_todo))
        .route("/todos/:todo", delete(delete_todo))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_todos(State(pool): State<PgPool>) -> Result<Json<Vec<String>>, StatusCode> {
    sqlx::query_scalar("SELECT todo FROM todos")
        .fetch_all(&pool)
        .await
        .map(|todos| Json(todos))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_todo(Path(todo): Path<String>) -> String { todo }

async fn save_todo() {}

async fn delete_todo() {}
