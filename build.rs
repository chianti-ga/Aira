fn main() {
    //let compiler_config:CompilerConfiguration =CompilerConfiguration::default();
    slint_build::compile("ui/main.slint").unwrap();
    slint_build::print_rustc_flags().unwrap();
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("logo.ico");
        res.compile().unwrap();
    }
}
