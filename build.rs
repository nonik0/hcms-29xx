fn main() {
    if cfg!(feature = "avr-progmem") {
        let rustc_version = std::process::Command::new("rustc")
            .arg("--version")
            .output()
            .expect("Failed to execute rustc")
            .stdout;

        let version_str = String::from_utf8_lossy(&rustc_version);
        if !version_str.contains("nightly") {
            panic!("The 'avr-progmem' feature requires a nightly toolchain!");
        }
    }
}