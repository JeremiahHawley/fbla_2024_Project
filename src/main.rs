// TODO:
// open file and create 2d vector
// edit file and create files
// autosave feature
// q&a feature
// encrypt and decrypt 

// CURRENTLY EXCLUDED SINCE USING ui/appwindow.slint instead of gui.rs though may use gui.rs for some functions
// mod gui;

mod csv;

/* 

fn main() {
    csv::hello_csv();
    gui::hello_gui();
}
*/

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak(); // currently unused but will be used for interactive ui
    /* ui. on_request_increase_value (move || {
        let ui = ui_handle.unwrap();
        ui. set_counter(ui.get_counter () + 1);
    }); */

    ui.run()
}