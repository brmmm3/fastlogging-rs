use std::path::Path;
use std::process::Command;
use std::{env, fs};

fn main() {
    // Step 1: Generate C header with cbindgen
    println!("cargo:warning=Starting build.rs execution");
    let out_dir = match env::var("OUT_DIR") {
        Ok(dir) => dir,
        Err(e) => {
            println!("cargo:warning=Failed to get OUT_DIR: {}", e);
            return;
        }
    };
    let header_path = Path::new(&out_dir).join("jfastlogging_ffm.h");

    // Configure cbindgen
    println!(
        "cargo:warning=Generating C header: {}",
        header_path.display()
    );
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        cpp_compat: true,
        header: Some(String::from(
            "#ifndef JFASTLOGGING_FFM_H\n\
             #define JFASTLOGGING_FFM_H\n\
             \n\
             #include <stdint.h>\n\
             #include <stdbool.h>\n\
             \n\
             /* Opaque handles for Rust types from the fastlogging crate.\n\
              * cbindgen only analyses this crate's own source and cannot\n\
              * generate definitions for types that come from external\n\
              * dependencies.  All of these types are used exclusively as\n\
              * pointer-behind-opaque-handle; the layout is managed by Rust. */\n\
             typedef struct ExtConfig        ExtConfig;\n\
             typedef struct Logging          Logging;\n\
             typedef struct Logger           Logger;\n\
             typedef struct WriterConfigEnum WriterConfigEnum;\n\
             typedef struct WriterTypeEnum   WriterTypeEnum;\n\
             typedef struct WriterEnum       WriterEnum;\n\
             typedef struct LevelSyms        LevelSyms;\n\
             \n\
            ",
        )),

        trailer: Some(String::from("#endif // JFASTLOGGING_FFM_H\n")),
        ..Default::default()
    };

    // Generate headers from src/lib.rs
    if let Err(e) = cbindgen::Builder::new()
        .with_crate(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
        .with_config(config)
        .generate()
        .map(|bindings| bindings.write_to_file(&header_path))
    {
        println!("cargo:warning=Failed to generate C header: {}", e);
        return;
    }

    // Step 2: Run jextract to generate Java bindings
    let java_project_dir = Path::new("java_project/src/main/java/generated"); // Adjust to your Java project path
    let package_name = "com.example";
    let lib_name = "jfastlogging_ffm";

    let jextract_path = env::var("JEXTRACT_PATH")
        .or_else(|_| env::var("JAVA_HOME").map(|jh| format!("{}/bin/jextract", jh)))
        .unwrap_or_else(|_| "jextract".to_string());

    fs::create_dir_all(java_project_dir).expect("Failed to create Java output directory");

    println!(
        "cargo:warning=Running jextract: {} --output {} -t {} -l {} {}",
        jextract_path,
        java_project_dir.display(),
        package_name,
        lib_name,
        header_path.display()
    );

    let output = match Command::new(&jextract_path)
        .arg("--output")
        .arg(java_project_dir)
        .arg("-t")
        .arg(package_name)
        .arg("-l")
        .arg(lib_name)
        .arg(&header_path)
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            println!("cargo:warning=Failed to execute jextract: {}", e);
            return;
        }
    };

    if !output.status.success() {
        eprintln!(
            "cargo:error=jextract stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!(
            "cargo:error=jextract failed with exit code: {:?}",
            output.status.code()
        );
    } else {
        println!(
            "cargo:warning=jextract stdout: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    }

    // Note: Step 3 (copying the shared library to Java resources) cannot run here
    // because build.rs executes BEFORE the crate is compiled. The .so/.dll/.dylib
    // output is only produced after build.rs completes and the crate itself is
    // compiled. Use a post-build script (e.g. run_jextract.sh) or the Makefile
    // to copy the library to java_project/src/main/resources/ after cargo build.
    println!(
        "cargo:warning=Shared library will be placed at target/{}/libjfastlogging_ffm.so after a successful build.",
        env::var("PROFILE").unwrap_or_else(|_| "debug".to_string())
    );

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");
}
