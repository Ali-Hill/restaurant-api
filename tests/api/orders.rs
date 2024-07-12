use crate::helpers::spawn_app;

#[actix_rt::test]
async fn order_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "table_no=1&item=hamburger&quantity=1&preparation_time=5";

    // Act
    let response = app.post_order(body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn order_persists() {
    // Arrange
    let app = spawn_app().await;
    let body = "table_no=1&item=hamburger&quantity=1&preparation_time=5";

    // Act
    let response = app.post_order(body.into()).await;

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
    let body = "table_no=1&item=unicorn&quantity=1&preparation_time=5";

    // Act
    let response = app.post_order(body.into()).await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[actix_rt::test]
async fn order_fails_with_negative_numbers() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            "table_no=-1&item=hamburger&quantity=1&preparation_time=5",
            "Negative table number",
        ),
        (
            "table_no=1&item=hamburger&quantity=-2&preparation_time=5",
            "Negative quantity",
        ),
        (
            "table_no=1&item=hamburger&quantity=1&preparation_time=-5",
            "Negative preparation time",
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
