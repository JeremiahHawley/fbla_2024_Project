//TODO:
// open file and create 2d vector
// edit file and create files
// 


/* this is the partner struct that will be the datatype for each individual "parntner", 
it'll make data management easier and more straightforward (entire database can be one vector of partners)*/ 
struct partner {
    //name will be used for the name of the actual partner wether that is the name of the person like "John" or a buisness like "DJ's"
    name: String,
    // all values are strings, this values vector will just store every value like donations, scholarsips, etc.
    values: Vec<String>,
    /*this vector wil store what the values of "values" actually is, so if values[0] \
    stores the name of a scholarship, then values_names[0] would be "scholarships" */ 
    values_names: Vec<String>
}


fn read_collumn(filename: &str, column_name: &str) -> Vec<String>{
    // return a vector of the string valies of the collumn where the top value of the column is column_name
    return vec!["test".to_string()]
}

fn read_row(filename: &str, row_name: &str) -> Vec<String>{
    //return the vector of the string valies of the row where the leftmost value of the column is row_name
    return vec!["test".to_string()]
}

fn create_file(filename: &str) {
    //TODO: create a csv file with the filename
}

fn edit_file(databasename: &str, row: usize, columnum: usize, new_value: &String) { 
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