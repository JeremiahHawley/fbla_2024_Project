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


    // The main problem appears to be that the Database struct does not implement Copy, which it can't due to having a string field

    // MUST USE RC AND REFCELL

    let mut reference_database: Rc<RefCell<Database>> = Rc::new(RefCell::new(csv::load_from_csv("src/test.csv")));

    update_table_display_from_database(&ui, &reference_database.borrow()); // initial table display

    // Rc is used for multiple ownership so that it can be passed to the callbacks
    // RefCell is used so these other owners can mutate the database

    let working_database: Rc<RefCell<Database>> = Rc::new(RefCell::new(reference_database.borrow().clone())); // initial working database


    // DEFINE CALLBACKS

    // Initial table display
    update_table_display_from_database(&ui, &working_database.borrow());

    // Define callbacks

    // OK I don't entriely know how this works but it works
    let ui_handle = ui.as_weak();
    let ref_db = Rc::clone(&reference_database);
    let work_db = Rc::clone(&working_database);
    ui.on_hide_column(move |input: SharedString| {
        if let Some(ui) = ui_handle.upgrade() {
            let mut temp_database = work_db.borrow_mut();
            *temp_database = ref_db.borrow().clone().delete_column(&temp_database, input);
            update_table_display_from_database(&ui, &temp_database);
        }
    });

    let ui_handle = ui.as_weak();
    let ref_db = Rc::clone(&reference_database);
    let work_db = Rc::clone(&working_database);
    ui.on_show_column(move |input: SharedString| {
        if let Some(ui) = ui_handle.upgrade() {
            let mut temp_database = work_db.borrow_mut();
            *temp_database = ref_db.borrow().clone().show_column(&temp_database, input);
            update_table_display_from_database(&ui, &temp_database);
        }
    }); 









    


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

