use crate::domain::DatabaseResponse;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use actix_web::{error, Result};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
pub struct SqlError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for SqlError {}

#[tracing::instrument(
  name = "Query with table number",
  skip(table_no, pool),
)]
pub async fn query_with_table_number(
    table_no: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, SqlError> {

    match query_table_request(&pool, &table_no).await {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(_) => Err(SqlError{name: "Sql error"}),
    }
}

#[tracing::instrument(name = "Retrieving orders for a table from the database", skip(table_no, pool))]
pub async fn query_table_request(pool: &PgPool, table_no: &i32) -> Result<Vec<DatabaseResponse>, sqlx::Error> {
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
