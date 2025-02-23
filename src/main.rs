mod v1;

use axum::Router;
use std::{env, io};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
  dotenvy::dotenv().unwrap();

  let router = Router::new().nest("/v1", v1::v1_api().await);

  let port = env::var("PORT").unwrap_or("3000".into());
  let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

  axum::serve(listener, router).await
}
