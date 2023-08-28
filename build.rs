fn main() {
    //let compiler_config:CompilerConfiguration =CompilerConfiguration::default();
    slint_build::compile("ui/main.slint").unwrap();
    slint_build::print_rustc_flags().unwrap();

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("ui/logo.ico");
        res.compile().unwrap();
    }
}
