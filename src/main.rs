use std::path::{Path, PathBuf};

use rfd::FileDialog;
use slint::SharedString;
use crate::qc_utils::{create_qc_props, PropsData};

mod tools_utils;
mod qc_utils;

slint::include_modules!();
fn main() {
    slint::init_translations!(std::env::current_exe().unwrap().parent().unwrap().join("lang"));

    let app_window: App = App::new().unwrap();
    //let app_window_weak:Weak<App> = app_window.as_weak();
    app_window.global::<TextLogic>().set_logs(slint::SharedString::from("TEST LOGS:\n"));

    set_callbacks(&app_window);


    app_window.run().unwrap();
}

fn set_callbacks(app: &App) {
    let app_weak: App = app.as_weak().clone().unwrap();

    let btn_logic: BtnLogic = app.global::<BtnLogic>();

    btn_logic.on_btn_compile(move || {
        let qc_inf: PropsData = PropsData::default();
        create_qc_props(qc_inf);
    });

    btn_logic.on_btn_gmod_bin_selection(move || {
        let gmod_bin_dir: Option<PathBuf> = FileDialog::new().pick_folder();
        match gmod_bin_dir {
            Some(gmod_buf) => {
                let gmod_path: &Path = gmod_buf.as_path();

                app_weak.global::<FilesPathsLogic>().set_gmod_bin_path(SharedString::from(gmod_path.to_str().unwrap_or("INVALID")));
                let mut verif_text: String = String::new();


                let gmad_buff: PathBuf = gmod_path.join("gmad.exe");//Borrower can suck my dick
                let gmad_path: &Path = gmad_buff.as_path();

                let studio_mdl_buff = gmod_path.join("studiomdl.exe"); //Borrower can suck my dick
                let studio_mdl_path = studio_mdl_buff.as_path();

                verif_text.push_str("GMAD: ");
                verif_text.push_str(if gmad_path.exists() { "FOUND\n" } else { "NOT FOUND\n" });

                verif_text.push_str("STUDIOMDL: ");
                verif_text.push_str(if studio_mdl_path.exists() { "FOUND\n" } else { "NOT FOUND\n" });

                app_weak.global::<TextLogic>().set_gmod_bin_verif_text(SharedString::from(verif_text))
            }
            None => app_weak.global::<FilesPathsLogic>().set_gmod_bin_path(SharedString::from("INVALID")),
        }
    });

    let app_weak: App = app.as_weak().clone().unwrap();

    btn_logic.on_btn_vtfedit_bin_selection(move || {
        let vtf_bin_dir: Option<PathBuf> = FileDialog::new().pick_folder();
        match vtf_bin_dir {
            Some(vtf_buf) => {
                let vtf_path: &Path = vtf_buf.as_path();

                app_weak.global::<FilesPathsLogic>().set_vtf_bin_path(SharedString::from(vtf_path.to_str().unwrap_or("INVALID")));

                let mut verif_text: SharedString = app_weak.global::<TextLogic>().get_gmod_bin_verif_text();


                let vtf_buff: PathBuf = vtf_path.join("VTFEdit.exe"); //Borrower can suck my dick
                let vtf_path: &Path = vtf_buff.as_path();

                verif_text.push_str("VTFEDIT: ");
                verif_text.push_str(if vtf_path.exists() { "FOUND\n" } else { "NOT FOUND\n" });

                app_weak.global::<TextLogic>().set_gmod_bin_verif_text(verif_text)
            }
            None => app_weak.global::<FilesPathsLogic>().set_vtf_bin_path(SharedString::from("INVALID")),
        }
    });
}

