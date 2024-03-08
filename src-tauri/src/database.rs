use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    id: u32,
    name: String,
    age: u16,
    sex: String,
    address: String,
    time: String,
    status: String,
}

pub mod sqlite_database {
    use dotenv;
    use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

    #[tauri::command]
    pub async fn create_database() -> Result<String, ()> {
        let database_url: String = dotenv::var("DATABASE_URL").unwrap();
        Sqlite::create_database(&database_url).await.unwrap();
        Ok("数据库创建成功！".to_string())
    }

    async fn connect_database() -> Result<SqlitePool, sqlx::Error> {
        let database_url: String = dotenv::var("DATABASE_URL").unwrap();
        let pool: sqlx::Pool<Sqlite> = SqlitePool::connect(&database_url).await?;
        Ok(pool)
    }

    #[tauri::command]
    pub async fn create_table() -> Result<String, ()> {
        let pool: sqlx::Pool<Sqlite>;
        match connect_database().await {
            Ok(p) => {
                pool = p;
            }
            Err(_) => return Ok("数据库连接失败！".to_string()),
        }
        let _ = sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT, 
                name TEXT NOT NULL, 
                age INTEGER NOT NULL, 
                sex TEXT NOT NULL, 
                address TEXT NOT NULL,
                time TEXT NOT NULL,
                status TEXT NOT NULL
            );
        ",
        )
        .execute(&pool)
        .await;
        Ok("表格创建成功！".to_string())
    }

    #[tauri::command]
    pub async fn insert_data(
        name: String,
        age: u16,
        sex: String,
        address: String,
        time: String,
        status: String,
    ) -> Result<String, ()> {
        let pool: sqlx::Pool<Sqlite>;
        match connect_database().await {
            Ok(p) => {
                pool = p;
            }
            Err(_) => return Ok("数据库连接失败！".to_string()),
        }
        let sql = format!("
            INSERT INTO users (name, age, sex, address, time, status) VALUES ('{}', {}, '{}', '{}', '{}', '{}')
        ", name, age, sex, address, time, status);
        let _ = sqlx::query(&sql).execute(&pool).await;
        Ok("添加成功！".to_string())
    }

    #[tauri::command]
    pub async fn select_data() -> Result<String, ()> {
        let pool: sqlx::Pool<Sqlite>;
        match connect_database().await {
            Ok(p) => {
                pool = p;
            }
            Err(_) => return Ok("数据库连接失败！".to_string()),
        }
        let sql = format!("SELECT * FROM users");
        match sqlx::query_as::<_, super::User>(&sql)
            .fetch_all(&pool)
            .await
        {
            Ok(rows) => {
                let mut result = String::new();
                result += "[";
                for row in rows {
                    result += "{";
                    result += &format!(
                        "\"id\": {}, \"name\": \"{}\", \"age\": {}, \"sex\": \"{}\", \"address\": \"{}\", \"time\": \"{}\", \"status\": \"{}\"",
                        row.id, row.name, row.age, row.sex, row.address, row.time, row.status
                    )
                    .as_str();
                    result += "},";
                }
                result.pop();
                result += "]";
                Ok(result)
            }
            Err(_) => Ok("查询失败！".to_string()),
        }
    }

    #[tauri::command]
    pub async fn delete_data(id: u32) -> Result<String, ()> {
        let pool: sqlx::Pool<Sqlite>;
        match connect_database().await {
            Ok(p) => {
                pool = p;
            }
            Err(_) => return Ok("数据库连接失败！".to_string()),
        }
        let sql = format!("DELETE FROM users WHERE id = {}", id);
        let _ = sqlx::query(&sql).execute(&pool).await;
        Ok("删除成功！".to_string())
    }

    #[tauri::command]
    pub async fn update_data(
        id: u32,
        name: String,
        age: u16,
        sex: String,
        address: String,
        time: String,
        status: String,
    ) -> Result<String, ()> {
        let pool: sqlx::Pool<Sqlite>;
        match connect_database().await {
            Ok(p) => {
                pool = p;
            }
            Err(_) => return Ok("数据库连接失败！".to_string()),
        }
        let sql = format!("
            UPDATE users SET name = '{}', age = {}, sex = '{}', address = '{}', time = '{}', status = '{}' WHERE id = {}
        ", name, age, sex, address, time, status, id);
        let _ = sqlx::query(&sql).execute(&pool).await;
        Ok("更新成功！".to_string())
    }
}
