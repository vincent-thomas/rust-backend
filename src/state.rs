use sqlx::{Pool, Sqlite, SqlitePool};

async fn sqlite_connection() -> Pool<Sqlite> {
  let testing = SqlitePool::connect("").await.unwrap();
  sqlx::migrate!("./migrations").run(&testing).await.unwrap();
  testing
}

pub struct AppState {
  db: Pool<Sqlite>,
}

impl AppState {
  pub async fn new() -> Self {
    Self { db: sqlite_connection().await }
  }
}
