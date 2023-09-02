#![allow(clippy::single_match)]

use std::{fs, thread};
use std::path::Path;
use std::sync::mpsc::Receiver;

use rfd::{MessageDialog, MessageLevel};
use slint::{ComponentHandle, SharedString, Weak};

use crate::{App, BtnLogic, ERROR_THREAD, TextLogic, TOOLS_PATHS};
use crate::qc::PropsData;
use crate::tools_utils::{BASE_NAMES, studiomdl_compile, vmt_generate, vtex_compile};

pub enum CompileLogicMessage {
    Texture {
        app_weak: Weak<App>,
        materials_path: SharedString,
        compilation_out_path: SharedString,
    },
    QCProps {
        app_weak: Weak<App>,
        props_data: Box<PropsData>,
        model_path: SharedString,
        compilation_out_path: SharedString,
    },
    StudioMdl {
        app_weak: Weak<App>,
        compilation_out_path: SharedString,
        model_path: SharedString,
    },

    // You can add more variants here for different message types.
}

pub fn init_logic_thread(rx: Receiver<CompileLogicMessage>) {
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(msg) => {
                    match msg {
                        CompileLogicMessage::Texture {
                            app_weak,
                            materials_path,
                            compilation_out_path,
                        } => {
                            app_weak.clone().upgrade_in_event_loop(move |app| {
                                app.global::<BtnLogic>().set_is_enabled(false);
                            }).unwrap();

                            vtex_compile(app_weak.clone(), Path::new(compilation_out_path.as_str()), Path::new(materials_path.as_str()), TOOLS_PATHS.lock().unwrap());
                            vmt_generate(app_weak.clone(), Path::new(compilation_out_path.as_str()));

                            app_weak.clone().upgrade_in_event_loop(move |app| app.global::<BtnLogic>().set_is_enabled(true)).unwrap();
                        }
                        CompileLogicMessage::QCProps {
                            app_weak,
                            mut props_data,
                            model_path,
                            compilation_out_path,
                        } => {
                            BASE_NAMES.lock().unwrap().iter().for_each(|name| {
                                props_data.texturegroup.push(name.clone());
                            });
                            props_data.collisionmodel.modelname = model_path.to_string();

                            let qc_file = Path::new(compilation_out_path.as_str()).join("qc.qc");
                            match fs::write(qc_file, props_data.to_string()) {
                                Ok(_) => {}
                                Err(err) => {
                                    MessageDialog::new()
                                        .set_title("Error")
                                        .set_level(MessageLevel::Error)
                                        .set_description(&format!("Cannot write to qc file :, {}", err))
                                        .show();
                                }
                            };
                            app_weak.upgrade_in_event_loop(move |app| {
                                app.global::<TextLogic>().set_qc_file_viewer(SharedString::from(props_data.to_string()));
                            }).expect(ERROR_THREAD);
                        }
                        CompileLogicMessage::StudioMdl {
                            app_weak,
                            compilation_out_path,
                            model_path,
                        } => {
                            studiomdl_compile(app_weak, Path::new(compilation_out_path.as_str()), model_path)
                        }
                    }
                }
                Err(_) => {}
            }
        }
    });
}
