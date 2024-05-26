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

    let from_body: Vec<Vec<&str>> = vec![ // TODO: rename
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"],
    ];

    let body_model : Rc<VecModel<Vec<&str>>> = Rc::new(VecModel::from(from_body.clone()));

    // =================================================================================
    // =================================================================================
    // TODO: FIX THIS THING SO THAT IT ACTUALLY WORKS IVE SPENT HOURS ON THIS LITERALLY (line 42: let body_model_rc ...)
    // =================================================================================
    // =================================================================================

    let body_model_rc: ModelRc<ModelRc<StandardListViewItem>> = ModelRc::from(body_model.clone().into());
    
    /*let body_model_rc = ModelRc::<ModelRc<StandardListViewItem>>::from(
        Into::<ModelRc<ModelRc<StandardListViewItem>>>::into(body_model.clone()).into()
    );*/
    //ui.set_body_model(body_model_rc);

    let from_header: Vec<&str> = vec![ // TODO: rename
        "Header 1",
        "Header 2",
        "Header 3"
    ];

    let header_model : Rc<VecModel<&str>> = Rc::new(VecModel::from(from_header.clone()));
    let header_model_rc = ModelRc::from(header_model.into());
    //ui.set_header_model(header_model_rc);

    //let body_model_rc: ModelRc<ModelRc<StandardListViewItem>> = ModelRc::<ModelRc<StandardListViewItem>>::new(From::<dyn,ModelRc<ModelRc<StandardListViewItem>>>::into(from_body));

    //let header_data: ModelRc<TableColumn> = ModelRc::<TableColumn>::new(from_header.into());

    ui.set_table_data(body_model_rc);
    ui.set_header_data(header_model_rc);



    let ui_handle = ui.as_weak(); // currently unused but will be used for interactive ui
    /* ui. on_request_increase_value (move || {
        let ui = ui_handle.unwrap();
        ui. set_counter(ui.get_counter () + 1);
    }); */

    ui.run()
}


/* 
let ui = Example::new().unwrap();
// Create a VecModel and put it in an Rc.
let the_model : Rc<VecModel<SharedString>> =
        Rc::new(VecModel::from(vec!["Hello".into(), "World".into()]));
// Convert it to a ModelRc.
let the_model_rc = ModelRc::from(the_model.clone());
// Pass the model to the ui: The generated set_the_model setter from the
// the_model property takes a ModelRc.
ui.set_the_model(the_model_rc);

// We have kept a strong reference to the_model, to modify it in a callback.
ui.on_add_item(move || {
    // Use VecModel API: VecModel uses the Model notification mechanism to let Slint
    // know it needs to refresh the UI.
    the_model.push("SomeValue".into());
});

// Alternative: we can re-use a getter.
let ui_weak = ui.as_weak();
ui.on_add_item(move || {
    let ui = ui_weak.unwrap();
    let the_model_rc = ui.get_the_model();
    let the_model = the_model_rc.as_any().downcast_ref::<VecModel<SharedString>>()
        .expect("We know we set a VecModel earlier");
    the_model.push("An Item".into());
});

*/