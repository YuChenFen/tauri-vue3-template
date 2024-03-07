use dotenv;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

// 实验性方法
#[tokio::main]
pub async fn create_database() -> Result<(), sqlx::Error> {
    let database_url: String = dotenv::var("DATABASE_URL").unwrap();
    Sqlite::create_database(&database_url).await.unwrap();
    let pool: sqlx::Pool<Sqlite> = SqlitePool::connect(&database_url).await?;
    println!("Database created!");
    let _f: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query("CREATE TABLE IF NOT EXISTS web (id,age,name)").execute(&pool).await;
    Ok(())
}