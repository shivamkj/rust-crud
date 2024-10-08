use axum::http::StatusCode;
use once_cell::sync::OnceCell;
use serde_json::json;
use sqlx::postgres::PgPool;

pub const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:3000";

// Define a global static variable for the pool
pub static DB_CONNECTION_POOL: OnceCell<PgPool> = OnceCell::new();

#[inline]
pub fn get_pool() -> &'static PgPool {
  DB_CONNECTION_POOL.get().expect("Database pool is not initialized")
}

#[inline]
pub fn map_sqlx_error(e: sqlx::Error) -> (StatusCode, String) {
  (
    StatusCode::INTERNAL_SERVER_ERROR,
    json!({"success": false, "message": e.to_string()}).to_string(),
  )
}
