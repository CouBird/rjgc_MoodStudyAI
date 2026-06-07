use sqlx::MySqlPool;

pub async fn ping(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await.map(|_| ())
}
