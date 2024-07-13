use once_cell::sync::Lazy;
use restaurant::configuration::{get_configuration, DatabaseSettings};
use restaurant::startup::{get_connection_pool, Application};
use restaurant::telemetry::{get_user, init_user};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio;
use uuid::Uuid;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let user_name = "test".to_string();
    // We cannot assign the output of `get_user` to a variable based on the value of `TEST_LOG`
    // because the sink is part of the type returned by `get_user`, therefore they are not the
    // same type. We could work around it, but this is the most straight-forward way of moving forward.
    if std::env::var("TEST_LOG").is_ok() {
        let user = get_user(user_name, default_filter_level, std::io::stdout);
        init_user(user);
    } else {
        let user = get_user(user_name, default_filter_level, std::io::sink);
        init_user(user);
    };
});

pub struct TestClient {
    pub address: String,
    pub db_pool: PgPool,
    pub port: u16,
}

impl TestClient {
    pub async fn post_order(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/order", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    // Used to test app can handle multiple client requests at once
    // Also used to create post request with multiple items
    // returns false if any future returns a status code other than 200
    pub async fn post_parallel_orders(&self, bodies: Vec<String>) -> bool {
        let client = reqwest::Client::new();

        let mut handles = Vec::new();

        for body in bodies {
            let posturl = format!("{}/order", &self.address);
            let client = client.clone();
            let handle = tokio::spawn(async move {
                client
                    .post(&posturl)
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(body)
                    .send()
                    .await
            });
            handles.push(handle);
        }

        // TODO: check the length of the responses is the same length as the bodies
        let responses = futures::future::join_all(handles).await;

        let mut result = true;

        // TODO: add error handling for cases when no response
        for response in responses {
            match response {
                Ok(Ok(res)) => {
                    if res.status() != reqwest::StatusCode::OK {
                        result = false;
                        break;
                    }
                }
                Ok(Err(_)) => result = false,
                Err(_) => result = false,
            }
        }

        result
    }

    pub async fn query_table(&self, table_no: i32) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!("{}/query_table/{}", &self.address, table_no))
            .send()
            .await
            .expect("Failed to get data.")
    }

}

pub fn gen_body(table_no: i32, item: &str, quantity: i32) -> String {
    format!("table_no={}&item={}&quantity={}", table_no, item, quantity,)
}

pub fn gen_multi_item_bodies(table_no: i32, items: Vec<(&str, i32)>) -> Vec<String> {
    let mut orders = Vec::new();

    for item in items {
        orders.push(gen_body(table_no, &item.0, item.1));
    }

    orders
}

// Launch our application in the background
pub async fn spawn_app() -> TestClient {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        c
    };

    // Create and migrate the database
    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    // Get the port before spawning the application
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestClient {
        address,
        port: application_port,
        db_pool: get_connection_pool(&configuration.database),
    }
}

// TODO: Add cleanup to remove empty databases
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create Database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[cfg(test)]
mod tests {
    use crate::client::{gen_body, gen_multi_item_bodies};

    #[test]
    fn gen_body_is_eq() {
        let body = gen_body(1, "hamburger", 1);
        assert_eq!(body, "table_no=1&item=hamburger&quantity=1");
    }

    #[test]
    fn gen_multi_item_body_test() {
        let table_no = 1;
        let items = [("hamburger", 2), ("fries", 2), ("water", 1), ("cola", 1)].to_vec();

        let bodies = gen_multi_item_bodies(table_no, items);

        let expected_result = [
            "table_no=1&item=hamburger&quantity=2",
            "table_no=1&item=fries&quantity=2",
            "table_no=1&item=water&quantity=1",
            "table_no=1&item=cola&quantity=1",
        ]
        .to_vec();

        assert_eq!(bodies, expected_result);
    }
}
