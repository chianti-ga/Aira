
use std::path::Path;
use std::process::Command;

use crate::App;

pub struct ToolsPaths<'a> {
    pub vtex:&'a Path,
    pub studio_mdl: &'a  Path,
    pub gmad:&'a  Path,
}

// https://developer.valvesoftware.com/wiki/Vtex_CLI_use
pub fn vtex_compile(materials_path: &Path, tools_paths: ToolsPaths, app: &App) {
    let vtex: &Path = tools_paths.vtex;
    let gmd: &Path = tools_paths.studio_mdl;
    let mut vtex_cmd: Command = Command::new(vtex);
    println!("{}", gmd.to_str().unwrap());
    vtex_cmd.args(["-dontusegamedir",
        "-nopause",
       "*.tga"]);
    vtex_cmd.current_dir(materials_path);
    let bb = vtex_cmd.spawn().unwrap();
    println!("{:?}", bb.wait_with_output().unwrap().stdout);
}

// // https://developer.valvesoftware.com/wiki/StudioMDL_(Source_1)
pub fn studiomdl_compile(game_path: &Path, qc_file_path: &Path, tools_paths:ToolsPaths, app: &App) {
    let studio_mdl: &Path = tools_paths.studio_mdl;

    let studio_mdl_cmd: &mut Command = Command::new(studio_mdl).args(["-game", game_path.to_str().unwrap(), "-nop4", "-verbose", qc_file_path.to_str().unwrap()]);
}