mod csv;

/* 

fn main() {
    let _ = csv::test_file_reading();
    csv::hello_csv();
    gui::hello_gui();
}
*/

slint::include_modules!();
use slint::{Model, ModelRc, StandardListViewItem, TableColumn, VecModel, SharedString, SortModel};
use std::rc::Rc;



fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;



    let from_body: Vec<Vec<&str>> = vec![ // TODO: rename
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"],
    ];

    // THIS WORKS DON'T TOUCH
    let transition1: Vec<Vec<StandardListViewItem>> = from_body.into_iter().map(|row: Vec<&str>| {
        row.into_iter().map(|cell: &str| {
            StandardListViewItem::from(SharedString::from(cell.to_string()))
        }).collect()
    }).collect();

    let transition2: Vec<VecModel<StandardListViewItem>> = transition1.into_iter().map(|row: Vec<StandardListViewItem>| {
        VecModel::from(row)
    }).collect();

    let transition3: Vec<Rc<VecModel<StandardListViewItem>>> = transition2.into_iter().map(|row: VecModel<StandardListViewItem>| {
        Rc::from(row)
    }).collect();

    let transition4: Vec<ModelRc<StandardListViewItem>> = transition3.into_iter().map(|row: Rc<VecModel<StandardListViewItem>>| {
        ModelRc::from(row)
    }).collect();

    let transition5: VecModel<ModelRc<StandardListViewItem>> = VecModel::from(transition4);
    let transition6: Rc<VecModel<ModelRc<StandardListViewItem>>> = Rc::from(transition5);
    let transition7: ModelRc<ModelRc<StandardListViewItem>> = ModelRc::from(transition6);



    ui.set_table_data(transition7);


    let from_header: Vec<&str> = vec![ // TODO: rename
        "Header 1",
        "Header 2",
        "Header 3"
    ];
    
    // THIS WORKS DON'T TOUCH
    let header_transition1: Vec<TableColumn> = from_header.into_iter().map(|cell: &str| {
        let mut column: TableColumn = TableColumn::default();
        column.title = SharedString::from(cell);
        column
    }).collect();
    let header_transition2: VecModel<TableColumn> = VecModel::from(header_transition1);
    let header_transition3: Rc<VecModel<TableColumn>> = Rc::from(header_transition2);
    let header_transition4: ModelRc<TableColumn> = ModelRc::from(header_transition3);

    ui.set_header_data(header_transition4);
    


    let ui_handle = ui.as_weak(); // currently unused but will be used for interactive ui
    /* ui. on_request_increase_value (move || {
        let ui = ui_handle.unwrap();
        ui. set_counter(ui.get_counter () + 1);
    }); */

    ui.run()
}


