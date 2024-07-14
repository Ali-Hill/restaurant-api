use crate::client::{gen_body, gen_multi_item_bodies, spawn_app};
use crate::comparison::{check_response, gen_expected_result};
use restaurant::domain::DatabaseResponse;
use uuid::Uuid;

#[actix_rt::test]
async fn successfully_delete_item_using_id() {
    // Arrange
    let app = spawn_app().await;

    let table_no = 1;
    let items = [("hamburger", 2), ("fries", 1), ("cola", 1)].to_vec();

    let initial_body = gen_body(table_no, "hamburger", 1);
    let bodies = gen_multi_item_bodies(table_no, items.clone());

    // Act

    // Push initial order
    app.post_order(initial_body).await;

    // Retrieve initial body uuid
    let get_initial = app.query_all().await;

    // Initial value
    let saved = get_initial.json::<Vec<DatabaseResponse>>().await.unwrap();

    let id = saved[0].id;

    let push_response = app.post_parallel_orders(bodies).await;

    assert_eq!(true, push_response);

    // delete initial body from database
    app.delete_with_id(id).await;

    // Retrieve orders
    let get_response = app.query_all().await;

    // Assert OK response
    assert!(get_response.status().is_success());

    // retrieved json
    let saved = get_response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // get expected result
    let expected_result = gen_expected_result(table_no, items.clone());

    // Assert that the response is empty
    assert_eq!(check_response(&saved, expected_result.to_vec()), true);
}

#[actix_rt::test]
async fn delete_item_using_nonexistent_id_does_nothing() {
    // Arrange
    let app = spawn_app().await;

    let table_no = 1;
    let items = [("hamburger", 2), ("fries", 1), ("cola", 1)].to_vec();

    let bodies = gen_multi_item_bodies(table_no, items.clone());

    // Gen random id to be deleted
    let id = Uuid::new_v4();

    // Act
    let push_response = app.post_parallel_orders(bodies).await;

    assert_eq!(true, push_response);

    // delete initial body from database
    app.delete_with_id(id).await;

    // Retrieve orders
    let get_response = app.query_all().await;

    // Assert OK response
    assert!(get_response.status().is_success());

    // retrieved json
    let saved = get_response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // get expected result
    let expected_result = gen_expected_result(table_no, items.clone());

    // Assert that the response is empty
    assert_eq!(check_response(&saved, expected_result.to_vec()), true);
}

#[actix_rt::test]
async fn delete_item_name_deletes_all_items_from_matching_table() {
    // Arrange
    let app = spawn_app().await;

    // Adding to items of the same name to check both are deleted
    let table_no = 1;
    let items = [
        ("hamburger", 1),
        ("fries", 2),
        ("fries", 1),
        ("fries", 1),
        ("cola", 1),
    ]
    .to_vec();
    // Adding additional items with the same item name to another table
    let other_table_no = 3;
    let other_items = [("fries", 2), ("fries", 1)].to_vec();

    let mut bodies = gen_multi_item_bodies(table_no, items.clone());
    let mut other_bodies = gen_multi_item_bodies(other_table_no, other_items);
    bodies.append(&mut other_bodies);

    // Act
    let push_response = app.post_parallel_orders(bodies).await;

    assert_eq!(true, push_response);

    let expected_result = [
        (table_no, "hamburger", 1),
        (table_no, "cola", 1),
        (other_table_no, "fries", 2),
        (other_table_no, "fries", 1),
    ]
    .to_vec();

    // Delete orders
    app.delete_with_item(table_no, "fries".to_string()).await;

    // Retrieve orders
    let get_response = app.query_all().await;

    // Assert OK response
    assert!(get_response.status().is_success());

    // retrieved json
    let saved = get_response.json::<Vec<DatabaseResponse>>().await.unwrap();

    //assert_eq!(false, true);
    assert_eq!(check_response(&saved, expected_result), true);
}

#[actix_rt::test]
async fn delete_with_name_does_nothing_when_item_is_not_in_table() {
    // Arrange
    let app = spawn_app().await;

    let table_no = 1;
    let items = [("hamburger", 2), ("fries", 1), ("cola", 1)].to_vec();

    let bodies = gen_multi_item_bodies(table_no, items.clone());

    // Act
    let push_response = app.post_parallel_orders(bodies).await;

    assert_eq!(true, push_response);

    // delete water in table 1 which should do nothing
    app.delete_with_item(table_no, "water".to_string()).await;

    // Retrieve orders
    let get_response = app.query_all().await;

    // Assert OK response
    assert!(get_response.status().is_success());

    // retrieved json
    let saved = get_response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // get expected result
    let expected_result = gen_expected_result(table_no, items.clone());

    // Assert that the response is empty
    assert_eq!(check_response(&saved, expected_result.to_vec()), true);
}

// Testing that multiple clients can send delete requests at the same time
#[actix_rt::test]
async fn successfully_delete_items_in_parallel_for_a_specific_table() {
    // Arrange
    let app = spawn_app().await;

    // Adding to items of the same name to check both are deleted
    let body = gen_body(1, "hamburger", 1);
    let num_bodies = 20;

    // Checking that 20 orders can be deleted in parallel
    let bodies = vec![body; num_bodies];

    // Adding additional items with the same item name to another table
    let other_table_no = 3;
    let other_items = [("hamburger", 2), ("fries", 1), ("cola", 2)].to_vec();

    let other_bodies = gen_multi_item_bodies(other_table_no, other_items.clone());

    // Act

    // Add the bodies we want to delete
    let push_response = app.post_parallel_orders(bodies).await;

    assert_eq!(true, push_response);

    // Retrieve initial bodies
    let get_initial = app.query_all().await;

    // Initial value
    let saved = get_initial.json::<Vec<DatabaseResponse>>().await.unwrap();

    let mut delete_ids = Vec::new();

    for s in saved {
        delete_ids.push(s.id);
    }

    // Now add the additional data that we don't want to be deleted
    let additional_push_response = app.post_parallel_orders(other_bodies).await;

    assert_eq!(true, additional_push_response);

    // Delete orders in parallel
    app.parallel_delete_request(delete_ids).await;

    // Retrieve orders
    let get_response = app.query_all().await;

    // Assert OK response
    assert!(get_response.status().is_success());

    // retrieved json
    let saved = get_response.json::<Vec<DatabaseResponse>>().await.unwrap();

    //assert_eq!(false, true);
    assert_eq!(
        check_response(
            &saved,
            gen_expected_result(other_table_no, other_items.clone())
        ),
        true
    );
}
