pub fn hello_gui(){
    println!("Hello, world, from gui");
}


//skeleton code to go in main.rs

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {

    // data struct definitions
    // TODO: take in csv information from csv.rs

    // component definitions


    component Button inherits Rectangle {
        // TODO: implement button component/struct
    }





    export component MainWindow inherits Window {
        width: 400px; // logical pixels (before device specific scaling)
        height: 400px;



        // component member implementation (put into displayed window)
    }
}

/*
TouchArea { // when clicked (put this in the component)
    clicked => {
        // Delegate to the user of this element
        root.clicked(); // root refers to the outermost element in the component
    }
}

callback keyword is used to call a rust function outside of slint

*/



/* TODO:

Make good layout

Buttons:
    - implement the functions below

Functions:
- sort/filter table
    - has scholarships
    - alphabetically by name
    - reverse alphabetically by name
    - TODO: add more filters/sorts
- edit table
    - add
    - update
    - delete

Windows/Modes/Menus:
- help menu
- import/export csv
- settings menu
- home menu
- table view 

Table Display:
- sort options
- filter options


*/




