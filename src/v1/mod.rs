mod state;

use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get, Router};
use state::AppState;

pub async fn v1_api() -> Router {
  let state = AppState::new().await;
  Router::new().route("/", get(testing)).with_state(Arc::new(state))
}

async fn testing(State(testing): State<Arc<AppState>>) -> impl IntoResponse {
  let result = sqlx::query!("SELECT * FROM users").fetch_all(&testing.db).await;
  dbg!(result);

  "hello"
}
