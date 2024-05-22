// implement the single-node SpeeDB database of SurrealDB
// Docs: https://surrealdb.com/docs/surrealdb/embedding/rust

// TODO: create database
// TODO: make sure that it checks for an existing database and if there is not one, prompt the user to create one
// TODO: create database/table structure
// TODO: create query functions that work with rust functions (maybe located in main.rs)

use surrealdb::Surreal; // use the SurrealDB library
use surrealdb::sql::Thing;
use surrealdb::sql::Value;
use surrealdb::engine::local::Db; // for a local database
use tokio; // for async operations
use serde::{Deserialize, Serialize}; // used for serializing and deserializing Rust data structures.

// TODO: create structs

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

/* this is the partner struct that will be the datatype for each individual "partner", 
it'll make data management easier and more straightforward (entire database can be one vector of partners)*/ 
#[derive(Debug, Serialize)] // By deriving Debug and Serialize, you can now print instances of Person using println!("{:?}", person) and serialize them using serde's serialization functions.
struct partner {
    // name will be used for the name of the actual partner whether that is the name of the person like "John" or a business like "DJ's"
    name: String,
    // values will be the list of scholarships/donations/etc that the partner has using a vector of enums that implement the scholarship/donation structs
    values: Vec<partner_value_type>,
}

enum partner_value_type { 
    Scholarship { // the `struct` keyword is not needed when creating an enum variant (design pattern is often refered to as "struct-like enum variants")
        name: String,
        amount: f32
        //TODO: add more types if needed
    },
    Donation {
        name: String,
        amount: f32
        //TODO: add more types if needed
    }
    //TODO: add more types
}




#[tokio::main]
async fn main() {
    // Create database connection with SpeeDB
    // create new database/connect to the database folder where it will put the database files
    let db = Surreal::new::<Speedb>("surreal_database").await?;
    
}

// TODO: function to import from CSV
// TODO: function to prompt user to create new record using the slint GUI




// =========
// Reference
// =========
/*
Create: db.create("person").content(...) creates a new record.
Read: db.select("person").await? reads the record.
Update: db.update("person").content(...) updates the record.
Delete: db.delete("person").await? deletes the record.

// Create a record
let created: Value = db
    .create("person")
    .content(serde_json::json!({
        "name": "John Doe",
        "age": 30,
     }))
    .await?;

*/







