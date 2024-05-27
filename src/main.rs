mod csv;
use slint;


slint::slint!{
    import { Button , VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        in property <int> counter: 1;
        callback clicked <=> btn.clicked;
        VerticalBox {
            Text { text: "Inshallah"; }
            btn := Button { text: "Allah U Ahkbar";}
        }
    }

}
fn main() {
    let app: App = App::new().unwrap();
    let weak = app.as_weak();
    app.on_clicked(move || {
        let app = weak.upgrade().unwrap();
        app.set_counter(app.get_counter() + 1);
    }
    );
    println!("test");
}