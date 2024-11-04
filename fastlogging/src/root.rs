use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::path::PathBuf;
use std::process;
use std::sync::Mutex;
use std::{env, fs};

use once_cell::sync::Lazy;

use crate::config::{default_config_file, ConfigFile, FileMerge};
use crate::console::ConsoleWriterConfig;
use crate::net::{ClientWriterConfig, EncryptionMethod, ServerConfig, AUTH_KEY};
use crate::{getppid, Logging, LoggingError, WriterConfigEnum, NOTSET};

pub static PARENT_LOGGER_ADDRESS: Lazy<Mutex<Option<(u32, ClientWriterConfig)>>> =
    Lazy::new(|| Mutex::new(None));

pub static ROOT_LOGGER: Lazy<Mutex<Logging>> = Lazy::new(|| {
    fn create_default_logger(config_file: Option<PathBuf>) -> Logging {
        let mut logging = Logging::new(NOTSET, "root", vec![], None, config_file).unwrap();
        if let Err(err) =
            logging.set_root_writer_config(&WriterConfigEnum::Server(ServerConfig::new(
                NOTSET,
                "127.0.0.1",
                EncryptionMethod::AuthKey(AUTH_KEY.to_vec()),
            )))
        {
            eprintln!("Failed to create Root ServerLogger: {err:?}");
        }
        logging.drop = false;
        logging
    }

    fn get_port_file(pid: u32) -> PathBuf {
        let mut temp_dir = env::temp_dir();
        temp_dir.push(format!("fastlogging_rs_server_port.{pid}"));
        temp_dir
    }

    fn get_parent_server_address() -> Result<Option<(String, EncryptionMethod)>, LoggingError> {
        let port_file = get_port_file(getppid());
        if port_file.exists() {
            // Parent process exists. Check if logging server is reachable.
            let mut buffer = Vec::new();
            if fs::File::open(port_file)?.read_to_end(&mut buffer)? >= 4 {
                let port = u16::from_le_bytes(buffer[..2].try_into().unwrap());
                let address = format!("127.0.0.1:{port}");
                let encryption = match buffer[2] {
                    0 => EncryptionMethod::NONE,
                    1 => EncryptionMethod::AuthKey(buffer[3..].to_vec()),
                    2 => EncryptionMethod::AES(buffer[3..].to_vec()),
                    _ => {
                        return Err(LoggingError::InvalidValue(format!(
                            "Invalid encryption type {}",
                            buffer[2]
                        )))
                    }
                };
                if let Ok(mut stream) = TcpStream::connect(&address) {
                    let buffer = vec![0xfeu8, 0xffu8, 0xffu8, 0xffu8];
                    stream.write_all(&buffer)?;
                    stream.flush()?;
                    stream.shutdown(Shutdown::Both)?;
                    return Ok(Some((address, encryption)));
                }
            }
        }
        Ok(None)
    }

    fn setup_logging() -> Result<Logging, LoggingError> {
        // Check if parent process with fastlogging instance exists.
        let mut logging = create_default_logger(None);
        if let Ok(server) = logging.get_server_config(0) {
            let port_file = get_port_file(process::id());
            // Server config above is just a copy. So we need to access the original directly.
            logging
                .instance
                .lock()
                .unwrap()
                .get_server_config(0)
                .unwrap()
                .port_file = Some(port_file.clone());
            let mut file = fs::File::create(port_file)?;
            file.write_all(&u16::to_le_bytes(server.port))?;
            file.write_all(&logging.get_server_auth_key().to_bytes())?;
        }
        if let Some((server_address, encryption)) = get_parent_server_address()? {
            // Connect to parent server port
            let mut client = ClientWriterConfig::new(NOTSET, server_address, encryption);
            client.debug = logging.instance.lock().unwrap().debug;
            *PARENT_LOGGER_ADDRESS.lock().unwrap() = Some((getppid(), client.clone()));
            logging.add_writer_config(&WriterConfigEnum::Client(client))?;
        } else {
            // If default config file exists, then use this configuration. Else create default console logger.
            let default_file_config = default_config_file();
            if default_file_config.1.is_empty() {
                logging.add_writer_config(&WriterConfigEnum::Console(ConsoleWriterConfig::new(
                    NOTSET, false,
                )))?;
            } else {
                let mut config_file = ConfigFile::new();
                config_file.load(&default_file_config.0)?;
                let mut instance = logging.instance.lock().unwrap();
                config_file.merge(&mut instance, FileMerge::MergeReplace)?;
            }
        }
        Ok(logging)
    }

    let logging = match setup_logging() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to setup default logger: {e}");
            create_default_logger(None)
        }
    };
    Mutex::new(logging)
});
