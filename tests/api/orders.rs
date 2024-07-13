use crate::client::{gen_body, spawn_app};

#[actix_rt::test]
async fn order_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = gen_body(1, "hamburger", 1, 5);

    // Act
    let response = app.post_order(body).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn parallel_orders_succeed_and_persist() {
    // Arrange
    let app = spawn_app().await;
    let body = gen_body(1, "hamburger", 1, 5);
    let num_bodies = 20;

    // Checking that 20 orders succeed
    let bodies = vec![body; num_bodies];

    // Act
    let response = app.post_parallel_orders(bodies).await;

    let saved = sqlx::query!("SELECT table_no, item, quantity, preparation_time FROM orders",)
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to fetch inserted order.");

    // Assert

    // All orders succeeded
    assert_eq!(true, response);

    // Number of items in database is the same as the number of bodies
    assert_eq!(num_bodies, saved.len());

    // All orders persist
    for s in saved {
        assert_eq!(s.table_no, 1);
        assert_eq!(s.item, "hamburger");
        assert_eq!(s.quantity, 1);
        assert_eq!(s.preparation_time, 5);
    }
}

#[actix_rt::test]
async fn order_multiple_fail() {
    // Arrange
    let app = spawn_app().await;
    let body = gen_body(1, "hamburger", 1, 5);
    let fail_body = gen_body(1, "unicorn", 1, 5);

    let bodies = vec![body, fail_body];

    // Act
    let response = app.post_parallel_orders(bodies).await;

    let saved = sqlx::query!("SELECT table_no, item, quantity, preparation_time FROM orders",)
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to fetch inserted order.");

    // Assert
    // One of the orders has failed
    assert_eq!(false, response);

    // Number of items in database is the same as the number of successful bodies
    assert_eq!(1, saved.len());
}

#[actix_rt::test]
async fn order_persists() {
    // Arrange
    let app = spawn_app().await;
    let body = gen_body(1, "hamburger", 1, 5);

    // Act
    let response = app.post_order(body).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT table_no, item, quantity, preparation_time FROM orders",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch inserted order.");

    assert_eq!(saved.table_no, 1);
    assert_eq!(saved.item, "hamburger");
    assert_eq!(saved.quantity, 1);
    assert_eq!(saved.preparation_time, 5);
}

#[actix_rt::test]
async fn order_fails_when_missing_data() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            "item=hamburger&quantity=1&preparation_time=5",
            "missing table number",
        ),
        ("table_no=1&quantity=1&preparation_time=5", "missing item"),
        (
            "table_no=1&item=hamburger&preparation_time=5",
            "missing quantity",
        ),
        (
            "table_no=1&item=hamburger&quantity=1",
            "missing preparation time",
        ),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = app.post_order(invalid_body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

#[actix_rt::test]
async fn order_fails_with_invalid_item() {
    // Arrange
    let app = spawn_app().await;
    let body = gen_body(1, "unicorn", 1, 5);

    // Act
    let response = app.post_order(body).await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[actix_rt::test]
async fn order_fails_with_negative_numbers() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (gen_body(-1, "hamburger", 1, 5), "Negative table number"),
        (gen_body(1, "hamburger", -2, 5), "Negative quantity"),
        (gen_body(1, "hamburger", 1, -5), "Negative preparation time"),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = app.post_order(invalid_body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
