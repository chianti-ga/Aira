use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStderr, ChildStdout, Command, Stdio};
use std::sync::MutexGuard;

use serde::{Deserialize, Serialize};

use crate::App;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsPaths {
    pub vtex2: PathBuf,
    pub studio_mdl: PathBuf,
    pub gmad: PathBuf,
}

//https://github.com/StrataSource/vtex2/releases/tag/v0.1.1
pub fn vtex_compile(materials_path: &Path, tools_paths: MutexGuard<ToolsPaths>) -> (BufReader<ChildStdout>, BufReader<ChildStderr>) {
    let vtex: &Path = tools_paths.vtex2.as_path();

    let mut vtex_cmd: Command = Command::new(vtex);
    vtex_cmd.args(["convert", "-c", "9", "-f", "dxt5", "-r", materials_path.to_str().unwrap()]);
    vtex_cmd.stdout(Stdio::piped());
    vtex_cmd.stderr(Stdio::piped());

    vtex_cmd.current_dir(materials_path);
    let mut vtex_cmd_child: Child = vtex_cmd.spawn().unwrap();

    let stdout = vtex_cmd_child.stdout.take().unwrap();
    let stderr = vtex_cmd_child.stderr.take().unwrap();

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);
    (stdout_reader, stderr_reader)
}

// https://developer.valvesoftware.com/wiki/StudioMDL_(Source_1)
pub fn studiomdl_compile(game_path: &Path, qc_file_path: &Path, tools_paths: ToolsPaths, app: &App) {
    let studio_mdl: &Path = tools_paths.studio_mdl.as_path();

    let studio_mdl_cmd: &mut Command = Command::new(studio_mdl).args(["-game", game_path.to_str().unwrap(), "-nop4", "-verbose", qc_file_path.to_str().unwrap()]);
}