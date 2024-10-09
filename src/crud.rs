use crate::utils::{get_pool, map_sqlx_error};
use axum::{extract::Path, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub async fn get_tasks() -> Result<(StatusCode, String), (StatusCode, String)> {
  let query = sqlx::query_as!(TaskRow, "SELECT * FROM tasks ORDER BY task_id");
  let rows = query.fetch_all(get_pool()).await.map_err(map_sqlx_error)?;

  Ok((StatusCode::OK, json!({"success": true, "data": rows}).to_string()))
}

pub async fn create_task(Json(task): Json<TaskReq>) -> Result<(StatusCode, String), (StatusCode, String)> {
  let query = sqlx::query_as!(
    CreateTaskRow,
    "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
    task.name,
    task.priority
  );
  let row = query.fetch_one(get_pool()).await.map_err(map_sqlx_error)?;

  Ok((StatusCode::CREATED, json!({"success": true, "data": row}).to_string()))
}

pub async fn update_task(
  Path(task_id): Path<i32>,
  Json(task): Json<TaskReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let query = sqlx::query("UPDATE tasks SET name = $1, priority = $2 WHERE task_id = $3")
    .bind(task.name)
    .bind(task.priority)
    .bind(task_id);
  query.execute(get_pool()).await.map_err(map_sqlx_error)?;

  Ok((StatusCode::OK, json!({"success":true}).to_string()))
}

pub async fn delete_task(Path(task_id): Path<i32>) -> Result<(StatusCode, String), (StatusCode, String)> {
  let query = sqlx::query!("DELETE FROM tasks WHERE task_id = $1", task_id);
  query.execute(get_pool()).await.map_err(map_sqlx_error)?;

  Ok((StatusCode::OK, json!({"success":true}).to_string()))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskRow {
  task_id: i32,
  name: String,
  priority: Option<i32>,
}

#[derive(Deserialize)]
pub struct TaskReq {
  name: String,

  #[serde(default = "default_priority")]
  priority: i32,
}

#[rustfmt::skip]
fn default_priority() -> i32 { 1 }

#[derive(Serialize)]
struct CreateTaskRow {
  task_id: i32,
}
