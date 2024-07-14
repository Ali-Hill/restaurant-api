use crate::configuration::DatabaseSettings;
use crate::configuration::Settings;
use crate::routes::{
    delete_with_id, delete_with_item_name, health_check, order, query_all, query_with_id,
    query_with_item_name, query_with_table_number,
};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// A new type to hold the newly built server and its port
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture `connection` from the surrounding environment
    // move converts any variables captured by reference or mutable reference
    // to variables captured by value.a
    let server = HttpServer::new(move || {
        App::new()
            // Middlewares are added using the `wrap` method on `App`
            .wrap(TracingLogger::default())
            // health check route
            .route("/health_check", web::get().to(health_check))
            // place an order
            .route("/order", web::post().to(order))
            // query for all items in the database
            .route("/query_all", web::get().to(query_all))
            // query a specific item using unique id
            .route("/query_id/{id}", web::get().to(query_with_id))
            // query a specific item for a table in the database
            .route(
                "/query_item/{table_no}/{item}",
                web::get().to(query_with_item_name),
            )
            // query orders for a table in the database
            .route(
                "/query_table/{table_no}",
                web::get().to(query_with_table_number),
            )
            // delete item using unique id
            .route("/delete/{id}", web::delete().to(delete_with_id))
            // delete all occurences of an item from a specified table
            .route(
                "/delete_item/{table_no}/{item}",
                web::delete().to(delete_with_item_name),
            )
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    //Now run server instead of await
    Ok(server)
}
