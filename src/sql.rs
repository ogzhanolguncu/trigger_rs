use axum::{http::HeaderMap, Json};
use serde_json::{json, Value};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tracing::info;

use crate::utils::convert;

use sqlx::Type;

#[derive(Type, Debug, Clone, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TaskStatus {
    Created,
    Pending,
    Retry,
    Failed,
}

impl TaskStatus {
    fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Created => "CREATED",
            TaskStatus::Pending => "PENDING",
            TaskStatus::Retry => "RETRY",
            TaskStatus::Failed => "FAILED",
        }
    }
}

pub async fn create_table() {
    const DB_URL: &str = "sqlite://sqlite.db";

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS tasks (
        id TEXT PRIMARY KEY,
        status TEXT CHECK (status IN ('CREATED', 'PENDING', 'RETRY', 'FAILED')),
        created_at TIMESTAMP,
        updated_at TIMESTAMP,
        endpoint TEXT,
        header TEXT,
        body TEXT,
        attempts INT CHECK (attempts <= 5)
      );",
    )
    .execute(&db)
    .await
    .unwrap();

    println!("Create user table result: {:?}", result);
}

pub async fn add_task(message_id: &str, endpoint: &str, header: HeaderMap, body: Value) {
    const DB_URL: &str = "sqlite://sqlite.db";

    let dt = chrono::Utc::now();
    let timestamp = dt.timestamp();

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let result = sqlx::query("INSERT INTO tasks (id, status, created_at, updated_at, endpoint, header, body, attempts) VALUES (?, ?, ?, ?, ?, ?, ?, ?);")
		.bind(message_id)
		.bind(TaskStatus::Created.as_str())
		.bind(timestamp)
		.bind(timestamp)
		.bind(endpoint)
		.bind(convert(&header))
		.bind(serde_json::to_string_pretty(&body).unwrap())
		.bind(0)
		.execute(&db)
		.await
		.unwrap();

    info!("Add task result: {:?}", result);
}

pub async fn update_task(message_id: &str, status: TaskStatus) {
    const DB_URL: &str = "sqlite://sqlite.db";

    let dt = chrono::Utc::now();
    let timestamp = dt.timestamp();

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let result = sqlx::query("UPDATE tasks SET status = ?, updated_at = ? WHERE id = ?;")
        .bind(status.as_str())
        .bind(timestamp)
        .bind(message_id)
        .execute(&db)
        .await
        .unwrap();

    info!("Update task result: {:?}", result);
}
