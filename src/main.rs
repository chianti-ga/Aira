use std::path::PathBuf;
use rfd::FileDialog;
use slint::{SharedString, Weak};
use crate::qc_utils::{create_qc_props, PropsData};

mod qc_utils;

slint::include_modules!();
fn main() {
    slint::init_translations!(std::env::current_exe().unwrap().parent().unwrap().join("lang"));

    let app_window:App = App::new().unwrap();
    app_window.global::<TextLogic>().set_logs(slint::SharedString::from("TEST LOGS:\n"));

    set_btn_actions(&app_window);

    app_window.run().unwrap();
}
fn set_btn_actions(app:&'static App) {
    let btn_logic:BtnLogic = app.global::<BtnLogic>();
    let path_logic:FilesPathsLogic = app.global::<FilesPathsLogic>();

    btn_logic.on_btn_compile(move || {
        let qcinf = PropsData::default();
        create_qc_props(qcinf)
    });

    btn_logic.on_btn_gmod_bin_selection(move || {
        let dir:Option<PathBuf> = FileDialog::new()
            .pick_folder();
        match dir {
            Some(path) => {path_logic.set_gmod_bin_path(SharedString::from(path.as_path().to_str().get_or_insert("INVALID STR").to_string()))}
            None => {path_logic.set_gmod_bin_path(SharedString::from("INVALID"))}
        }
    });
}