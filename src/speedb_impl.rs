// implement the single-node SpeeDB database of SurrealDB
// Docs: https://surrealdb.com/docs/surrealdb/embedding/rust

// TODO: create database
// TODO: make sure that it checks for an existing database and if there is not one, prompt the user to create one
// TODO: create database/table structure
// TODO: create query functions that work with rust functions (maybe located in main.rs)

use surrealdb::Surreal; // use the SurrealDB library
use surrealdb::sql::Thing;
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

// TODO: create database

// Create database connection with SpeeDB

fn main() {

    let db = match Surreal::new::<SpeeDB>("surrealdb_speedb").await {
        Ok(db) => db,
        Err(err) => {
            // print text to the error stream
            eprintln!("Failed to create Speedb database: {}", err);
            process::exit(1);
        }
    };
    
}





// TODO: determine where database is stored
// TODO: if existing database, use that, if not, create one















