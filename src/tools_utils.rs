use std::path::Path;
use std::process::Command;

use crate::App;

pub struct ToolsPaths<'a> {
    vtex: &'a Path,
    studio_mdl: &'a Path,
    gmad: &'a Path,
}

// https://developer.valvesoftware.com/wiki/Vtex_CLI_use
fn vtex_compile(materials_path: &Path, tools_paths: ToolsPaths, app: &App) {
    let vtex: &Path = tools_paths.vtex;

    let vtex_cmd: &mut Command = Command::new(vtex).args(["-nopause",
        "-dontusegamedir",
        "-mkdir",
        "-shader VertexLitGeneric",
        "-vmtparam \"$baseTexture\" \"base\"",
        "-vmtparam \"$bumpmap\" \"normal\"",
        materials_path.to_str().unwrap()]);
}

// https://developer.valvesoftware.com/wiki/StudioMDL_(Source_1)
pub fn studiomdl_compile(game_path: &Path, qc_file_path: &Path, tools_paths: ToolsPaths, app: &App) {
    let studio_mdl: &Path = tools_paths.studio_mdl;

    let studio_mdl_cmd: &mut Command = Command::new(studio_mdl).args(["-game", game_path.to_str().unwrap(), "-nop4", "-verbose", qc_file_path.to_str().unwrap()+"*.tga"]);
}