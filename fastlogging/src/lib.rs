mod def;
pub use def::*;
mod file;
pub use file::{ FileWriter, FileWriterConfig };
mod net;
pub use net::{ LoggingServer, ServerConfig, ClientWriter, ClientWriterConfig };
mod console;
pub use console::{ ConsoleWriter, ConsoleWriterConfig };
mod logging;
pub use logging::Logging;
mod logger;
pub use logger::Logger;

use once_cell::sync::Lazy;
use rand::{ distributions::Alphanumeric, thread_rng, Rng };

static AUTH_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    thread_rng().sample_iter(&Alphanumeric).take(32).collect()
});

#[cfg(test)]
mod tests {
    use self::logging::Logging;

    use super::*;

    #[test]
    fn it_works() {
        let mut logging = Logging::new(None, None, None, None, None, None).unwrap();
        logging.info("Hello".to_string()).unwrap();
        logging.shutdown(Some(true)).unwrap();
    }
}
