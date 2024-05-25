use std::fs::File;
use std::io::{self, Read, BufReader};

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
}

pub fn test_file_reading() -> io::Result<()> {
    let file = File::open(FILE_PATH)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);
    Ok(())
}

pub fn create_partner(row: &Vec<String>) -> Partner{
    let name = row[0].clone();
    let mut values: Vec<String> = Vec::new(); 
    for i in 0..row.clone().len()-1{
        if i > 0 {
            values.push(row[i].clone());
        }
    }
    return Partner{name: name, values: values};
}

// pub fn 

fn main(){
    let test_row_vec: Vec<String> = vec!["Jeremiah Hawley".to_string(),
                                        "955 N Burritt Buffalo Wy 82834".to_string(),
                                        "3076201429".to_string(), 
                                        "funny".to_string()];
    let test_partner: Partner = create_partner(&test_row_vec);
    println!("{}",test_partner.return_name());
}