use restaurant::domain::DatabaseResponse;

// Generate the expected result by adding table number to the item vector
// We don't know what random number will be generated for preparation time
// so it is left out
pub fn gen_expected_result(table_no: i32, items: Vec<(&str, i32)>) -> Vec<(i32, &str, i32)> {
    let mut res = Vec::new();

    for i in items {
        let item = (table_no, i.0, i.1);
        res.push(item);
    }

    res
}

// Compare if a data response and a vector of items is equal to each other
// independent of order
pub fn check_response(res: &Vec<DatabaseResponse>, expected_result: Vec<(i32, &str, i32)>) -> bool {
    // If the length is unequal then they are not the same
    if !(res.len() == expected_result.len()) {
        return false;
    };

    // clone of expected_result for comparison
    let compare_expected_result = expected_result.clone();

    // Check that each 3 tuple of table_no, item and quantity are contained the same number of times
    // in both the saved response and the expected result vector
    for i in expected_result {
        if res
            .iter()
            .filter(|s| (s.table_no == i.0 && s.item == i.1 && s.quantity == i.2))
            .count()
            != compare_expected_result
                .iter()
                .filter(|g| (g.0 == i.0 && g.1 == i.1 && g.2 == i.2))
                .count()
        {
            return false;
        }
    }

    // return true if all tests have passed
    true
}
