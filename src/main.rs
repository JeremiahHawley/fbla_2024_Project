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
use std::rc::Rc;



fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;



    // This can be set the the Vec<&str> of headers from the csv file
    let header_list: Vec<&str> = vec![ 
        // test data
        "Header 1",
        "Header 2",
        "Header 3"
    ];

    // This can be set to the Vec<Vec<&str>> of rows from the csv file
    let body_list: Vec<Vec<&str>> = vec![ 
        // test data
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"],
    ];


    // TODO: create database from csv
    /* 
    let database = csv::Database {
        partners: body_list, // TODO: set these to the data from the csv
        headers: header_list,  // TODO: set these to the data from the csv
    };
    */




    


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


fn transform_header_list(header_list: Vec<&str>) -> ModelRc<TableColumn> {
    // THIS WORKS DON'T TOUCH
    let header_transition1: Vec<TableColumn> = header_list.into_iter().map(|cell: &str| {
        let mut column: TableColumn = TableColumn::default();
        column.title = SharedString::from(cell);
        column
    }).collect();
    let header_transition2: VecModel<TableColumn> = VecModel::from(header_transition1);
    let header_transition3: Rc<VecModel<TableColumn>> = Rc::from(header_transition2);
    let header_transition_final: ModelRc<TableColumn> = ModelRc::from(header_transition3);
    return header_transition_final;
}

fn transform_body_list(body_list: Vec<Vec<&str>>) -> ModelRc<ModelRc<StandardListViewItem>> {
    // THIS WORKS DON'T TOUCH
    let body_transition1: Vec<Vec<StandardListViewItem>> = body_list.into_iter().map(|row: Vec<&str>| {
        row.into_iter().map(|cell: &str| {
            StandardListViewItem::from(SharedString::from(cell.to_string()))
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

