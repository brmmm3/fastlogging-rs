import tempfile

from pyfastlogging import (
    TRACE,
    DEBUG,
    Logging,
    ConsoleWriterConfig,
    FileWriterConfig,
    ServerConfig,
    ClientWriterConfig,
)


def SomeThread(logger):
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.success("Success Message")
    logger.warning("Warning Message")
    logger.error("Error Message")
    logger.fatal("Fatal Message")


if __name__ == "__main__":
    tmpDir = tempfile.mkdtemp(prefix="fastlogging")
    logging_server = Logging(
        TRACE,
        "LOGSRV",
        [
            ConsoleWriterConfig(TRACE, True),
            FileWriterConfig(TRACE, f"{tmpDir}/fastlogging.log"),
            ServerConfig(TRACE, "127.0.0.1"),
        ],
    )
    logging_server.sync_all(5.0)
    address = logging_server.get_server_address()
    print(address)
    key = logging_server.get_server_auth_key()
    print(key)
    logging_client = Logging(
        TRACE, "LOGCLIENT", [ClientWriterConfig(DEBUG, address, key)]
    )
    logging_client.trace("Trace Message")
    logging_client.debug("Debug Message")
    logging_client.info("Info Message")
    logging_client.success("Success Message")
    logging_client.warning("Warning Message")
    logging_client.error("Error Message")
    logging_client.fatal("Fatal Message")

    logging_server.trace("Trace Message")
    logging_server.debug("Debug Message")
    logging_server.info("Info Message")
    logging_server.success("Success Message")
    logging_server.warning("Warning Message")
    logging_server.error("Error Message")
    logging_server.fatal("Fatal Message")

    logging_client.sync_all(1.0)
    logging_server.sync_all(1.0)

    logging_client.shutdown()
    logging_server.shutdown()
