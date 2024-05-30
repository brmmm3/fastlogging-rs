use std::io::Error;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use fastlogging::{
    ClientWriterConfig, CompressionMethodEnum, ConsoleWriterConfig, EncryptionMethod, ExtConfig,
    FileWriterConfig, Logging, MessageStructEnum, ServerConfig, DEBUG, ERROR, FATAL, INFO,
};

const MB: usize = 1024 * 1024;

fn main() -> Result<(), Error> {
    let mut logger = Logging::default();
    logger.save_config(Path::new("/tmp/config_default.json"))?;
    logger.save_config(Path::new("/tmp/config_default.xml"))?;
    logger.save_config(Path::new("/tmp/config_default.yaml"))?;
    logger.shutdown(false)?;
    let mut logger = Logging::new(
        Some(INFO),
        Some("main".to_string()),
        Some(ExtConfig::new(
            MessageStructEnum::String,
            true,
            true,
            true,
            true,
            true,
        )),
        Some(ConsoleWriterConfig::new(ERROR, true)),
        Some(FileWriterConfig::new(
            DEBUG,
            PathBuf::from("/tmp/write_config_file.log"),
            MB,
            4,
            Some(Duration::from_secs(3600)),
            Some(
                SystemTime::now()
                    .checked_add(Duration::from_secs(1200))
                    .unwrap(),
            ),
            Some(CompressionMethodEnum::Deflate),
        )?),
        Some(ServerConfig::new(
            ERROR,
            "127.0.0.1:12345",
            EncryptionMethod::NONE,
        )),
        Some(ClientWriterConfig::new(
            FATAL,
            "127.0.0.1:12346",
            EncryptionMethod::NONE,
        )),
        Some(DEBUG),
        None,
    )?;
    logger.save_config(Path::new("/tmp/config_full.json"))?;
    logger.save_config(Path::new("/tmp/config_full.xml"))?;
    logger.save_config(Path::new("/tmp/config_full.yaml"))?;
    logger.shutdown(false)?;
    Ok(())
}
