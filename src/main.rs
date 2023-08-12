use std::path::PathBuf;
use rfd::FileDialog;
use slint::{SharedString, Weak};
use crate::qc_utils::{create_qc_props, PropsData};

mod qc_utils;

slint::include_modules!();
fn main() {
    slint::init_translations!(std::env::current_exe().unwrap().parent().unwrap().join("lang"));

    let app_window:App = App::new().unwrap();
    //let app_window_weak:Weak<App> = app_window.as_weak();
    app_window.global::<TextLogic>().set_logs(slint::SharedString::from("TEST LOGS:\n"));

    set_btn_actions(&app_window);


    app_window.run().unwrap();
}

fn set_btn_actions(app:&App){
    let btn_logic: BtnLogic = app.global::<BtnLogic>();

    let app_weak = app.as_weak().unwrap();

    btn_logic.on_btn_compile(move || {
        let qc_inf = PropsData::default();
        create_qc_props(qc_inf);
    });

    btn_logic.on_btn_gmod_bin_selection(move || {
        let dir: Option<PathBuf> = FileDialog::new().pick_folder();
        app_weak.global::<FilesPathsLogic>().set_gmod_bin_path(SharedString::from(dir.unwrap().to_str().unwrap()));
    });
}