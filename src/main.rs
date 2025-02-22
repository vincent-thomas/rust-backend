mod state;

use axum::Router;
use std::{env, io, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
  let state = state::AppState::new().await;

  let router = Router::default().with_state(Arc::new(state));

  let port = env::var("PORT").unwrap_or("3000".into());
  let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

  axum::serve(listener, router).await
}
