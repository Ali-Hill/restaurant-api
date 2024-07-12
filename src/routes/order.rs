use crate::domain::{Item, Nat, NewOrder};
use crate::startup::ApplicationBaseUrl;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    table_no: i32,
    item: String,
    quantity: i32,
    preparation_time: i32,
}

impl TryFrom<FormData> for NewOrder {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let table_no = Nat::parse(value.table_no)?;
        let item = Item::parse(value.item)?;
        let quantity = Nat::parse(value.quantity)?;
        let preparation_time = Nat::parse(value.preparation_time)?;
        Ok(Self {
            table_no,
            item,
            quantity,
            preparation_time,
        })
    }
}

#[tracing::instrument(
  name = "Adding a new subscriber",
  skip(form, pool, base_url),
  fields(
    order_tableNo = %form.table_no,
    order_item = %form.item,
    order_quantity = %form.quantity,
    order_preparation_time = %form.preparation_time
  )
)]

pub async fn order(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> HttpResponse {
    let new_order = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_order(&pool, &new_order).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Adding new order to database", skip(order, pool))]
pub async fn insert_order(pool: &PgPool, order: &NewOrder) -> Result<(), sqlx::Error> {
    let order_id = Uuid::new_v4();
    sqlx::query!(
        r#"
    INSERT INTO orders (id, table_no, item, quantity, preparation_time, placed_at)
    VALUES ($1, $2, $3, $4, $5, $6)
    "#,
        order_id,
        order.table_no.as_ref(),
        order.item.as_ref(),
        order.quantity.as_ref(),
        order.preparation_time.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // // We will talk about error handling in depth later!
    })?;
    Ok(())
}
