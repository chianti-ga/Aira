extern crate core;

use std::path::{Path, PathBuf};
use std::sync::{mpsc, Mutex, MutexGuard};
use std::sync::mpsc::Sender;

use lazy_static::lazy_static;
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use rfd::FileDialog;
use slint::{SharedString, Weak};

use crate::compile_logic::{CompileLogicMessage, init_logic_thread};
use crate::config::{load_from_config, save_to_config};
use crate::tools_utils::ToolsPaths;

mod tools_utils;
mod qc_utils;
mod config;
mod compile_logic;
slint::include_modules!();

static ERROR_THREAD: &str = "ERROR IN UI THREAD.";

lazy_static! {
    static ref CONFIG_FILE:PathBuf = PathBuf::from("config.toml");
    static ref TOOLS_PATHS: Mutex<ToolsPaths> = Mutex::new(load_from_config());
}

fn main() {
    let log_stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {l} {m}{n}")))
        .build();

    let log_file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {l} {m}{n}")))
        .build("logs.txt")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(log_stdout)))
        .appender(Appender::builder().build("logs_file", Box::new(log_file_appender)))
        .build(Root::builder().appender("stdout").build(LevelFilter::max()))
        .unwrap();

    let handle_log = log4rs::init_config(config).unwrap();

    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));

    info!("Init app window");
    let app_window: App = App::new().unwrap();

    let (tx, rx) = mpsc::channel();
    init_logic_thread(rx);

    update_path_ui(&app_window);
    set_props_page_callbacks(&app_window, tx);
    set_settings_page_callbacks(&app_window);

    app_window.run().unwrap();
}

fn set_props_page_callbacks(app: &App, tx: Sender<CompileLogicMessage>) {
    let app_weak: Weak<App> = app.as_weak().clone();
    let btn_logic: BtnLogic = app.global::<BtnLogic>();

    btn_logic.on_btn_models_selection(move || {
        app_weak.upgrade_in_event_loop(move |app| {
            let models_dir: Option<PathBuf> = FileDialog::new().add_filter("*", &["smd"]).pick_file();
            match models_dir {
                None => app.global::<FilesPathsLogic>().set_models_path(SharedString::from("")),
                Some(models_buff) => {
                    let models_path: &Path = models_buff.as_path();
                    app.global::<FilesPathsLogic>().set_models_path(SharedString::from(models_path.to_str().unwrap_or("INVALID")));
                }
            }
        }).unwrap();
    });
    let app_weak: Weak<App> = app.as_weak().clone();

    btn_logic.on_btn_materials_selection(move || {
        app_weak.upgrade_in_event_loop(move |app| {
            let materials_dir: Option<PathBuf> = FileDialog::new().pick_folder();
            match materials_dir {
                None => app.global::<FilesPathsLogic>().set_materials_path(SharedString::from("")),
                Some(materials_buff) => {
                    let materials_path: &Path = materials_buff.as_path();
                    app.global::<FilesPathsLogic>().set_materials_path(SharedString::from(materials_path.to_str().unwrap_or("INVALID")));
                }
            }
        }).expect(ERROR_THREAD);
    });

    let app_weak: Weak<App> = app.as_weak().clone();

    btn_logic.on_btn_out_selection(move || {
        app_weak.upgrade_in_event_loop(move |app| {
            let out_dir: Option<PathBuf> = FileDialog::new().pick_folder();
            match out_dir {
                None => app.global::<FilesPathsLogic>().set_compilation_out_path(SharedString::from("")),
                Some(out_buff) => {
                    let out_path: &Path = out_buff.as_path();
                    app.global::<FilesPathsLogic>().set_compilation_out_path(SharedString::from(out_path.to_str().unwrap_or("INVALID")));
                }
            }
        }).expect(ERROR_THREAD);
    });

    let app_weak: Weak<App> = app.as_weak().clone();
    btn_logic.on_btn_compile(move || {
        tx.send(CompileLogicMessage::Texture {
            app_weak: app_weak.clone(),
            materials_path: app_weak.unwrap().global::<FilesPathsLogic>().get_materials_path().clone(),
            compilation_out_path: app_weak.unwrap().global::<FilesPathsLogic>().get_compilation_out_path().clone(),
        }).expect(ERROR_THREAD);
    });
}

fn set_settings_page_callbacks(app: &App) {
    let app_weak: App = app.as_weak().clone().unwrap();
    let btn_logic: BtnLogic = app.global::<BtnLogic>();

    btn_logic.on_btn_gmod_bin_selection(move || {
        let gmod_bin_dir: Option<PathBuf> = FileDialog::new().pick_folder();
        match gmod_bin_dir {
            None => app_weak.global::<FilesPathsLogic>().set_gmod_bin_path(SharedString::from("")),
            Some(gmod_buf) => {
                let gmod_path: &Path = gmod_buf.as_path();

                app_weak.global::<FilesPathsLogic>().set_gmod_bin_path(SharedString::from(gmod_path.to_str().unwrap_or("INVALID")));
                let mut verif_text: SharedString = app_weak.global::<TextLogic>().get_gmod_bin_verif_text();

                let gmad_buff: PathBuf = gmod_path.join("gmad.exe"); //Borrower can suck my dick
                let gmad_path: &Path = gmad_buff.as_path();

                let studio_mdl_buff: PathBuf = gmod_path.join("studiomdl.exe"); //Borrower can suck my dick
                let studio_mdl_path: &Path = studio_mdl_buff.as_path();

                let mut tools_path: MutexGuard<ToolsPaths> = TOOLS_PATHS.lock().unwrap();

                verif_text.push_str("GMAD: ");
                if gmad_path.exists() {
                    verif_text.push_str("FOUND\n");
                    tools_path.gmad = gmad_buff;
                } else {
                    verif_text.push_str("NOT FOUND\n");
                }

                verif_text.push_str("STUDIOMDL: ");
                if studio_mdl_path.exists() {
                    verif_text.push_str("FOUND\n");
                    tools_path.studio_mdl = studio_mdl_buff;
                } else {
                    verif_text.push_str("NOT FOUND\n");
                }
                app_weak.global::<TextLogic>().set_gmod_bin_verif_text(verif_text);

                save_to_config(&tools_path);
            }
        }
    });

    let app_weak: App = app.as_weak().clone().unwrap();

    btn_logic.on_btn_vtex_bin_selection(move || {
        let vtf_bin_dir: Option<PathBuf> = FileDialog::new().pick_folder();
        match vtf_bin_dir {
            None => app_weak.global::<FilesPathsLogic>().set_vtf_bin_path(SharedString::from("")),
            Some(vtf_buf) => {
                let vtf_path = vtf_buf.as_path();

                app_weak.global::<FilesPathsLogic>().set_vtf_bin_path(SharedString::from(vtf_path.to_str().unwrap_or("INVALID")));

                let mut verif_text: SharedString = app_weak.global::<TextLogic>().get_gmod_bin_verif_text();

                let vtf_buff: PathBuf = vtf_path.join("vtex2.exe");
                let mut tools_path: MutexGuard<ToolsPaths> = TOOLS_PATHS.lock().unwrap();

                verif_text.push_str("VTEX2: ");
                if vtf_buff.exists() {
                    verif_text.push_str("FOUND\n");
                    tools_path.vtex2 = vtf_buff;
                } else {
                    verif_text.push_str("NOT FOUND\n");
                }
                app_weak.global::<TextLogic>().set_gmod_bin_verif_text(verif_text);
                save_to_config(&tools_path);
            }
        }
    });
}

fn update_path_ui(app: &App) {
    let app_weak: App = app.as_weak().clone().unwrap();
    let tools_path: MutexGuard<ToolsPaths> = TOOLS_PATHS.lock().unwrap();

    if tools_path.vtex2.to_str().unwrap() == "" || tools_path.gmad.to_str().unwrap() == "" || tools_path.studio_mdl.to_str().unwrap() == "" {
        return;
    }

    app_weak.global::<FilesPathsLogic>().set_gmod_bin_path(SharedString::from(tools_path.gmad.parent().unwrap().to_str().unwrap()));
    app_weak.global::<FilesPathsLogic>().set_vtf_bin_path(SharedString::from(tools_path.vtex2.parent().unwrap().to_str().unwrap()));

    let mut verif_text: String = String::new();

    verif_text.push_str("GMAD: ");
    if tools_path.gmad.exists() {
        verif_text.push_str("FOUND\n");
    } else {
        verif_text.push_str("NOT FOUND\n");
    }

    verif_text.push_str("STUDIOMDL: ");
    if tools_path.studio_mdl.exists() {
        verif_text.push_str("FOUND\n");
    } else {
        verif_text.push_str("NOT FOUND\n");
    }

    verif_text.push_str("VTEX2: ");
    if tools_path.vtex2.exists() {
        verif_text.push_str("FOUND\n");
    } else {
        verif_text.push_str("NOT FOUND\n");
    }

    app_weak.global::<TextLogic>().set_gmod_bin_verif_text(SharedString::from(verif_text));
}
