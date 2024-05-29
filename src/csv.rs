
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
use std::io::{self, Read, BufReader, Write};

const FILE_PATH: &str = "test.csv";
pub struct Partner {
    name: String,
    values: Vec<String>,
}
impl Partner{
    pub fn return_name(&self) -> String{self.name.clone()}
    pub fn return_values(&self) -> Vec<String>{self.values.clone()}
    pub fn return_index_values(&self,n: usize) -> String{
        if n < self.values.len(){
            return self.values[n].clone();
        }
        return "".to_string();
    }
    pub fn clone(&self) -> Partner{
        return Partner{name: self.name.clone(), values: self.values.clone()};
    }
}
pub struct Database {
    partners: Vec<Partner>,
    headers: Vec<String>,
}
impl Database {
    pub fn add_column(mut self, header: &str){
        self.headers.push(header.to_owned());
        for i in 0..(self.partners.len()-1) {
            self.partners[i].values.push("N/A".to_string());
        }
    }
    pub fn delete_column(mut self, header: &str){
        let mut index: usize = 0; 
        for data_type in &self.headers{
            if data_type == header{
                break;
            }
            index += 1;
        }
        self.headers.remove(index);
    }
    pub fn add_row(mut self, name: &String){
        self.partners.push(
            Partner { 
                name: name.clone(), 
                values: Vec::new() }
        );
    }
    pub fn add_row_from_vec(mut self, name: &String, values: &Vec<String>){
        self.partners.push(
            Partner { 
                name: name.clone(), 
                values: values.clone() 
            }
        )
    }
    pub fn delete_row(mut self,name: &String){
        let mut index: usize = 0;
        for partner in &self.partners{
            if &partner.name == name{
                break;
            }
            index += 1;
        }
        self.partners.remove(index);
    }
    pub fn edit_row(mut self, name: &String, values: &Vec<String>){
        let mut index: usize = 0;
        for partner in &self.partners{
            if &partner.name == name{
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
            if &partner.name == name{
                break;
            }
            name_index += 1;
        }
        self.partners[name_index].values[header_index] = value.clone() ;
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
    pub fn load_from_csv(mut self,filepath: &str){
        let mut new_database: Database = Database{
            partners: Vec::new(),
            headers: Vec::new(),

        };
        let file_string = file_to_string(filepath);
        let mut iterator: usize = 0;
        let char_vec = string_to_char_vec(&file_string);
        loop {
            if char_vec[iterator] == &'\n'.to_string() {
                break;
            }
            iterator +=1;
        }
        let header_row = file_string[0..iterator].to_string();
        iterator = 0;
        for (i, character) in header_row.chars().enumerate() {
            if character == ','{
                new_database.headers.push(header_row[iterator..i].to_string());
                iterator = i+1;
            }
        }
        iterator = 0;
        let mut two_vec: Vec<Vec<String>> = Vec::new(); 
        let mut pass_header: bool = false;
        let mut temp_vec: Vec<String> = Vec::new();
        for (i, character) in file_string.chars().enumerate(){
            if character == '\n' || character == ','{
                if !pass_header {
                    iterator = i+1;
                    pass_header = true;
                    continue;
                }else{
                    if character == ','{
                        temp_vec.push(file_string[iterator..(i-1)].to_string());
                    }
                    if character == '\n'{
                        two_vec.push(temp_vec);
                        temp_vec = Vec::new();
                    }
                }
            }
        }
    }
    pub fn hide_row(self, name: &String) -> Database{
        let mut temp_database:Database = Database{
            partners: Vec::new(),
            headers: self.headers,
        };
        for partner in self.partners{
            if &partner.name != name{
                temp_database.partners.push(partner);
            }
        }
        return temp_database;
    }

    pub fn sort_by_column(){

    }
}



pub fn new_database() -> Database{
    return Database{partners: vec![Partner{name: "".to_string(), values: Vec::new()}], headers: Vec::new()};
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

pub fn db_to_2d_vec(db: Database) -> Vec<Vec<String>> {
    let mut return_vector: Vec<Vec<String>> = Vec::new();
    let mut header_row: Vec<String> = Vec::new();
    for header in db.headers{
        header_row.push(header);
    }
    return_vector.push(header_row);
    for partner in db.partners{
        let mut temp_vector: Vec<String> = Vec::new(); 
        temp_vector.push(partner.name.clone());
        for value in partner.values{
            temp_vector.push(value);
        }
        return_vector.push(temp_vector);
    }
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