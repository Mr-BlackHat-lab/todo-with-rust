// src/models/todo.rs
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct CreateTodoReq {
    pub title: String,
}

impl Todo {
    pub async fn create(db: &PgPool, title: &str, user_id: i32) -> Result<Self, AppError> {
        let todo = sqlx::query_as!(
            Todo,
            "INSERT INTO todos (title, user_id) VALUES ($1, $2) RETURNING id, title, completed, user_id",
            title,
            user_id
        )
        .fetch_one(db)
        .await?;
        Ok(todo)
    }

    pub async fn list_for_user(db: &PgPool, user_id: i32) -> Result<Vec<Self>, AppError> {
        let todos = sqlx::query_as!(
            Todo,
            "SELECT id, title, completed, user_id FROM todos WHERE user_id = $1",
            user_id
        )
        .fetch_all(db)
        .await?;
        Ok(todos)
    }
}
