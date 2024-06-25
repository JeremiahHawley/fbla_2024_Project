
/*  TODO: implement these functions (may need to add header and body to function inputs)

add row/record to csv file
fn add_row(row: Vec<&str>) -> Vec<Vec<&str>> {}


edit row/record in csv file
fn edit_row(row: Vec<&str>) -> Vec<Vec<&str>> {// basically find the row and replace it?}// can change implementation


delete row/record from csv file
fn delete_row(row: Vec<&str>) -> Vec<Vec<&str>> {} // again, can choose how to implement, just make sure to communicate what the input is


hide column from view (may need to implement separate functions for header (Vec<&str>) and body (Vec<Vec<&str>>))
make sure to keep original to not lose data
fn hide_column(column: &str) -> Vec<&str>, Vec<Vec<&str>> {}

fn sort_csv_by_column(column: &str, order: &str) -> Vec<Vec<&str>> {
    match order {
        // implement different orders (asc, desc, etc) and call respective sort function for that column
    }
}
        let mut index: usize = 0; 
        for data_type in self.headers{
            if data_type == header.to_owned(){
                break;
            }
            index += 1;
        }
*/


use std::fs::File;
use std::ptr::null;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Read, BufReader, Write};

const FILE_PATH: &str = "src/test.csv";

#[derive(Clone, PartialEq)] // allows Partner to be cloned and use == and !=
pub struct Partner {
    values: Vec<String>,
}
impl Partner{
    pub fn return_values(&self) -> Vec<String>{self.values.clone()}
    pub fn return_index_values(&self,n: usize) -> String{
        if n < self.values.len(){
            return self.values[n].clone();
        }
        return "".to_string();
    }
}

#[derive(Clone, PartialEq)]
pub struct Database {
    pub partners: Vec<Partner>,
    pub headers: Vec<String>,
}
impl Database {
    pub fn add_column(mut self, header: &str){
        self.headers.push(header.to_owned());
        for i in 0..(self.partners.len()-1) {
            self.partners[i].values.push("N/A".to_string());
        }
    }
    pub fn show_column(mut self, database: &Database, header: slint::SharedString) -> Database {
        if database.headers.iter().any(|x| x.to_ascii_lowercase() == header.to_ascii_lowercase()) 
            || !self.headers.iter().any(|x| x.to_ascii_lowercase() == header.to_ascii_lowercase())
        {
            return database.clone(); // returns the existing database if that column already exists or if the column doesn't exist in the reference database
        }

        let header = header.to_string();
        let mut temp_database: Database = Database {
            partners: database.partners.clone(),
            headers: database.headers.clone(),
        };
        let mut index: usize = 0;
        for i in 0..self.headers.len()-1 {
            if self.headers[i].to_ascii_lowercase() == header.to_ascii_lowercase() {
                break;
            }
            index += 1;
        }

        temp_database.headers.insert(index, header.to_string());
      

        for i in 0..(temp_database.partners.len()) {
            temp_database.partners[i].values.insert(index, self.partners[i].values[index].clone());
        }
        
        return temp_database;
    }
    pub fn delete_column(mut self, database: &Database, header: slint::SharedString) -> Database {
        let header = header.to_string();
        let mut temp_database: Database = Database {
            partners: database.partners.clone(),
            headers: database.headers.clone(),
        };
        let mut index: usize = 0; 
        let mut is_header_found: bool = false;
        for data_type in &temp_database.headers{
            if data_type.to_ascii_lowercase() == header.to_ascii_lowercase(){ // BUG: this is never true for if the column is not found
                is_header_found = true;
                break;
            }
            index += 1;
        }
        if !is_header_found || temp_database.partners[0].values.len() == 1{ 
            // return the same database if the column is not found
            // return the same database if removing said column would result in an empty values vector for partners (partner 0 used as reference)
            return temp_database;
        }
        // modify temp_database
        temp_database.headers.remove(index); 
        for i in 0..temp_database.partners.len() { 
            temp_database.partners[i].values.remove(index); 
        }
        return temp_database;  
    }

    pub fn add_row(mut self){
        self.partners.push(
            Partner { 
                values: Vec::new() }
        );
    }
    pub fn add_row_from_vec(mut self, values: &Vec<String>){
        self.partners.push(
            Partner { 
                values: values.clone() 
            }
        )
    }
    pub fn delete_row(mut self,name: &String){
        let mut index: usize = 0;
        for partner in &self.partners{
            if &partner.values[0] == name{
                break;
            }
            index += 1;
        }
        self.partners.remove(index);
    }
    pub fn edit_row(mut self, name: &String, values: &Vec<String>){
        let mut index: usize = 0;
        for partner in &self.partners{
            if &partner.values[0] == name{
                break;
            }
            index += 1;
        }
        self.partners[index].values = values.clone();
    }
    pub fn edit_value(mut self, name: &String, header: &String,value: &String){
        let mut name_index: usize = 0;
        let mut header_index: usize = 0;
        for data_type in &self.headers{
            if data_type == header{
                break;
            }
            header_index += 1;
        }
        for partner in &self.partners{
            if &partner.values[0] == name{
                break;
            }
            name_index += 1;
        }
        self.partners[name_index].values[header_index] = value.clone();
    }
    pub fn save_to_csv(self) -> io::Result<()> {
        let mut string_to_write = String::new();
        let two_vec: Vec<Vec<String>> = db_to_2d_vec(self);
        for vector in two_vec {
            for string in vector {
                string_to_write += string.as_str();
            }
            string_to_write += "\n";
        }
        write_to_csv(FILE_PATH, &mut string_to_write)
    }
    
    // Returns a new database with the specified row hidden
    pub fn hide_row(self, name: &String) -> Database{
        let mut temp_database: Database = Database{
            partners: Vec::new(),
            headers: self.headers,
        };
        for partner in self.partners{
            if &partner.values[0] != name{
                temp_database.partners.push(partner);
            }
        }
        return temp_database;
    }

    /* SHOULDNT NEED THIS (use delete_column) 
    pub fn hide_column(self, header: &str) -> Database{
        let mut temp_database: Database = Database {
            partners: Vec::new(),
            headers: Vec::new(),
        };

        return temp_database;
    }
    */
    pub fn sort_by_column(){
        // TODO: implement FUNCTION ===================================================================
    }

    
    //
    // TODO
    // IMPORTANT NOTE: must do all columns at once due to how reference and working databases work (each call is a completely new filter from scratch)
    // uses working database as a reference database in a sense to maintain what columns are hidden
    //
    // NOTE:not sure it needs a reference database (self)
    // TODO: use self for reference database and then call the hides to update those
    pub fn search_column(self, database: &Database, target_vec: Vec<String>) -> Database {
        // TODO: implement creation of target vector in main.rs to pass into this from the callback definition
        println!("DEBUG: target_vec: {:?}", target_vec);
        // returns a WORKING DATABASE
        // TODO: add functionality for if the target_vec is shorter than database.headers.len()
        if target_vec.len() == 0 || self.partners.len() == 0 || target_vec.len() < self.headers.len(){
            // BEGIN DEBUG ===========================================================================
            if target_vec.len() == 0{
                println!("DEBUG: search column called with empty target_vec");
            }
            if self.partners.len() == 0{
                println!("DEBUG: search column called with empty database");
            }
            if target_vec.len() != self.headers.len(){
                println!("DEBUG: target_vec length does not match database headers length");
            }
            println!("DEBUG: search column called with incorrect data");
            // END DEBUG =============================================================================
            return self.clone();
        }


    
        


        // TODO: truncate the self width(reference database (doesn't affect the actual reference database in main.rs I DON'T THINK))
        // make a copy of self just to make sure we don't change the original in main.rs
        // use database since is working_database and has the data for which columns are hidden
        // compare self (reference database) with database (working_database) to see which columns are hidden and truncate the copy of self accordingly and then use that copy instead of self in the rest of the function

        let mut reference_database_copy: Database = new_database();
        reference_database_copy.headers = self.headers.clone();
        reference_database_copy.partners = self.partners.clone();

        reference_database_copy.headers.retain(|header| {database.headers.contains(header)}); // keep all headers that are in the working_database
        reference_database_copy.partners.retain(|partner| {database.partners.contains(partner)}); // keep all partners that are in the working_database



        let mut temp_database: Database = new_database(); 
        temp_database.headers = reference_database_copy.headers.clone();
        
        let mut blacklist_database: Database = new_database();
        blacklist_database.headers = reference_database_copy.headers.clone();




        // if the target_vec is shorter than the database, use the shorter length
        // uses variable shadowing instead of mutability
        let target_vec = if target_vec.len() == self.headers.len() {
            target_vec
        } else {
            target_vec[..self.headers.len()].to_vec()
        };
       

        // TODO: change loops to search through partners with the inner loop searching through partner.values
        'search_through_partners: for working_partner in &reference_database_copy.partners{    
            'parse_each_partner: for target_index in 0..reference_database_copy.headers.len(){
             // search the respective column
                // if that row is already in the database, skip
                // search by name (values[0])
             
                // skip if partner already in temp_database
                // SHOULD ONLY CONTINUE IF IN THE BLACKLIST DATABASE (just beacuse it passed one filter does not mean it will pass the others)
                /* 
                if temp_database.partners.iter().any(|partner| partner.values.len() > 0 && partner.values[target_index] == working_partner.values[target_index]) {
                    println!("DEBUG: partner already in temp_database");
                    continue 'search_through_partners; // jump to next partner in reference_database_copy.partners
                }*/

                // skip if partner already in blacklist_database (doesn't fit pattern (must fit all patterns))
                if blacklist_database.partners.iter().any(|partner| partner.values.len() > 0 && partner.values[target_index] == working_partner.values[target_index]) {
                    print!("DEBUG: partner already in blacklist_database | ");
                    continue 'search_through_partners; // jump to next partner in reference_database_copy.partners
                }
                

                //data_value.values[target_index] // string to search
                // target[target_index] // string to search for
                //test equality for all substrings of target of the same length as target
                let working_partner_clone = working_partner.clone();
                // DEBUG: adding the +1 does not seem to work but will require more testing
                //println!("DEBUG: working_partner_clone.values[target_index] {}", working_partner_clone.values[target_index]);
                //println!("DEBUG: target_vec[target_index] {}", target_vec[target_index]);
                if working_partner_clone.values.len() > 0 && working_partner_clone.values[target_index].contains(target_vec[target_index].as_str()){ // if target string found
                    // only add if not already in temp_database 
                    // also needs to pop from temp_database if in blacklist_database
                    if temp_database.partners.iter().any(|partner| partner.values.len() > 0 && partner.values[target_index] == working_partner_clone.values[target_index]) { // if in temp_database
                        print!("DEBUG: {} already in temp_database | ", working_partner_clone.values[target_index]);
                        continue 'parse_each_partner;
                    } else {
                        temp_database.partners.push(working_partner_clone.clone());
                        print!("DEBUG: {} added to temp_database | ", working_partner_clone.values[target_index]);
                    }
                } else {
                    if blacklist_database.partners.iter().any(|partner| partner.values.len() > 0 && partner.values[target_index] == working_partner_clone.values[target_index]) { // if in blacklist_database
                    // only add if not already in blacklist_database 
                    // also needs to pop from temp_database if in blacklist_database
                        print!("DEBUG: {} already in blacklist_database | ", working_partner_clone.values[target_index]);
                        continue 'parse_each_partner;
                    } else {
                        // remove from temp_database
                        if temp_database.partners.iter().any(|partner| partner.values.len() > 0 && partner.values[target_index] == working_partner_clone.values[target_index]) {
                            temp_database.partners.retain(|partner| partner.values.len() > 0 &&partner.values[target_index] != working_partner_clone.values[target_index]);
                        }
                        // add to blacklist_database
                        blacklist_database.partners.push(working_partner_clone.clone());
                        if working_partner_clone.values.len() > 0 {print!("DEBUG: {} added to blacklist_database | ", working_partner_clone.values[target_index])};
                    }
                }
                print!("DEBUG: target_index {} | ", target_index);
                println!("");
               

            }
        }
    
        return temp_database;
    }

    
}


    



pub fn load_from_csv(filepath: &str) -> Database {
    let mut new_database: Database = Database{
        partners: Vec::new(),
        headers: Vec::new(),
    };
    let file_string = file_to_string(filepath);
    let mut iterator: usize = 0;
    let char_vec = string_to_char_vec(&file_string);
    // find header row
    loop {
        if iterator >= char_vec.len() {
            break;
        } else if char_vec[iterator] == &'\n'.to_string() {
            iterator += 1;
            break;
        }
        iterator +=1;
    }
    let header_row = file_string[0..iterator].to_string();
    iterator = 0;
    // add headers to database
    for (i, character) in header_row.chars().enumerate() {
        if character == ',' || character == '\n'{
            new_database.headers.push(header_row[iterator..i].to_string());
            iterator = i+1;
        }
    }
    iterator = 0;
    let mut two_vec: Vec<Vec<String>> = Vec::new(); 
    let mut pass_header: bool = false;
    let mut temp_vec: Vec<String> = Vec::new();
    for (i, character) in file_string.chars().enumerate(){ //  ======= THE CSV FILE MUST END WITH A '\n' IN ORDER TO THE LAST LINE TO BE PUSHED TO THE VECTOR
        if character == '\n' || character == ','{ // it needs the "," part to work
            if !pass_header {
                if character == '\n' {
                    pass_header = true;
                }
                iterator = i+1;
                continue;
            }else{ // in the main body (past header)
                // add next value to temp_vec (representing a single row)
                if character == ','{
                    temp_vec.push(file_string[iterator..i].to_string());
                    iterator = i+1;
                }
                // add complete row to two_vec and reset temp_vec
                if character == '\n'{
                    temp_vec.push(file_string[iterator..i].to_string()); // add last value because csv does not end lines with ','
                    two_vec.push(temp_vec);
                    temp_vec = Vec::new();
                    iterator = i+1; 
                }
            }
        }
    }
    // add two_vec to partners vector
    for row in two_vec{
        if row.len() == 0{
            continue;
        }
        let partner = Partner {
            values: row.to_vec()
        };
        new_database.partners.push(partner);

    }
    return new_database;
}
// Creates a new EMPTY database
pub fn new_database() -> Database{
    return Database{partners: vec![Partner{values: Vec::new()}], headers: Vec::new()};
}

pub fn test_file_reading() -> io::Result<()> {
    let file = File::open(FILE_PATH)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);
    Ok(())
}

fn error_read(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn file_to_string(filepath: &str) -> String{
    let mut value = String::new();
    match error_read(filepath) {
        Ok(contents) => value = contents,
        Err(e) => value = String::from("error reading"),
    }
    return value;
}
pub fn db_to_2d_vec(db: Database, shown_headers: Vec<String>) -> Vec<Vec<String>> {
    let mut return_vector: Vec<Vec<String>> = Vec::new(); // entire database (headers and body)
    let mut header_row: Vec<String> = Vec::new();
    for header in db.headers{
        header_row.push(header);
    }
    return_vector.push(header_row);
    for partner in db.partners{
        let mut temp_vector: Vec<String> = Vec::new(); 
        for value in partner.values{
            temp_vector.push(value);
        }
        return_vector.push(temp_vector);
    }

    return_vector.retain(|element| {shown_headers.contains(&element[0])}); //element where element[0] is in shown_headers
    return return_vector;
}
fn write_to_csv(file_path: &str, text: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;    
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn string_to_char_vec(string: &str) -> Vec<&str>{
    // Collecting the character slices into a Vec<&str>
    let char_slices: Vec<&str> = string.chars().map(|c| {
        // Find the starting byte offset of the character
        let start = string.find(c).unwrap();
        // Convert the character to a string slice
        &string[start..start + c.len_utf8()]
    }).collect();

    return char_slices;
}