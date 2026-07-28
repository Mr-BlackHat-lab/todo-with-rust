// src/routes/todos.rs
use crate::models::todo::{CreateTodoReq, Todo};
use crate::{error::AppError, extractors::auth::AuthUser, state::AppState};
use axum::{Json, extract::State};

pub async fn create_todo(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser, // Fails automatically if token is invalid!
    Json(payload): Json<CreateTodoReq>,
) -> Result<Json<Todo>, AppError> {
    let todo = Todo::create(&state.db, &payload.title, user_id).await?;
    Ok(Json(todo))
}

pub async fn list_todos(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = Todo::list_for_user(&state.db, user_id).await?;
    Ok(Json(todos))
}
