mod csv;

/* 

fn main() {
    let _ = csv::test_file_reading();
    csv::hello_csv();
    gui::hello_gui();
}
*/

slint::include_modules!();
use csv::Database;
use slint::{ ModelRc, StandardListViewItem, TableColumn, VecModel, SharedString};
use std::rc::Rc;
use std::cell::RefCell;



fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;


    // MUST USE RC AND REFCELL

    let mut reference_database: Rc<RefCell<Database>> = Rc::new(RefCell::new(csv::load_from_csv("src/test.csv")));

    update_table_display_from_database(&ui, &reference_database.borrow()); // initial table display

    // Rc is used for multiple ownership so that it can be passed to the callbacks
    // RefCell is used so these other owners can mutate the database

    let working_database: Rc<RefCell<Database>> = Rc::new(RefCell::new(reference_database.borrow().clone())); // initial working database


    // DEFINE CALLBACKS

    let ui_handle = ui.as_weak();
    ui.on_hide_column(|input: SharedString| {
        if let Some(ui) = ui_handle.upgrade() {
            let mut temp_database = working_database.borrow_mut();
            *temp_database = reference_database.borrow().delete_column(&temp_database.clone(), input);
            update_table_display_from_database(&ui, &working_database.borrow());
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_show_column(|input: SharedString| {
        if let Some(ui) = ui_handle.upgrade() {
            let mut temp_database = working_database.borrow_mut();
            *temp_database = reference_database.borrow().show_column(&temp_database.clone(), input);
            update_table_display_from_database(&ui, &working_database.borrow());
        }
    });







/* 
    let mut reference_database: csv::Database = csv::new_database(); // initial reference database. csv::new_database();
    reference_database = csv::load_from_csv("src/test.csv");

    update_table_display_from_database(&ui, &reference_database); // initial table display
    
    // Rc is used for multiple ownership so that it can be passed to the callbacks
    // RefCell is used so these other owners can mutate the database
    let mut working_database = reference_database.clone(); // initial working database

    // DEFINE CALLBACKS
    let ui_handle = ui.as_weak();
    //let working_database_clone: Rc<RefCell<csv::Database>> = Rc::clone(&working_database);
    ui.on_hide_column( |input: slint::SharedString| {
        let ui = ui_handle.upgrade().unwrap();
        let temp_database = &mut working_database;
        *temp_database = reference_database.clone().delete_column(&temp_database.clone(), input);
        update_table_display_from_database(&ui, &working_database.clone());
    });
    


    let ui_handle = ui.as_weak();
    //let working_database_clone = Rc::clone(&working_database);
    ui.on_show_column( |input: slint::SharedString| {
        let ui = ui_handle.upgrade().unwrap();
        let temp_database = &mut working_database;
        *temp_database = reference_database.clone().show_column(&temp_database.clone(), input);
        update_table_display_from_database(&ui, &working_database.clone());
    });
*/











    /* 
    let mut reference_database: Rc<RefCell<csv::Database>> = Rc::new(RefCell::new(csv::new_database())); // initial reference database. csv::new_database();
    reference_database = Rc::new(RefCell::new(csv::load_from_csv("src/test.csv")));

    update_table_display_from_database(&ui, &reference_database); // initial table display
    
    // Rc is used for multiple ownership so that it can be passed to the callbacks
    // RefCell is used so these other owners can mutate the database
    let working_database = reference_database.clone(); // initial working database

    // DEFINE CALLBACKS
    let ui_handle = ui.as_weak();
    //let working_database_clone: Rc<RefCell<csv::Database>> = Rc::clone(&working_database);
    ui.on_hide_column(move |input: slint::SharedString| {
        let ui = ui_handle.upgrade().unwrap();
        let mut db = working_database.borrow_mut();
        *db = Rc::new(RefCell::new(reference_database.borrow().clone().delete_column(&db.clone(), input)));
        update_table_display_from_database(&ui, &db);
    });
    
    let ui_handle = ui.as_weak();
    //let working_database_clone = Rc::clone(&working_database);
    ui.on_show_column(move |input: slint::SharedString| {
        let ui = ui_handle.upgrade().unwrap();
        let mut db = working_database.borrow_mut();
        *db = Rc::new(RefCell::new(reference_database.borrow().clone().show_column(&db.clone(), input)));
        update_table_display_from_database(&ui, &db);
    });
*/



    


    //let ui_handle = ui.as_weak(); // currently unused but will be used for interactive ui
    /* ui. on_request_increase_value (move || {
        let ui = ui_handle.unwrap();
        ui. set_counter(ui.get_counter () + 1);
    }); */

    ui.run()
}



fn update_table_display_from_database(ui: &AppWindow, database: &Database) {
    let mut header_list: Vec<String> = header_list_from_database(database.clone());
    let mut body_list: Vec<Vec<String>> = body_list_from_database(database.clone());

    // Convert the data from the csv file into types that slint uses
    let transformed_header_list = transform_header_list(header_list);
    let transformed_body_list = transform_body_list(body_list); 

    // Set the table data
    ui.set_header_data(transformed_header_list); // Column headers
    ui.set_table_data(transformed_body_list); // All other cells
}


fn header_list_from_database(database: csv::Database) -> Vec<String> {
    return csv::db_to_2d_vec(database)[0].clone();
}
fn body_list_from_database(database: csv::Database) -> Vec<Vec<String>> {
    return csv::db_to_2d_vec(database)[1..].to_vec();
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

