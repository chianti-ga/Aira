use std::path::{Path, PathBuf};

use rfd::FileDialog;
use slint::SharedString;

use crate::qc_utils::{create_qc_props, PropsData};

mod tools_utils;
mod qc_utils;

slint::include_modules!();
fn main() {
    //log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    println!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));

    let app_window: App = App::new().unwrap();
    //let app_window_weak:Weak<App> = app_window.as_weak();
    app_window.global::<TextLogic>().set_logs(slint::SharedString::from("TEST LOGS:\n"));

    set_props_page_callbacks(&app_window);
    set_settings_page_callbacks(&app_window);

    app_window.run().unwrap();
}

fn set_props_page_callbacks(app: &App) {
    let app_weak: App = app.as_weak().clone().unwrap();
    let btn_logic: BtnLogic = app.global::<BtnLogic>();

    btn_logic.on_btn_models_selection(move || {
        let models_dir: Option<PathBuf> = FileDialog::new().pick_folder();
        match models_dir {
            None => app_weak.global::<FilesPathsLogic>().set_models_path(SharedString::from("")),
            Some(models_buff) => {
                let models_path = models_buff.as_path();
                app_weak.global::<FilesPathsLogic>().set_models_path(SharedString::from(models_path.to_str().unwrap_or("INVALID")));
            }
        }
    });

    let app_weak: App = app.as_weak().clone().unwrap();

    btn_logic.on_btn_materials_selection(move || {
        let materials_dir: Option<PathBuf> = FileDialog::new().pick_folder();
        match materials_dir {
            None => app_weak.global::<FilesPathsLogic>().set_materials_path(SharedString::from("")),
            Some(materials_buff) => {
                let materials_path = materials_buff.as_path();
                app_weak.global::<FilesPathsLogic>().set_materials_path(SharedString::from(materials_path.to_str().unwrap_or("INVALID")));
            }
        }
    });

    let app_weak: App = app.as_weak().clone().unwrap();

    btn_logic.on_btn_out_selection(move || {
        let out_dir: Option<PathBuf> = FileDialog::new().pick_folder();
        match out_dir {
            None => app_weak.global::<FilesPathsLogic>().set_compilation_out_path(SharedString::from("")),
            Some(out_buff) => {
                let out_path = out_buff.as_path();
                app_weak.global::<FilesPathsLogic>().set_compilation_out_path(SharedString::from(out_path.to_str().unwrap_or("INVALID")));
            }
        }
    });

    btn_logic.on_btn_compile(move || {
        let qc_inf: PropsData = PropsData::default();
        create_qc_props(qc_inf);
    });
}

fn set_settings_page_callbacks(app: &App) {
    let app_weak: App = app.as_weak().clone().unwrap();

    let btn_logic: BtnLogic = app.global::<BtnLogic>();

    btn_logic.on_btn_gmod_bin_selection(move || {
        let gmod_bin_dir: Option<PathBuf> = FileDialog::new().pick_folder();
        match gmod_bin_dir {
            Some(gmod_buf) => {
                let gmod_path: &Path = gmod_buf.as_path();

                app_weak.global::<FilesPathsLogic>().set_gmod_bin_path(SharedString::from(gmod_path.to_str().unwrap_or("INVALID")));
                let mut verif_text: String = String::new();

                let gmad_buff: PathBuf = gmod_path.join("gmad.exe"); //Borrower can suck my dick
                let gmad_path: &Path = gmad_buff.as_path();

                let studio_mdl_buff = gmod_path.join("studiomdl.exe"); //Borrower can suck my dick
                let studio_mdl_path = studio_mdl_buff.as_path();

                verif_text.push_str("GMAD: ");
                verif_text.push_str(if gmad_path.exists() { "FOUND\n" } else { "NOT FOUND\n" });

                verif_text.push_str("STUDIOMDL: ");
                verif_text.push_str(if studio_mdl_path.exists() { "FOUND\n" } else { "NOT FOUND\n" });

                app_weak.global::<TextLogic>().set_gmod_bin_verif_text(SharedString::from(verif_text))
            }
            None => app_weak.global::<FilesPathsLogic>().set_gmod_bin_path(SharedString::from("")),
        }
    });

    let app_weak: App = app.as_weak().clone().unwrap();

    btn_logic.on_btn_vtex_bin_selection(move || {
        let vtf_bin_dir: Option<PathBuf> = FileDialog::new().pick_folder();
        match vtf_bin_dir {
            None => app_weak.global::<FilesPathsLogic>().set_vtf_bin_path(SharedString::from("")),
            Some(vtf_buf) => {
                let vtf_path: &Path = vtf_buf.as_path();

                app_weak.global::<FilesPathsLogic>().set_vtf_bin_path(SharedString::from(vtf_path.to_str().unwrap_or("INVALID")));

                let mut verif_text: SharedString = app_weak.global::<TextLogic>().get_gmod_bin_verif_text();

                let vtf_buff: PathBuf = vtf_path.join("vtex.exe"); //Borrower can suck my dick
                let vtf_path: &Path = vtf_buff.as_path();

                verif_text.push_str("VTEX: ");
                verif_text.push_str(if vtf_path.exists() { "FOUND\n" } else { "NOT FOUND\n" });

                app_weak.global::<TextLogic>().set_gmod_bin_verif_text(verif_text)
            }
        }
    });
}
