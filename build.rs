// Compiles the slint file
// DO NOT EDIT THIS FILE
fn main() {
    let config =
        slint_build::CompilerConfiguration::new()
        .with_style("cupertino".into());
    slint_build::compile_with_config("ui/appwindow.slint", config).unwrap();
}

