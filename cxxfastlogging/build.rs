fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/fastlogging.cc")
        .std("c++14")
        .compile("cxxfastlogging");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/fastlogging.cc");
    println!("cargo:rerun-if-changed=h/fastlogging.h");
}
