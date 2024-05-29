mod csv;

/* 

fn main() {
    let _ = csv::test_file_reading();
    csv::hello_csv();
    gui::hello_gui();
}
*/

slint::include_modules!();
use slint::{ ModelRc, StandardListViewItem, TableColumn, VecModel, SharedString};
use std::{rc::Rc, string};



fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;



    


    // TODO: create database from csv
    /* 
    let database = csv::Database {
        partners: body_list, // TODO: set these to the data from the csv
        headers: header_list,  // TODO: set these to the data from the csv
    };
    */


    // DEBUG: it appears that the body of the csv is not being loaded into the database
    let mut database: csv::Database = csv::new_database();
    database = csv::load_from_csv("src/test.csv");

    let two_vec: Vec<Vec<String>> = csv::db_to_2d_vec(database);
    
    print!("two_vec length: {} \n", two_vec.len());//DEBUG purposes

    

    let header_list: Vec<String> = two_vec[0].clone();
    let mut body_list: Vec<Vec<String>> = Vec::new();
    for row in two_vec.iter() {
        if row == &two_vec[0] {
            continue;
        }
        body_list.push(row.to_vec());
    }
    
    //two_vec[1..].to_vec();


    for row in two_vec.iter() {
        for cell in row.iter() {
            println!("{}", cell);
        }
    }

    print!("header_list: {:?}", header_list);
    print!("body_list: {:?}", body_list);


    


    // Convert the data from the csv file into types that slint uses
    let transformed_header_list = transform_header_list(header_list);
    let transformed_body_list = transform_body_list(body_list);    

    // Set the table data
    ui.set_header_data(transformed_header_list); // Column headers
    ui.set_table_data(transformed_body_list); // All other cells

  
    


    let ui_handle = ui.as_weak(); // currently unused but will be used for interactive ui
    /* ui. on_request_increase_value (move || {
        let ui = ui_handle.unwrap();
        ui. set_counter(ui.get_counter () + 1);
    }); */

    ui.run()
}








fn get_help_text() -> SharedString {
    // create string from txt file
    let help_text = include_str!("help.txt");
    let help_text = help_text.to_string();
    let help_text = SharedString::from(help_text);
    return help_text;
}


fn transform_header_list(header_list: Vec<String>) -> ModelRc<TableColumn> {
    // THIS WORKS DON'T TOUCH
    let header_transition1: Vec<TableColumn> = header_list.into_iter().map(|cell: String| {
        let mut column: TableColumn = TableColumn::default();
        column.title = SharedString::from(cell);
        column
    }).collect();
    let header_transition2: VecModel<TableColumn> = VecModel::from(header_transition1);
    let header_transition3: Rc<VecModel<TableColumn>> = Rc::from(header_transition2);
    let header_transition_final: ModelRc<TableColumn> = ModelRc::from(header_transition3);
    return header_transition_final;
}

fn transform_body_list(body_list: Vec<Vec<String>>) -> ModelRc<ModelRc<StandardListViewItem>> {
    // THIS WORKS DON'T TOUCH
    let body_transition1: Vec<Vec<StandardListViewItem>> = body_list.into_iter().map(|row: Vec<String>| {
        row.into_iter().map(|cell: String| {
            StandardListViewItem::from(SharedString::from(cell))
        }).collect()
    }).collect();
    let body_transition2: Vec<VecModel<StandardListViewItem>> = body_transition1.into_iter().map(|row: Vec<StandardListViewItem>| {
        VecModel::from(row)
    }).collect();
    let body_transition3: Vec<Rc<VecModel<StandardListViewItem>>> = body_transition2.into_iter().map(|row: VecModel<StandardListViewItem>| {
        Rc::from(row)
    }).collect();
    let body_transition4: Vec<ModelRc<StandardListViewItem>> = body_transition3.into_iter().map(|row: Rc<VecModel<StandardListViewItem>>| {
        ModelRc::from(row)
    }).collect();
    let body_transition5: VecModel<ModelRc<StandardListViewItem>> = VecModel::from(body_transition4);
    let body_transition6: Rc<VecModel<ModelRc<StandardListViewItem>>> = Rc::from(body_transition5);
    let body_transition_final: ModelRc<ModelRc<StandardListViewItem>> = ModelRc::from(body_transition6);
    return body_transition_final;
}

