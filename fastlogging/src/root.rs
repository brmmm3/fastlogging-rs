use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process;
use std::sync::Mutex;
use std::{env, fs};

use once_cell::sync::Lazy;

use crate::config::default_config_file;
use crate::console::ConsoleWriterConfig;
use crate::net::{ClientWriterConfig, EncryptionMethod, ServerConfig, AUTH_KEY};
use crate::{getppid, Logging, WriterConfigEnum, NOTSET};

pub static PARENT_LOGGER_ADDRESS: Lazy<Mutex<Option<(u32, ClientWriterConfig)>>> =
    Lazy::new(|| Mutex::new(None));

pub static ROOT_LOGGER: Lazy<Mutex<Logging>> = Lazy::new(|| {
    fn create_default_logger(config_file: Option<PathBuf>) -> Logging {
        //println!("create_default_logger with config_file={config_file:?}");
        let server = ServerConfig::new(
            NOTSET,
            "127.0.0.1",
            EncryptionMethod::AuthKey(AUTH_KEY.to_vec()),
        );
        let mut logging = Logging::new(
            None,
            None,
            None,
            None,
            None,
            Some(server),
            None,
            None,
            config_file,
        )
        .unwrap();
        logging.drop = false;
        logging
    }

    fn get_port_file(pid: u32) -> PathBuf {
        //println!("get_port_file for pid {pid}");
        let mut temp_dir = env::temp_dir();
        temp_dir.push(format!("fastlogging_rs_server_port.{pid}"));
        temp_dir
    }

    fn get_parent_server_address() -> Result<Option<(String, EncryptionMethod)>, Error> {
        /*println!("get_parent_server_address");
        println!(
            "## {} {}",
            process::id(),
            std::os::unix::process::parent_id()
        );*/
        let port_file = get_port_file(getppid());
        //println!("CHECK port_file={port_file:?} {}", port_file.exists());
        if port_file.exists() {
            // Parent process exists. Check if logging server is reachable.
            let mut buffer = Vec::new();
            if fs::File::open(port_file)?.read_to_end(&mut buffer)? >= 4 {
                let port = u16::from_le_bytes(buffer[..2].try_into().unwrap());
                let address = format!("127.0.0.1:{port}");
                //println!("Connecting to address={address}");
                let encryption = match buffer[2] {
                    0 => EncryptionMethod::NONE,
                    1 => EncryptionMethod::AuthKey(buffer[3..].to_vec()),
                    2 => EncryptionMethod::AES(buffer[3..].to_vec()),
                    _ => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Invalid encryption type {}", buffer[2]),
                        ))
                    }
                };
                if TcpStream::connect(&address).is_ok() {
                    //println!("CONNECTED to {address}");
                    return Ok(Some((address, encryption)));
                }
            }
        }
        Ok(None)
    }

    fn setup_logging() -> Result<Logging, Error> {
        //println!("---setup_logging");
        // Check if parent process with fastlogging instance exists.
        let mut logging = create_default_logger(None);
        if let Some(server) = logging.get_server_configs().get(0) {
            let port_file = get_port_file(process::id());
            // Server config above is just a copy. So we need to access the original directly.
            logging
                .instance
                .lock()
                .unwrap()
                .servers
                .values()
                .collect::<Vec<_>>()
                .get_mut(0)
                .unwrap()
                .config
                .lock()
                .unwrap()
                .port_file = Some(port_file.clone());
            println!("Create port_file {port_file:?} for port {}", server.port);
            //println!("SERVER_AUTH_KEY {:?}", logging.get_server_auth_key());
            let mut file = fs::File::create(port_file)?;
            file.write_all(&u16::to_le_bytes(server.port))?;
            file.write_all(&logging.get_server_auth_key().to_bytes())?;
        }
        if let Some((server_address, encryption)) = get_parent_server_address()? {
            //println!("** CHILD {} ", process::id());
            // Connect to parent server port
            let client = ClientWriterConfig::new(NOTSET, server_address, encryption);
            *PARENT_LOGGER_ADDRESS.lock().unwrap() = Some((getppid(), client.clone()));
            //println!("ADD_WRITER {client:?}");
            logging.add_writer(&WriterConfigEnum::Client(client))?;
        } else {
            //println!("** ROOT");
            // If default config file exists, then use this configuration. Else create default console logger.
            let config_file = default_config_file();
            //println!("config_file={config_file:?}");
            if config_file.1.is_empty() {
                //println!("add_writer CONSOLE");
                logging.add_writer(&WriterConfigEnum::Console(ConsoleWriterConfig::new(
                    NOTSET, false,
                )))?;
            } else {
                logging.apply_config(&config_file.0)?;
            }
        }
        Ok(logging)
    }

    //println!("Setup ROOT_LOGGER");
    let logging = match setup_logging() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to setup default logger: {e}");
            create_default_logger(None)
        }
    };
    //println!("ROOT_LOGGER={logging:?}");
    Mutex::new(logging)
});
