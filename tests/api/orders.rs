use crate::client::{gen_body, gen_multi_item_bodies, spawn_app};

#[actix_rt::test]
async fn order_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = gen_body(1, "hamburger", 1);

    // Act
    let response = app.post_order(body).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn parallel_orders_succeed_and_persist() {
    // Arrange
    let app = spawn_app().await;
    let body = gen_body(1, "hamburger", 1);
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
        assert_eq!((5..15).contains(&s.preparation_time), true);
    }
}

#[actix_rt::test]
async fn order_multiple_fail() {
    // Arrange
    let app = spawn_app().await;
    let body = gen_body(1, "hamburger", 1);
    let fail_body = gen_body(1, "unicorn", 1);

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
    let body = gen_body(1, "hamburger", 1);

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
    assert_eq!((5..15).contains(&saved.preparation_time), true);
}

#[actix_rt::test]
async fn order_fails_when_missing_data() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("item=hamburger&quantity=1", "missing table number"),
        ("table_no=1&quantity=1&preparation_time=5", "missing item"),
        ("table_no=1&item=hamburger", "missing quantity"),
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
    let body = gen_body(1, "unicorn", 1);

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
        (gen_body(-1, "hamburger", 1), "Negative table number"),
        (gen_body(1, "hamburger", -2), "Negative quantity"),
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
async fn multi_item_order_succeeds_and_persists() {
    // Arrange
    let app = spawn_app().await;
    let table_no = 3;
    let items = [
        ("hamburger", 1),
        ("hamburger", 1),
        ("fries", 2),
        ("water", 1),
        ("cola", 1),
    ]
    .to_vec();
    // need to clone items for late comparison
    let bodies = gen_multi_item_bodies(table_no, items.clone());

    let num_bodies = bodies.len();

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

    // Since we do not know the order of saved due to parallel requests we have to be the same we have
    // check if items contain the expected order details.
    // The issue is that a client may be able to order the same item twice instead of
    // setting the quantity to 2. To test that the item is stored the correct amount times
    // we use a filter to count the number of times each item is stored.

    let item_counter = items.clone();

    // Check that each pair of item and quantity are contained the same number of times
    // in both the saved response and the item vector
    for i in items {
        assert_eq!(
            saved
                .iter()
                .filter(|s| (s.item == i.0 && s.quantity == i.1))
                .count(),
            item_counter
                .iter()
                .filter(|g| (g.0 == i.0 && g.1 == i.1))
                .count()
        );
    }

    // Check that all table numbers are correct and that preparation time has been set
    for s in saved {
        assert_eq!(s.table_no, table_no);
        assert_eq!((5..15).contains(&s.preparation_time), true);
    }
}
