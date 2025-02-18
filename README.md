# Restaurant Api

This repository contains a deployable Rust implementation of a restaurant REST api. 

## Database Design 

PostgreSQL was chosen due to familiarity. To make the database as simple as possible all information is stored in a single table.

```sql
CREATE TABLE orders(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   table_no integer NOT NULL,
   item TEXT NOT NULL,
   quantity integer NOT NULL,
   preparation_time integer NOT NULL,
   placed_at timestamptz NOT NULL
);
```

Assumptions: 

- It is assumed that items are not unique. This was assumed as preparation time is generated when the items are added to the table. If in the future that item is to be added again then it would need a new preparation time that is distinct from the previous occurences of that item in the table.
- Since items are not unique a unique identifier was added for when a client wants to delete a specific instance of an item. 
- It is assumed that data is added to the database only through the API. This is because in the Rust backend validity checks are performed to ensure that the item text corresponds to a valid menu item. This functionality could also be implemented in the database side by adding an additional table containing valid menu items. 
- For preparation_time it is acceptable to represent number of minutes as an integer. 

## Backend Data Design 

On the backend side an order is represented by:

```Rust
pub struct NewOrder {
    pub table_no: Nat,
    pub item: Item,
    pub quantity: Nat,
}

```

`Nat` ensures that the `i32` integer is a natural number e.g. 0 or greater.

`Item` ensures that the `String` input corresponds to a valid menu item. 

The unique id, prepartion_time and timestamp are generated by the backend when a post request is made.

## API 

The API for the restaurant is based on REST and has the following endpoints:

### POST 

Add item to the table by giving a table number, item and quantity. Additional information such as preparation_time and id is generated by the backend. The backend also checks that the item is valid.

```
curl -i -X POST -H 'Content-Type: application/x-www-form-urlencoded' -d "table_no=1&item=hamburger&quantity=1" http://127.0.0.1:8000/order
```

### GET 

Get all items stored in the database.
``` 
curl -i -X GET http://127.0.0.1:8000/query_all
```

Get a specific item using unique id. 
``` 
curl -i -X GET http://127.0.0.1:8000/query_id/39ccee83-841e-43e8-ba13-0b6ae794c454

```

Get all items for a specific table number.
```
curl -i -X GET http://127.0.0.1:8000/query_table/2
```

Get all occurences of an item in a specific table number. 
```
curl -i -X GET http://127.0.0.1:8000/query_item/1/hamburger
```

Example Output: 

``` json
[
{"id":"406cbc58-4050-4132-809b-69a9e0a761e0","table_no":2,"item":"hamburger","quantity":1,"preparation_time":7},
{"id":"19873f23-5b29-40d1-9dd9-49523e464f63","table_no":2,"item":"fries","quantity":1,"preparation_time":13},
{"id":"b9808655-4e6e-47fa-874a-28e1d027fb04","table_no":2,"item":"cola","quantity":2,"preparation_time":9}
]
```

Note: all get requests return '[]' if the get request does not retrieve data saved in the table.

### DELETE 

Delete a specific item using a unique id.
``` 
curl -i -X DELETE http://127.0.0.1:8000/delete/39ccee83-841e-43e8-ba13-0b6ae794c454
```

Delete all occurences of an item in a given table.
```
curl -i -X DELETE http://127.0.0.1:8000/delete_item/1/fries
```

## Testing Strategy 

A client is implemented in the `test/client.rs` file using `reqwest`. 

All API endpoints have been tested using the client. Some individual modules such as "item.rs" have additional module tests. To test that the application can handle multiple clients, `tokio::spawn` is used to spawn multiple clients and send requests in parallel. 

Test names were designed to convey what the test does e.g. "successfully_retrieve_specific_item_using_id" tests that you can successfully retrieve an item through the API using a unique identifier. 

The repo has CI through GitHub Actions. The CI runs the test suite, checks formatting and runs a linter. 

## Local Installation 

To locally run the application you need have `Docker` and `Cargo` installed. 

Additionaly `sqlx-cli` needs to be installed. This can be done by: `cargo install sqlx-cli`

While the docker daemon is running run the `init_db.sh` script to set up the database e.g. in the root of the repository run:

`./scripts/init_db.sh`

Once the database has been set up:

The server can be run with `cargo run`

See the API section on how to interact with the server while it is running.

The test suite can be run with `cargo test`

Note: The test suite is likely to fail unless you increase `ulimit`. This can be done by setting `ulimit -n 10000`

## Deployment 

This repo can be deployed using Digital Ocean. Intructions on deployment using the `doctl` command line interface are:

```
doctl apps create --spec spec.yaml
DATABASE_URL=YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run/
```

Afer these steps are run it should be possible to interact with the app using the app link which can be received using `doctl apps list`.

An example API call to a deployed version is:

```
curl -i -X POST -H 'Content-Type: application/x-www-form-urlencoded' -d "table_no=1&item=hamburger&quantity=1" https://restaurant-djbuc.ondigitalocean.app/order
```

NOTE: This app is no longer deployed at this address.

## Limitations / Future Work 

- There is currently no authentication on the server side for clients. This means that anyone with a link can add data to the table through the API. It would be good to add authentication so that only valid clients can send requests.
- It is assumed that all data in the database is valid as it is checked by the backend to be correct when added. It would, however, be useful to have a valid item table to have a second source of security. 
- More time could be spent on improving the error handling throughout the project. 
- Several TODOs have been left in the repo comments which would be good to address given more time.  


