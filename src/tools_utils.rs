use std::collections::HashSet;
use std::fs::{copy, create_dir_all, File, remove_dir_all, remove_file};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{mpsc, Mutex, MutexGuard};
use std::thread;

use lazy_static::lazy_static;
use log::{debug, error, info};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::ThreadPool;
use rfd::{MessageDialog, MessageLevel};
use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Weak};
use walkdir::WalkDir;

use crate::{App, TextLogic};

lazy_static! {
    static ref BASE_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsPaths {
    pub vtex2: PathBuf,
    pub studio_mdl: PathBuf,
    pub gmad: PathBuf,
}

// https://github.com/StrataSource/vtex2/
pub fn vtex_compile(app_weak: Weak<App>, out_path: &Path, materials_path: &Path, tools_paths: MutexGuard<ToolsPaths>) {
    let vtex: &Path = tools_paths.vtex2.as_path();
    let materials_out = out_path.join("materials/");
    match remove_dir_all(materials_out.clone()) {
        Ok(_) => {
            app_weak.upgrade_in_event_loop(move |app| {
                let mut logs = app.global::<TextLogic>().get_logs();
                logs.push_str("Cleaning up output folder...\n");
                app.global::<TextLogic>().set_logs(logs);
            }).unwrap();
        }
        Err(err) => {
            error!("{:?}", err);
            MessageDialog::new()
                .set_title("Error")
                .set_level(MessageLevel::Error)
                .set_description(&format!("Cannot clear output folder before compilation :, {}", err))
                .show();
        }
    };
    let mut textures_files: Vec<String> = Vec::new();

    match create_dir_all(&materials_out) {
        Ok(_) => {
            app_weak.upgrade_in_event_loop(move |app| {
                let mut logs = app.global::<TextLogic>().get_logs();
                logs.push_str("Creating materials folder...\n");
                app.global::<TextLogic>().set_logs(logs);
            }).unwrap();
        }
        Err(err) => {
            error!("{:?}", err);
            MessageDialog::new()
                .set_title("Error")
                .set_level(MessageLevel::Error)
                .set_description(&format!("Cannot create output folder :, {}", err))
                .show();
        }
    }

    app_weak.upgrade_in_event_loop(move |app| {
        let mut logs = app.global::<TextLogic>().get_logs();
        logs.push_str("Copying textures files...\n");
        app.global::<TextLogic>().set_logs(logs);
    }).unwrap();

    WalkDir::new(materials_path).into_iter().filter_map(|e| e.ok()).for_each(|entry| {
        if entry.file_type().is_file() {
            let entry_path: String = entry.path().to_string_lossy().to_string();
            let final_filename: String = entry_path.replace(materials_path.to_str().unwrap().trim_start_matches(std::path::MAIN_SEPARATOR), "").replace(std::path::MAIN_SEPARATOR, "_").trim_start_matches('_').to_lowercase();

            let base_name: String = final_filename.replace(entry.file_name().to_str().unwrap(), "");
            BASE_NAMES.lock().unwrap().insert(base_name);

            let final_file = materials_out.join(final_filename.as_str());

            copy(entry.path(), final_file.clone()).unwrap();

            app_weak.upgrade_in_event_loop(move |app| {
                let mut logs = app.global::<TextLogic>().get_logs();
                logs.push_str(format!("{} => {}\n", entry.path().to_string_lossy(), final_file.to_string_lossy()).as_str());
                app.global::<TextLogic>().set_logs(logs);
            }).unwrap();

            textures_files.push(final_filename.clone());
        }
    });

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for x in rx {
            app_weak.upgrade_in_event_loop(move |app| {
        let logs = app.global::<TextLogic>().get_logs();
        app.global::<TextLogic>().set_logs(logs);
    }).unwrap();
        }
    });

    let pool: ThreadPool = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    pool.install(|| {
        textures_files.par_iter().for_each(|file| {
            let mut vtex_cmd: Command = Command::new(vtex);
            vtex_cmd.args(["convert", "-c", "9", "-f", "dxt5", file.as_str()])
                .current_dir(&materials_out)
                .stdout(Stdio::piped());

            let mut vtex_cmd_child = vtex_cmd.output();
            let stdout = vtex_cmd_child.unwrap().stdout;
            tx.send(stdout.clone()).unwrap();
            info!("{}", String::from_utf8(stdout).unwrap().as_str());
        });
    });

    /*app_weak.upgrade_in_event_loop(move |app| {
        let logs = app.global::<TextLogic>().get_logs();
        app.global::<TextLogic>().set_logs(logs);
    }).unwrap();*/

    WalkDir::new(materials_out).into_iter().filter_map(|e| e.ok()).filter(|file| file.file_name().to_str().unwrap().contains(".png")).for_each(|entry| {
        match remove_file(entry.path()) {
            Ok(_) => {}
            Err(err) => {
                error!("{:?}", err);
                MessageDialog::new()
                    .set_title("Error")
                    .set_level(MessageLevel::Error)
                    .set_description(&format!("Cannot delete file :, {}", err))
                    .show();
            }
        }
    });
}

pub fn vmt_generate(app_weak: Weak<App>, out_path: &Path) {
    let materials_out = out_path.join("materials/");

    let base_names = BASE_NAMES.lock().unwrap();
    base_names.par_iter().for_each(|base_name| {
        let mut base_color: String = base_name.clone();
        base_color.push_str("basecolor");

        let mut normal_map: String = base_name.clone();
        normal_map.push_str("normal");

        let mut env_map: String = base_name.clone();
        env_map.push_str("envmap");
        let mut vmt_filename = base_name.clone().trim_end_matches('_').to_string();
        vmt_filename.push_str(".vmt");

        let mut vmt_string: String = String::from("\"VertexlitGeneric\"\n");
        vmt_string.push_str("{\n");
        vmt_string.push_str(format!("\"$basetexture\" \"{}\"\n", base_color).as_str());
        vmt_string.push_str(format!("\"$normalmap\" \"{}\"\n", normal_map).as_str());
        vmt_string.push_str(format!("\"$envmap\" \"{}\"\n", env_map).as_str());
        vmt_string.push_str("\"$nolod\" \"1\"\n");
        vmt_string.push_str("\"$translucent\" \"1\"\n");
        vmt_string.push('}');

        debug!("Texture basename: {}", base_name);
        debug!("Texture vmt: {:?}", materials_out.join(&vmt_filename).as_path());

        let mut file: File = match File::create(materials_out.join(vmt_filename).as_path()) {
            Ok(file) => file,
            Err(err) => {
                error!("{:?}", err);
                MessageDialog::new()
                    .set_title("Error")
                    .set_level(MessageLevel::Error)
                    .set_description(&format!("Cannot create vmt file :, {}", err.to_string()))
                    .show();
                return;
            }
        };

        match file.write_all(vmt_string.as_bytes()) {
            Ok(_) => {}
            Err(err) => {
                error!("{:?}", err);
                MessageDialog::new()
                    .set_title("Error")
                    .set_level(MessageLevel::Error)
                    .set_description(&format!("Cannot write to vmt file :, {}", err.to_string()))
                    .show();
            }
        }
    });
}

// https://developer.valvesoftware.com/wiki/StudioMDL_(Source_1)
pub fn studiomdl_compile(game_path: &Path, qc_file_path: &Path, tools_paths: ToolsPaths, app: &App) {
    let studio_mdl: &Path = tools_paths.studio_mdl.as_path();

    let studio_mdl_cmd: &mut Command = Command::new(studio_mdl).args(["-game", game_path.to_str().unwrap(), "-nop4", "-verbose", qc_file_path.to_str().unwrap()]);
}