//TODO:
// open file and create 2d vector
// edit file and create files
// 









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