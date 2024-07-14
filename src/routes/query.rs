use crate::domain::DatabaseResponse;
use actix_web::{error, Result};
use actix_web::{web, HttpResponse};
use derive_more::{Display, Error};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
pub struct SqlError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for SqlError {}

// Handle result allowing for one place to change response handling
fn handle_get_result(
    res: Result<Vec<DatabaseResponse>, sqlx::Error>,
) -> Result<HttpResponse, SqlError> {
    match res {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(_) => Err(SqlError { name: "Sql error" }),
    }
}

#[tracing::instrument(name = "Query with table number", skip(table_no, pool))]
pub async fn query_with_table_number(
    table_no: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SqlError> {
    let result = query_table_request(&pool, &table_no).await;
    handle_get_result(result)
}

#[tracing::instrument(
    name = "Retrieving orders for a table from the database",
    skip(table_no, pool)
)]
pub async fn query_table_request(
    pool: &PgPool,
    table_no: &i32,
) -> Result<Vec<DatabaseResponse>, sqlx::Error> {
    sqlx::query_as!(
        DatabaseResponse,
        r#"
    SELECT * FROM orders
    WHERE table_no = $1
    "#,
        table_no
    )
    .fetch_all(pool)
    .await
}

#[tracing::instrument(name = "Query with table number", skip(args, pool))]
pub async fn query_with_item_name(
    args: web::Path<(i32, String)>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SqlError> {
    let result = query_item_request(&pool, &args.0, args.1.to_string()).await;
    handle_get_result(result)
}

#[tracing::instrument(
    name = "Retrieving all matching items from a table in the database",
    skip(table_no, item, pool)
)]
pub async fn query_item_request(
    pool: &PgPool,
    table_no: &i32,
    item: String,
) -> Result<Vec<DatabaseResponse>, sqlx::Error> {
    sqlx::query_as!(
        DatabaseResponse,
        r#"
    SELECT * FROM orders
    WHERE table_no = $1 AND item = $2
    "#,
        table_no,
        item
    )
    .fetch_all(pool)
    .await
}

#[tracing::instrument(name = "Query with unique id", skip(id, pool))]
pub async fn query_with_id(
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SqlError> {
    let result = query_id_request(&pool, &id).await;
    handle_get_result(result)
}

#[tracing::instrument(
    name = "Retrieving item matching unique id from database",
    skip(id, pool)
)]
pub async fn query_id_request(
    pool: &PgPool,
    id: &Uuid,
) -> Result<Vec<DatabaseResponse>, sqlx::Error> {
    sqlx::query_as!(
        DatabaseResponse,
        r#"
    SELECT * FROM orders
    WHERE id = $1
    "#,
        id
    )
    .fetch_all(pool)
    .await
}

#[tracing::instrument(name = "Query all items", skip(pool))]
pub async fn query_all(pool: web::Data<PgPool>) -> Result<HttpResponse, SqlError> {
    let result = query_all_request(&pool).await;
    handle_get_result(result)
}

#[tracing::instrument(name = "Retrieving all items from the database", skip(pool))]
pub async fn query_all_request(pool: &PgPool) -> Result<Vec<DatabaseResponse>, sqlx::Error> {
    sqlx::query_as!(
        DatabaseResponse,
        r#"
    SELECT * FROM orders
    "#
    )
    .fetch_all(pool)
    .await
}
