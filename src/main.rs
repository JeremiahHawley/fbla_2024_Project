// TODO:
// open file and create 2d vector
// edit file and create files
// autosave feature
// q&a feature
// encrypt and decrypt 
// 
mod csv;

/* 

fn main() {
    let _ = csv::test_file_reading();
    csv::hello_csv();
    gui::hello_gui();
}
*/

slint::include_modules!();
use slint::{Model, ModelRc, StandardListViewItem, TableColumn, VecModel, SharedString};
use std::rc::Rc;



fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;





    /* PREVIOUS IMPLEMENTATION 
    let from_body: Vec<Vec<&str>> = vec![ // TODO: rename
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"],
    ];
    let body_model : Rc<VecModel<Vec<&str>>> = Rc::new(VecModel::from(from_body.clone()));
    let body_model_rc: ModelRc<ModelRc<StandardListViewItem>> = ModelRc::new(body_model.clone().into());
    
    let from_header: Vec<&str> = vec![ // TODO: rename
        "Header 1",
        "Header 2",
        "Header 3"
    ];
    let header_model : Rc<VecModel<&str>> = Rc::new(VecModel::from(from_header.clone()));
    let header_model_rc = ModelRc::from(header_model.into());

    ui.set_table_data(body_model_rc);
    ui.set_header_data(header_model_rc);
    */


    let ui_handle = ui.as_weak(); // currently unused but will be used for interactive ui
    /* ui. on_request_increase_value (move || {
        let ui = ui_handle.unwrap();
        ui. set_counter(ui.get_counter () + 1);
    }); */

    ui.run()
}


