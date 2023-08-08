slint::slint!(import { App } from "ui/main.slint";);
fn main() {
    let app = App::new().unwrap();
    app.run().unwrap();
}