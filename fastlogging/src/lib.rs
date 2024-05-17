mod def;
pub use def::*;
mod file;
pub use file::FileLogging;
mod net;
pub use net::{ LoggingServer, ClientLogging };
mod console;
pub use console::ConsoleLogging;
mod logging;
pub use logging::Logging;
mod logger;
pub use logger::Logger;

#[cfg(test)]
mod tests {
    use self::logging::Logging;

    use super::*;

    #[test]
    fn it_works() {
        let mut logging = Logging::new(None, None, None, None, None, None, None, None).unwrap();
        logging.info("Hello".to_string()).unwrap();
        logging.shutdown(Some(true)).unwrap();
    }
}
