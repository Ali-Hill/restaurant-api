use crate::client::{gen_body, gen_multi_item_bodies, spawn_app};
use restaurant::domain::DatabaseResponse;

#[actix_rt::test]
async fn successfully_retrieve_orders_for_a_specific_table () {
    // Arrange
    let app = spawn_app().await;

    // Add Data to the database that we don't want to fetch
    let wrong_table_no = 1;
    let wrong_items = [
        ("hamburger", 1),
        ("hamburger", 1),
        ("cola", 1),
    ]
    .to_vec();
    // need to clone items for late comparison
    let wrong_bodies = gen_multi_item_bodies(wrong_table_no, wrong_items);

    // Add Data to the database that we want to fetch
    let table_no = 3;
    let items = [
        ("fries", 1),
        ("water", 1),
    ]
    .to_vec();
    let item_counter = items.clone();
    // need to clone items for later comparison
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

    // Assert that the length is the same
    //assert_eq!(saved.len(), num_bodies);

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


#[actix_rt::test]
async fn retrieving_empty_table_no_returns_empty () {
    // Arrange
    let app = spawn_app().await;

    // Add Data to the database that we don't want to fetch
    let table_no = 1;
    let items = [
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
    let get_response = app.query_table(2).await;

    // Assert OK response
    assert!(get_response.status().is_success());

    // retrieved json
    let saved = get_response.json::<Vec<DatabaseResponse>>().await.unwrap();

    // Assert that the response is empty
    assert!{saved.is_empty()}

}
