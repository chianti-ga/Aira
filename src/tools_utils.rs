use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::MutexGuard;

use crate::App;

#[derive(Debug)]
pub struct ToolsPaths {
    pub vtex2: PathBuf,
    pub studio_mdl: PathBuf,
    pub gmad: PathBuf,
}

//https://github.com/StrataSource/vtex2/releases/tag/v0.1.1
pub fn vtex_compile(materials_path: &Path, tools_paths: MutexGuard<ToolsPaths>) -> String {
    let vtex: &Path = tools_paths.vtex2.as_path();
    let gmd: &Path = tools_paths.studio_mdl.as_path();

    let mut vtex_cmd: Command = Command::new(vtex);
    vtex_cmd.args(["convert", "--gamma-correct", "--srgb", "--trilinear",  "-f dxt5", "-r", materials_path.to_str().unwrap()]);

    vtex_cmd.current_dir(materials_path);
    println!("{:?}", vtex_cmd.get_args());
    let vtex_cmd_child: Child = vtex_cmd.spawn().unwrap();

    String::from_utf8(vtex_cmd_child.wait_with_output().unwrap().stdout).unwrap_or(String::from("err stdout"))
}

// https://developer.valvesoftware.com/wiki/StudioMDL_(Source_1)
pub fn studiomdl_compile(game_path: &Path, qc_file_path: &Path, tools_paths: ToolsPaths, app: &App) {
    let studio_mdl: &Path = tools_paths.studio_mdl.as_path();

    let studio_mdl_cmd: &mut Command = Command::new(studio_mdl).args(["-game", game_path.to_str().unwrap(), "-nop4", "-verbose", qc_file_path.to_str().unwrap()]);
}