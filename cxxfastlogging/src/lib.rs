#[cxx::bridge(namespace = "org::fastlogging")]
mod ffi {
    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("cxxfastlogging/h/fastlogging.h");

        type FastLogger;

        fn new_fastlogger() -> UniquePtr<FastLogger>;
        fn trace(self: &FastLogger, message: &str);
        fn debug(self: &FastLogger, message: &str);
        fn info(self: &FastLogger, message: &str);
        fn success(self: &FastLogger, message: &str);
        fn warning(self: &FastLogger, message: &str);
        fn error(self: &FastLogger, message: &str);
        fn critical(self: &FastLogger, message: &str);
        fn fatal(self: &FastLogger, message: &str);
        fn exception(self: &FastLogger, message: &str);
    }
}
