fn main() {
    // When pyo3 is used with "extension-module" feature, it intentionally skips
    // linking against the Python C library (the symbols are resolved at runtime
    // when the .so is loaded by Python). For native test binaries ("cargo test"),
    // we need to link the Python library explicitly.
    //
    // We detect the correct flags via python3-config and pass them
    // as rustc-link-search / rustc-link-lib so Cargo handles them properly.

    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = std::process::Command::new("python3-config");
        cmd.args(["--ldflags"]);
        // --embed is needed for Python >=3.8 on some distributions
        let output = cmd.arg("--embed").output();
        let output = match output {
            Ok(o) if o.status.success() => o,
            _ => {
                // Fallback to --ldflags without --embed
                let mut cmd = std::process::Command::new("python3-config");
                cmd.args(["--ldflags"]);
                cmd.output()
                    .expect("python3-config not found. Install Python development headers.")
            }
        };

        let flags = String::from_utf8_lossy(&output.stdout);
        for flag in flags.split_whitespace() {
            if let Some(path) = flag.strip_prefix("-L") {
                println!("cargo:rustc-link-search={}", path);
            } else if let Some(lib) = flag.strip_prefix("-l") {
                println!("cargo:rustc-link-lib={}", lib);
            }
        }
    }

    // On Windows, pyo3 with extension-module feature handles linking automatically
    // so we don't need to do anything here
    #[cfg(target_os = "windows")]
    {}
}
