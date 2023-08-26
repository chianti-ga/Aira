use std::error::Error;
use std::fs::{copy, create_dir_all};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::MutexGuard;
use rayon::iter::IntoParallelRefIterator;

use rfd::{MessageDialog, MessageLevel};
use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Weak};
use walkdir::WalkDir;
use rayon::iter::ParallelIterator;
use crate::{App};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsPaths {
    pub vtex2: PathBuf,
    pub studio_mdl: PathBuf,
    pub gmad: PathBuf,
}

//https://github.com/StrataSource/vtex2/releases/tag/v0.1.1
pub fn vtex_compile(out_path: &Path, materials_path: &Path, tools_paths: MutexGuard<ToolsPaths>) {
    let vtex: &Path = tools_paths.vtex2.as_path();
    let materials_out = out_path.join("materials/");
    let mut textures_files:Vec<String> = Vec::new();

    match create_dir_all(&materials_out) {
        Ok(_) => {}
        Err(err) => {
            MessageDialog::new()
                .set_title("Error")
                .set_level(MessageLevel::Error)
                .set_description(&format!("Cannot create output folder :, {}", err.description()))
                .show();
        }
    }
    WalkDir::new(materials_path).into_iter().filter_map(|e|e.ok()).for_each(|entry| {
        if entry.file_type().is_file() {
            let entry_path: String = entry.path().to_string_lossy().to_string();
            let final_filename: String = entry_path.replace(materials_path.to_str().unwrap().trim_start_matches(std::path::MAIN_SEPARATOR), "").replace(std::path::MAIN_SEPARATOR, "_").trim_start_matches('_').to_lowercase();

            copy(entry.path(),  materials_out.join(final_filename.as_str())).unwrap();
            textures_files.push(final_filename.clone());
        }
    });

    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    pool.install(||{
        textures_files.par_iter().for_each(|file| {
            let mut vtex_cmd: Command = Command::new(vtex);
            vtex_cmd.args(["convert", "-c", "9", "-f", "dxt5", file.as_str()]);
            vtex_cmd.current_dir(&materials_out);

            let vtex_cmd_child: Child = vtex_cmd.spawn().unwrap();
            let out = vtex_cmd_child.wait_with_output();
        });
    });

}

pub fn vmt_generate(out_path: &Path) {

}

// https://developer.valvesoftware.com/wiki/StudioMDL_(Source_1)
pub fn studiomdl_compile(game_path: &Path, qc_file_path: &Path, tools_paths: ToolsPaths, app: &App) {
    let studio_mdl: &Path = tools_paths.studio_mdl.as_path();

    let studio_mdl_cmd: &mut Command = Command::new(studio_mdl).args(["-game", game_path.to_str().unwrap(), "-nop4", "-verbose", qc_file_path.to_str().unwrap()]);
}