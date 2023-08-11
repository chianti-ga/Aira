fn main() {
    //let compiler_config:CompilerConfiguration =CompilerConfiguration::default();
    slint_build::compile("ui/main.slint").unwrap();
    slint_build::print_rustc_flags().unwrap();
}
