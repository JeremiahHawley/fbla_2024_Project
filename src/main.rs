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

const CSV_FILEPATH: &str = "src/test.csv";

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;


    // Rc is used for multiple ownership so that it can be passed to the callbacks
    // RefCell is used so these other owners can mutate the database
    let mut reference_database: Rc<RefCell<Database>> = Rc::new(RefCell::new(csv::load_from_csv(CSV_FILEPATH)));
    let mut working_database: Rc<RefCell<Database>> = Rc::new(RefCell::new(reference_database.borrow().clone())); // initial working database

    let mut shown_headers: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(csv::load_from_csv(CSV_FILEPATH).headers.clone()));
    let reference_headers: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(shown_headers.borrow().clone()));


    update_table_display_from_database(&ui, &reference_database.borrow(), Rc::new(RefCell::new(Vec::new()))); // initial table display


    // DEFINE CALLBACKS

    let ui_handle = ui.as_weak();
    let shown_headers_copy = Rc::clone(&shown_headers);
    let reference_headers_copy = Rc::clone(&reference_headers);
    ui.on_update_shown_headers(move || {
        if let Some(ui) = ui_handle.upgrade() {
            let reference_headers_internal = reference_headers_copy.borrow();
            let mut shown_headers_internal = shown_headers_copy.borrow_mut();
            let mut hidden_headers_bool_vec: Vec<bool> = vec![
                // NOTE: if the hide buttons correspond to the wrong columns, switch the orders of the following get functions/elements to be in the same order as the column headers
                ui.get_is_name_hidden(),
                ui.get_is_address_hidden(),
                ui.get_is_phone_number_hidden(),
                ui.get_is_value_hidden(),
                ui.get_is_type_hidden(),
                ui.get_is_scholarship_hidden()
            ];
            // reference headers index combined with the index and value of hidden headers bool vec determines if its added
            hidden_headers_bool_vec.truncate(reference_headers_internal.len());
            shown_headers_internal.clear();
            for i in 0..hidden_headers_bool_vec.len() {
                if hidden_headers_bool_vec[i] {
                    shown_headers_internal.push(reference_headers_internal[i].clone());
                }
            }
        }
    });

 /* 
    let ui_handle = ui.as_weak();
    let ref_db = Rc::clone(&reference_database);
    let work_db = Rc::clone(&working_database);
    ui.on_hide_column(move |input: SharedString| {
        if let Some(ui) = ui_handle.upgrade() {
            let mut temp_database = work_db.borrow_mut();
            *temp_database = ref_db.borrow().clone().delete_column(&temp_database, input);
            update_table_display_from_database(&ui, &temp_database, shown_headers);
        }
    });

    let ui_handle = ui.as_weak();
    let ref_db = Rc::clone(&reference_database);
    let work_db = Rc::clone(&working_database);
    ui.on_show_column(move |input: SharedString| {
        if let Some(ui) = ui_handle.upgrade() {
            let mut temp_database = work_db.borrow_mut();
            *temp_database = ref_db.borrow().clone().show_column(&temp_database, input);
            update_table_display_from_database(&ui, &temp_database, shown_headers);
        }
    }); 
*/

    let ui_handle = ui.as_weak();
    let ref_db = Rc::clone(&reference_database);
    let work_db = Rc::clone(&working_database);
    let shown_headers_copy = Rc::clone(&shown_headers);
    ui.on_update_table_display(move || {
        if let Some(ui) = ui_handle.upgrade() {
            let shown_headers_internal = shown_headers_copy.clone();
            let mut temp_database = work_db.borrow_mut();
            update_table_display_from_database(&ui, &temp_database, shown_headers_internal);
        }
    });

    let ui_handle = ui.as_weak();
    let ref_db = Rc::clone(&reference_database);
    let work_db = Rc::clone(&working_database);
    let shown_headers_copy = Rc::clone(&shown_headers);
    ui.on_update_search(move || {
        if let Some(ui) = ui_handle.upgrade() {
            let shown_headers_internal = shown_headers_copy.clone();
            let search_vector: Vec<String> = vec![ // collect the data from the LineEdits
                ui.get_inbox_name_var().to_string(),
                ui.get_inbox_value_var().to_string(),
                ui.get_inbox_type_var().to_string(),
                ui.get_inbox_phone_var().to_string(),
                ui.get_inbox_address_var().to_string(),
                ui.get_inbox_scholarship_var().to_string(),
            ];
            let mut temp_database = work_db.borrow_mut();
            *temp_database = ref_db.borrow().clone().search_column(&temp_database, search_vector);
            update_table_display_from_database(&ui, &temp_database, shown_headers_internal);
        }
    }); 






    /*
    when edit, call function that gets the values of all of the LineEdit inputs


    
    
    
     */





    /* 
    // Sort ascending
    let ui_handle = ui.as_weak();
    let ref_db = Rc::clone(&reference_database);
    let work_db = Rc::clone(&working_database);
    ui.on_sort_ascending(move |input: i32| { // TODO: may need to convert between slint int and rust i32
        if let Some(ui) = ui_handle.upgrade() {
            let mut temp_database = work_db.borrow_mut();
            // TODO: check correct function
            *temp_database = ref_db.borrow().clone().sort_ascending_by_column(&temp_database, input); // TODO: add the correct column and check whether input is i32
            update_table_display_from_database(&ui, &temp_database);
        }
    }); 

    // Sort decending
    let ui_handle = ui.as_weak();
    let ref_db = Rc::clone(&reference_database);
    let work_db = Rc::clone(&working_database);
    ui.on_sort_decending(move |input: i32| {// TODO: may need to convert between slint int and rust i32
        if let Some(ui) = ui_handle.upgrade() {
            let mut temp_database = work_db.borrow_mut();
            // TODO: check correct function
            *temp_database = ref_db.borrow().clone().sort_decending_by_column(&temp_database, input); // TODO: add the correct column and check whether input is i32
            update_table_display_from_database(&ui, &temp_database);
        }
    }); 
*/





    /* 
    let ui_handle = ui.as_weak();
    let ref_db = Rc::clone(&reference_database);
    let work_db = Rc::clone(&working_database);
    ui.on_hide_name(move || { // TODO: I THINK WE NEED TO ADD NAME INTO THE VECTOR BECAUSE THE CHANGE NEEDS TO BE COMMUNICATED TO WORKING_DATABASE
        if let Some(ui) = ui_handle.upgrade() {
            let mut temp_database = work_db.borrow_mut();
            *temp_database = ref_db.borrow().clone().show_column(&temp_database, input);
            update_table_display_from_database(&ui, &temp_database);
        }
    }); 
    */





    //let ui_handle = ui.as_weak(); // currently unused but will be used for interactive ui
    /* ui. on_request_increase_value (move || {
        let ui = ui_handle.unwrap();
        ui. set_counter(ui.get_counter () + 1);
    }); */

    ui.run()
}







fn update_table_display_from_database(ui: &AppWindow, database: &Database, shown_headers: Rc<RefCell<Vec<String>>>) {
    let mut header_list: Vec<String> = header_list_from_database(database.clone(), shown_headers.clone());
    let mut body_list: Vec<Vec<String>> = body_list_from_database(database.clone(), shown_headers.clone());

    // Convert the data from the csv file into types that slint uses
    let transformed_header_list = transform_header_list(header_list);
    let transformed_body_list = transform_body_list(body_list); 

    // Set the table data
    ui.set_header_data(transformed_header_list); // Column headers
    ui.set_table_data(transformed_body_list); // All other cells
}


fn header_list_from_database(database: csv::Database, shown_headers: Rc<RefCell<Vec<String>>>) -> Vec<String> {
    return csv::db_to_2d_vec(database, shown_headers.borrow().clone())[0].clone();
}
fn body_list_from_database(database: csv::Database, shown_headers: Rc<RefCell<Vec<String>>>) -> Vec<Vec<String>> {
    return csv::db_to_2d_vec(database, shown_headers.borrow().clone())[1..].to_vec();
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

