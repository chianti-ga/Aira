#![allow(clippy::single_match)]

use std::path::Path;
use std::sync::mpsc::Receiver;
use std::thread;

use slint::{ComponentHandle, SharedString, Weak};

use crate::{App, BtnLogic, TOOLS_PATHS};
use crate::tools_utils::{vmt_generate, vtex_compile};

pub enum CompileLogicMessage {
    Texture {
        app_weak: Weak<App>,
        materials_path: SharedString,
        compilation_out_path: SharedString,
    },
    QCProps(Weak<App>),

    // You can add more variants here for different message types.
}

pub fn init_thread(rx: Receiver<CompileLogicMessage>) {
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
                        CompileLogicMessage::QCProps(app_weak) => {}
                    }
                }
                Err(_) => {}
            }
        }
    });
}
