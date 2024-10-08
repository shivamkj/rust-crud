use axum::{
  routing::{delete, get, post, put},
  Router,
};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::net::TcpListener;
mod crud;
mod utils;

#[tokio::main]
async fn main() {
  // expose environment variables from .env file
  dotenvy::dotenv().expect("Unable to access .env file");

  // create our database pool
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");
  let db_pool = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(10))
    .connect(&database_url)
    .await
    .expect("can't connect to database");
  utils::DB_CONNECTION_POOL
    .set(db_pool)
    .expect("Failed to set the global pool");

  // create our tcp listener
  let server_address = std::env::var("SERVER_ADDRESS").unwrap_or(String::from(utils::DEFAULT_SERVER_ADDRESS));
  let listener = TcpListener::bind(server_address)
    .await
    .expect("Could not create tcp listener");

  println!("🌟 Listening on {} 🌟", listener.local_addr().unwrap());

  // compose the routes
  let app = Router::new()
    .route("/", get(|| async { "Hello world" }))
    .route("/tasks", get(crud::get_tasks))
    .route("/tasks", post(crud::create_task))
    .route("/tasks/:task_id", put(crud::update_task))
    .route("/tasks/:task_id", delete(crud::delete_task));

  // serve the application
  axum::serve(listener, app).await.expect("Error serving application");
}
