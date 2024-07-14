use actix_web::Result;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

// Handle result allowing for one place to change response handling
fn handle_delete_result(res: Result<(), sqlx::Error>) -> HttpResponse {
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Delete with unique id", skip(id, pool))]
pub async fn delete_with_id(id: web::Path<Uuid>, pool: web::Data<PgPool>) -> HttpResponse {
    let result = delete_id_request(&pool, &id).await;
    handle_delete_result(result)
}

#[tracing::instrument(name = "Delete item matching unique id from database", skip(id, pool))]
pub async fn delete_id_request(pool: &PgPool, id: &Uuid) -> Result<(), sqlx::Error> {
    sqlx::query_as!(
        DatabaseResponse,
        r#"
    DELETE FROM orders
    WHERE id = $1
    "#,
        id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Delete all matching items from table in the database",
    skip(args, pool)
)]
pub async fn delete_with_item_name(
    args: web::Path<(i32, String)>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let result = delete_item_request(&pool, &args.0, args.1.to_string()).await;
    handle_delete_result(result)
}

#[tracing::instrument(
    name = "Delete all matching items from table in the database sql request",
    skip(table_no, item, pool)
)]
pub async fn delete_item_request(
    pool: &PgPool,
    table_no: &i32,
    item: String,
) -> Result<(), sqlx::Error> {
    sqlx::query_as!(
        DatabaseResponse,
        r#"
    DELETE FROM orders
    WHERE table_no = $1 AND item = $2
    "#,
        table_no,
        item
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;
    Ok(())
}
