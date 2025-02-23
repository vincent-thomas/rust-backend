use sqlx::{PgPool, Pool, Postgres};

async fn db_conn() -> Pool<Postgres> {
  let url = std::env::var("DATABASE_URL").unwrap();
  let testing = PgPool::connect(&url).await.unwrap();
  sqlx::migrate!("./migrations").run(&testing).await.unwrap();
  testing
}

pub struct AppState {
  pub db: Pool<Postgres>,
}

impl AppState {
  pub async fn new() -> Self {
    Self { db: db_conn().await }
  }
}
