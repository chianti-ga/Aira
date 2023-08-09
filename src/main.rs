slint::include_modules!();
fn main() {
    slint::init_translations!(std::env::current_exe().unwrap().parent().unwrap().join("lang"));

    let app_window: App = App::new().unwrap();
    app_window.set_logs(slint::SharedString::from("TEST LOGS:\n"));

    let app_window_weak = app_window.as_weak();
    app_window.global::<BtnLogic>().on_btn_compile(move || {
        println!("Clicked!")
    });

    app_window.run().unwrap();
}