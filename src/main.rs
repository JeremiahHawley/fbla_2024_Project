//TODO:
// open file and create 2d vector
// edit file and create files
// 


/* this is the partner struct that will be the datatype for each individual "partner", 
it'll make data management easier and more straightforward (entire database can be one vector of partners)*/ 
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




fn read_column(filename: &str, column_name: &str) -> Vec<String>{
    // return a vector of the string values of the column where the top value of the column is column_name
    return vec!["test".to_string()]
}

fn read_row(filename: &str, row_name: &str) -> Vec<String>{
    // return the vector of the string values of the row where the leftmost value of the column is row_name
    return vec!["test".to_string()]
}

fn create_file(filename: &str) {
    //TODO: create a csv file with the filename
}

fn edit_file(databasename: &str, row: usize, column_num: usize, new_value: &String) { 
    //TODO: change the value of the database (file) in spot [row][column] to the value new_value
}

fn main() {
    let test: partner = partner {
        name: "test".to_string(),
        values: vec!["test_scholarship".to_string()],
        values_names: vec!["scholarships".to_string()]
    };
    println!("main ran!   {}",test.name);
}

// =============
// PROJECT NOTES/IDEAS
// =============
//
// I'm thinking that we should use a database like surrealdb instead of just csv files because since
// each donation or scholarship has its own data, that would require either a 3d csv of sorts, putting an 
// array within on "cell" if it was opened in excel. Instead I'm thinking that we can use surrealdb, query that
// for displaying the data, and then create a function that can be used to import a csv into the database, though
// I'm not sure how that would work. I just think that using csv files for the whole thing, perhaps needing
// to create a relational database ourselves (maybe) would get messy quickly.