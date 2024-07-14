use crate::client::{gen_multi_item_bodies, spawn_app};
use crate::comparison::{check_response, gen_expected_result};
use restaurant::domain::DatabaseResponse;
use uuid::Uuid;

#[actix_rt::test]
async fn successfully_retrieve_orders_for_a_specific_table() {
    // Arrange
    let app = spawn_app().await;

    // Add Data to the database that we don't want to fetch
    let wrong_table_no = 1;
    let wrong_items = [("hamburger", 1), ("hamburger", 1), ("cola", 1)].to_vec();
    // need to clone items for late comparison
    let wrong_bodies = gen_multi_item_bodies(wrong_table_no, wrong_items);

    // Add Data to the database that we want to fetch
    let table_no = 3;
    let items = [("fries", 1), ("water", 1)].to_vec();
    let bodies = gen_multi_item_bodies(table_no, items.clone());

    // Act

    // Add table 1 orders
    let table_1_response = app.post_parallel_orders(wrong_bodies).await;

    // Add table 3 orders
    let table_3_response = app.post_parallel_orders(bodies).await;

    // All orders succeeded
    assert_eq!(true, table_1_response);
    assert_eq!(true, table_3_response);

    // Retrieve orders for table 3
    let response = app.query_table(table_no).await;

    // Assert OK response
    assert!(response.status().is_success());

    // retrieved json
    let saved = response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // get expected result
    let expected_result = gen_expected_result(table_no, items.clone());

    // Asset that the response is equivalent to the expected result
    assert_eq!(check_response(&saved, expected_result), true);

    // Check that the preparation time has been set
    for s in saved {
        assert_eq!((5..15).contains(&s.preparation_time), true);
    }
}

#[actix_rt::test]
async fn retrieving_empty_table_no_returns_empty() {
    // Arrange
    let app = spawn_app().await;

    // Add Data to the database that we don't want to fetch
    let table_no = 1;
    let items = [("hamburger", 1), ("fries", 1), ("cola", 1)].to_vec();

    let bodies = gen_multi_item_bodies(table_no, items);

    // Act

    // Push orders
    let push_response = app.post_parallel_orders(bodies).await;

    assert_eq!(true, push_response);

    // Retrieve orders for table 2 which should be empty
    let get_response = app.query_table(2).await;

    // Assert OK response
    assert!(get_response.status().is_success());

    // retrieved json
    let saved = get_response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // Assert that the response is empty
    assert! {saved.is_empty()}
}

#[actix_rt::test]
async fn successfully_retrieve_all_orders() {
    // Arrange
    let app = spawn_app().await;

    // Add order for first table the database
    let first_table_no = 1;
    let first_items = [("hamburger", 1), ("hamburger", 1), ("cola", 1)].to_vec();
    // need to clone items for late comparison
    let first_bodies = gen_multi_item_bodies(first_table_no, first_items.clone());

    // Add order for second table to the database
    let second_table_no = 3;
    let second_items = [("fries", 1), ("water", 1)].to_vec();
    // need to clone items for later comparison
    let second_bodies = gen_multi_item_bodies(second_table_no, second_items.clone());

    // Act

    // Add table 1 orders
    let first_table_response = app.post_parallel_orders(first_bodies).await;

    // Add table 3 orders
    let second_table_response = app.post_parallel_orders(second_bodies).await;

    // All orders succeeded
    assert_eq!(true, first_table_response);
    assert_eq!(true, second_table_response);

    // Retrieve orders for table 3
    let response = app.query_all().await;

    // Assert OK response
    assert!(response.status().is_success());

    // retrieved json
    let saved = response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // get expected result
    let mut expected_result = gen_expected_result(first_table_no, first_items.clone());
    let mut second_result = gen_expected_result(second_table_no, second_items.clone());
    expected_result.append(&mut second_result);

    // Asset that the response is equivalent to the expected result
    assert_eq!(check_response(&saved, expected_result), true);

    // Check that the preparation time has been set
    for s in saved {
        assert_eq!((5..15).contains(&s.preparation_time), true);
    }
}

#[actix_rt::test]
async fn successfully_retrieve_specific_items_for_table() {
    // Arrange
    let app = spawn_app().await;

    // Add order for first table the database
    let first_table_no = 1;
    let first_items = [("hamburger", 1), ("hamburger", 1), ("cola", 1)].to_vec();
    // need to clone items for late comparison
    let first_bodies = gen_multi_item_bodies(first_table_no, first_items.clone());

    // Add order for second table to the database
    let second_table_no = 3;
    let second_items = [("fries", 1), ("water", 1)].to_vec();
    // need to clone items for later comparison
    let second_bodies = gen_multi_item_bodies(second_table_no, second_items.clone());

    // Act

    // Add table 1 orders
    let first_table_response = app.post_parallel_orders(first_bodies).await;

    // Add table 3 orders
    let second_table_response = app.post_parallel_orders(second_bodies).await;

    // All orders succeeded
    assert_eq!(true, first_table_response);
    assert_eq!(true, second_table_response);

    // Retrieve orders for table 3
    let response = app
        .query_item(first_table_no, "hamburger".to_string())
        .await;

    // set the expected result
    let expected_result = [(1, "hamburger", 1), (1, "hamburger", 1)];

    // Assert OK response
    assert!(response.status().is_success());

    // retrieved json
    let saved = response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // Asset that the response is equivalent to the expected result
    assert_eq!(check_response(&saved, expected_result.to_vec()), true);

    // Check that the preparation time has been set
    for s in saved {
        assert_eq!((5..15).contains(&s.preparation_time), true);
    }
}

#[actix_rt::test]
async fn successfully_retrieve_specific_item_for_table() {
    // Arrange
    let app = spawn_app().await;

    // Add order for first table the database
    let first_table_no = 1;
    let first_items = [("hamburger", 1), ("hamburger", 1), ("cola", 1)].to_vec();
    // need to clone items for late comparison
    let first_bodies = gen_multi_item_bodies(first_table_no, first_items.clone());

    // Add order for second table to the database
    let second_table_no = 3;
    let second_items = [("fries", 1), ("water", 1)].to_vec();
    // need to clone items for later comparison
    let second_bodies = gen_multi_item_bodies(second_table_no, second_items.clone());

    // Act

    // Add table 1 orders
    let first_table_response = app.post_parallel_orders(first_bodies).await;

    // Add table 3 orders
    let second_table_response = app.post_parallel_orders(second_bodies).await;

    // All orders succeeded
    assert_eq!(true, first_table_response);
    assert_eq!(true, second_table_response);

    // Retrieve orders for table 3
    let response = app.query_item(second_table_no, "fries".to_string()).await;

    // set the expected result
    let expected_result = [(3, "fries", 1)];

    // Assert OK response
    assert!(response.status().is_success());

    // retrieved json
    let saved = response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // Asset that the response is equivalent to the expected result
    assert_eq!(check_response(&saved, expected_result.to_vec()), true);

    // Check that the preparation time has been set
    for s in saved {
        assert_eq!((5..15).contains(&s.preparation_time), true);
    }
}

#[actix_rt::test]
async fn retrieving_item_not_in_table_returns_empty() {
    // Arrange
    let app = spawn_app().await;

    // Add Data to the database that we don't want to fetch
    let table_no = 1;
    let items = [("hamburger", 1), ("fries", 1), ("cola", 1)].to_vec();

    let bodies = gen_multi_item_bodies(table_no, items);

    // Act

    // Push orders
    let push_response = app.post_parallel_orders(bodies).await;

    assert_eq!(true, push_response);

    // Retrieve orders for table 2 which should be empty
    let get_response = app.query_item(1, "water".to_string()).await;

    // Assert OK response
    assert!(get_response.status().is_success());

    // retrieved json
    let saved = get_response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // Assert that the response is empty
    assert! {saved.is_empty()}
}

#[actix_rt::test]
async fn successfully_retrieve_specific_item_using_id() {
    // Arrange
    let app = spawn_app().await;

    // Add Data to the database including duplicate item to ensure id
    // match gets single instance of an item
    let table_no = 1;
    let items = [
        ("hamburger", 1),
        ("hamburger", 1),
        ("fries", 1),
        ("cola", 1),
    ]
    .to_vec();

    let bodies = gen_multi_item_bodies(table_no, items);

    // Act

    // Push orders
    let push_response = app.post_parallel_orders(bodies).await;

    assert_eq!(true, push_response);

    // Retrieve orders for table 2 which should be empty
    let get_response = app.query_item(1, "hamburger".to_string()).await;

    // Assert OK response
    assert!(get_response.status().is_success());

    // retrieved json
    let saved = get_response.json::<Vec<DatabaseResponse>>().await.unwrap();

    let id = saved[0].id;

    let get_id_response = app.query_id(id).await;

    let id_result = get_id_response
        .json::<Vec<DatabaseResponse>>()
        .await
        .unwrap();

    let expected_result = [(1, "hamburger", 1)];

    // Assert that the response is empty
    assert_eq!(check_response(&id_result, expected_result.to_vec()), true);
}

#[actix_rt::test]
async fn retrieving_id_not_in_table_returns_empty() {
    // Arrange
    let app = spawn_app().await;

    // Add Data to the database that we don't want to fetch
    let table_no = 1;
    let items = [("hamburger", 1), ("fries", 1), ("cola", 1)].to_vec();

    let bodies = gen_multi_item_bodies(table_no, items);

    // Act

    // Push orders
    let push_response = app.post_parallel_orders(bodies).await;

    assert_eq!(true, push_response);

    let id = Uuid::new_v4();

    // Retrieve orders for table 2 which should be empty
    let get_response = app.query_id(id).await;

    // Assert OK response
    assert!(get_response.status().is_success());

    // retrieved json
    let saved = get_response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // Assert that the response is empty
    assert! {saved.is_empty()}
}

#[actix_rt::test]
async fn successfully_retrieve_parallel_orders_for_a_specific_table() {
    // Arrange
    let app = spawn_app().await;

    // Add Data to the database that we don't want to fetch
    let wrong_table_no = 1;
    let wrong_items = [("hamburger", 1), ("hamburger", 1), ("cola", 1)].to_vec();
    // need to clone items for late comparison
    let wrong_bodies = gen_multi_item_bodies(wrong_table_no, wrong_items);

    // Add Data to the database that we want to fetch
    let table_no = 3;
    let items = [("fries", 1), ("water", 1)].to_vec();
    let bodies = gen_multi_item_bodies(table_no, items.clone());

    // Act

    // Add table 1 orders
    let table_1_response = app.post_parallel_orders(wrong_bodies).await;

    // Add table 3 orders
    let table_3_response = app.post_parallel_orders(bodies).await;

    // All orders succeeded
    assert_eq!(true, table_1_response);
    assert_eq!(true, table_3_response);

    // Retrieve parallel get responses for orders for table 3
    let responses = app.parallel_get_request(table_no).await;

    // Check that all responses have expected data
    for response in responses {
        // Assert OK response
        assert!(response.status().is_success());

        // retrieved json
        let saved = response.json::<Vec<DatabaseResponse>>().await.unwrap();

        // get expected result
        let expected_result = gen_expected_result(table_no, items.clone());

        // Asset that the response is equivalent to the expected result
        assert_eq!(check_response(&saved, expected_result), true);

        // Check that the preparation time has been set
        for s in saved {
            assert_eq!((5..15).contains(&s.preparation_time), true);
        }
    }
}
