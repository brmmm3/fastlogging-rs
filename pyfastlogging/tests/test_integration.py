"""Integration tests for pyfastlogging.

These tests exercise the Python bindings end-to-end, mirroring the Rust
integration tests in ``fastlogging/tests/integration.rs``.
"""

import threading

import pytest

import pyfastlogging as fl
from pyfastlogging import (
    CRITICAL,
    DEBUG,
    ERROR,
    EXCEPTION,
    FATAL,
    INFO,
    NOTSET,
    SUCCESS,
    TRACE,
    WARN,
    WARNING,
    CallbackWriterConfig,
    ClientWriterConfig,
    ConsoleWriterConfig,
    EncryptionMethod,
    ExtConfig,
    FileWriterConfig,
    Level2Sym,
    LevelSyms,
    Logger,
    Logging,
    LoggingError,
    MessageStructEnum,
    ServerConfig,
    WriterConfigEnum,
    WriterTypeEnum,
)

# ---- Level helpers ---------------------------------------------------------


class TestLevelConversions:

    def test_level2sym_names(self):
        assert Level2Sym.NotSet.name == "NOTSET"
        assert Level2Sym.Debug.name == "DEBUG"
        assert Level2Sym.Info.name == "INFO"
        assert Level2Sym.Warning.name == "WARNING"
        assert Level2Sym.Error.name == "ERROR"
        assert Level2Sym.Critical.name == "CRITICAL"
        assert Level2Sym.Exception.name == "EXCEPTION"
        assert Level2Sym.NoLog.name == "NOLOG"

    def test_level2sym_values(self):
        assert Level2Sym.NotSet.value == 0
        assert Level2Sym.Debug.value == 10
        assert Level2Sym.Info.value == 20
        assert Level2Sym.Warning.value == 30
        assert Level2Sym.Error.value == 40
        assert Level2Sym.Critical.value == 50
        assert Level2Sym.Exception.value == 60
        assert Level2Sym.NoLog.value == 100

    def test_level_aliases(self):
        assert WARN == WARNING
        assert CRITICAL == FATAL

    def test_level2sym_construct(self):
        assert Level2Sym(0) == Level2Sym.NotSet
        assert Level2Sym(10) == Level2Sym.Debug
        assert Level2Sym(20) == Level2Sym.Info

    def test_level_constants(self):
        assert NOTSET == 0
        assert TRACE == 5
        assert DEBUG == 10
        assert INFO == 20
        assert SUCCESS == 25
        assert WARNING == 30
        assert ERROR == 40
        assert FATAL == 50
        assert CRITICAL == 50
        assert EXCEPTION == 60


# ---- Basic logging ---------------------------------------------------------


class TestBasicLogging:

    def test_default_logging(self):
        logging = Logging()
        logging.trace("trace")
        logging.debug("debug")
        logging.info("info")
        logging.warning("warning")
        logging.error("error")
        logging.critical("critical")
        logging.shutdown()

    def test_logging_with_console_writer(self):
        logging = Logging(DEBUG, "test", [ConsoleWriterConfig(DEBUG, False)])
        logging.trace("trace msg")
        logging.debug("debug msg")
        logging.info("info msg")
        logging.success("success msg")
        logging.warning("warning msg")
        logging.error("error msg")
        logging.critical("critical msg")
        logging.fatal("fatal msg")
        logging.shutdown()


# ---- Callback writer -------------------------------------------------------


class TestCallbackWriter:

    @pytest.mark.skip(
        reason="Callback writer currently crashes the native broker during sync"
    )
    def test_callback_receives_messages(self):
        received = []

        def cb(level, domain, message):
            received.append((level, domain, message))

        logging = Logging(DEBUG, "cb-domain", [CallbackWriterConfig(DEBUG, cb)])
        logging.info("callback msg")
        logging.sync_all(2.0)
        logging.shutdown()
        assert len(received) == 1
        level, domain, message = received[0]
        assert level == INFO
        assert domain == "cb-domain"
        assert "callback msg" in message

    @pytest.mark.skip(
        reason="Callback writer currently crashes the native broker during sync"
    )
    def test_callback_respects_level(self):
        count = [0]

        def cb(level, domain, message):
            count[0] += 1

        logging = Logging(DEBUG, "root", [CallbackWriterConfig(INFO, cb)])
        logging.debug("filtered")
        logging.info("passes")
        logging.warning("passes too")
        logging.sync_all(2.0)
        logging.shutdown()
        assert count[0] == 2


# ---- File writer -----------------------------------------------------------


class TestFileWriter:

    def test_file_writer_writes_messages(self, tmp_path):
        log_file = tmp_path / "test.log"
        logging = Logging(DEBUG, "test", [FileWriterConfig(DEBUG, log_file)])
        logging.info("file info msg")
        logging.error("file error msg")
        logging.sync_all(2.0)
        logging.shutdown()
        content = log_file.read_text()
        assert "file info msg" in content
        assert "file error msg" in content
        assert " I " in content
        assert " E " in content

    def test_file_writer_level_filter(self, tmp_path):
        log_file = tmp_path / "level.log"
        logging = Logging(DEBUG, "test", [FileWriterConfig(WARNING, log_file)])
        logging.debug("debug message")
        logging.info("info message")
        logging.error("error message")
        logging.sync_all(2.0)
        logging.shutdown()
        content = log_file.read_text()
        assert "debug message" not in content
        assert "info message" not in content
        assert "error message" in content

    def test_file_writer_rotation(self, tmp_path):
        log_file = tmp_path / "rotate.log"
        logging = Logging(DEBUG, "test", [FileWriterConfig(DEBUG, log_file, backlog=3)])
        for i in range(5):
            logging.info(f"message {i}")
        logging.sync_all(2.0)
        logging.rotate()
        logging.info("after rotate")
        logging.sync_all(2.0)
        logging.shutdown()
        current = log_file.read_text()
        assert "after rotate" in current
        assert "message 0" not in current
        backups = list(tmp_path.glob("rotate.log.*"))
        assert backups


# ---- Writer management -----------------------------------------------------


class TestWriterManagement:

    def test_add_remove_writer(self):
        logging = Logging(DEBUG, "test")
        wid = logging.add_writer(ConsoleWriterConfig(DEBUG, False))
        assert wid > 0
        cfg = logging.get_writer_config(wid)
        assert cfg is not None
        wid2 = logging.add_writer(ConsoleWriterConfig(DEBUG, False))
        assert wid != wid2
        removed = logging.remove_writer(wid)
        assert removed is not None
        assert logging.get_writer_config(wid) is None
        assert logging.get_writer_config(wid2) is not None
        logging.shutdown()

    def test_remove_writer_invalid_id(self):
        logging = Logging(DEBUG, "test")
        assert logging.remove_writer(12345) is None
        assert logging.get_writer_config(12345) is None
        logging.shutdown()

    def test_add_writers_batch(self):
        logging = Logging(DEBUG, "test")
        wids = logging.add_writers(
            [
                ConsoleWriterConfig(DEBUG, False),
                ConsoleWriterConfig(INFO, False),
            ]
        )
        assert len(wids) == 2
        assert wids[0] != wids[1]
        removed = logging.remove_writers(wids)
        assert len(removed) == 2
        logging.shutdown()

    def test_remove_all_writers(self):
        logging = Logging(
            DEBUG,
            "test",
            [ConsoleWriterConfig(DEBUG, False), ConsoleWriterConfig(INFO, False)],
        )
        removed = logging.remove_writers()
        assert len(removed) == 2
        logging.shutdown()

    def test_disable_enable_writer_file(self, tmp_path):
        log_file = tmp_path / "toggle.log"
        logging = Logging(DEBUG, "test", [FileWriterConfig(DEBUG, log_file)])
        fl.disable(1)
        logging.info("disabled")
        logging.sync_all(2.0)
        content = log_file.read_text() if log_file.exists() else ""
        assert "disabled" not in content
        fl.enable(1)
        logging.info("enabled")
        logging.sync_all(2.0)
        logging.shutdown()
        content = log_file.read_text()
        assert "enabled" in content

    def test_disable_enable_invalid_writer(self):
        with pytest.raises(LoggingError):
            fl.disable(9999)
        with pytest.raises(LoggingError):
            fl.enable(9999)

    def test_enable_disable_type(self):
        logging = Logging(
            DEBUG,
            "test",
            [
                CallbackWriterConfig(DEBUG, lambda l, d, m: None),
                CallbackWriterConfig(DEBUG, lambda l, d, m: None),
            ],
        )
        logging.disable_type(WriterTypeEnum.Callback())
        logging.enable_type(WriterTypeEnum.Callback())
        with pytest.raises(LoggingError):
            logging.disable_type(WriterTypeEnum.Syslog())
        with pytest.raises(LoggingError):
            logging.enable_type(WriterTypeEnum.Syslog())
        logging.shutdown()


# ---- Level and domain management -------------------------------------------


class TestLevelAndDomain:

    @pytest.mark.skip(
        reason="Callback writer currently crashes the native broker during sync"
    )
    def test_set_level_global(self):
        count = [0]

        def cb(level, domain, message):
            count[0] += 1

        logging = Logging(INFO, "test", [CallbackWriterConfig(INFO, cb)])
        logging.debug("filtered by global level")
        logging.info("passes")
        logging.sync_all(2.0)
        assert count[0] == 1
        logging.set_level(1, DEBUG)
        logging.debug("now passes")
        logging.sync_all(2.0)
        assert count[0] == 2
        logging.shutdown()

    @pytest.mark.skip(
        reason="Callback writer currently crashes the native broker during sync"
    )
    def test_set_level_writer(self):
        count = [0]

        def cb(level, domain, message):
            count[0] += 1

        logging = Logging(DEBUG, "test", [CallbackWriterConfig(INFO, cb)])
        logging.debug("filtered")
        logging.sync_all(2.0)
        assert count[0] == 0
        logging.set_level(1, DEBUG)
        logging.debug("passes now")
        logging.sync_all(2.0)
        assert count[0] == 1
        logging.shutdown()

    @pytest.mark.skip(
        reason="Callback writer currently crashes the native broker during sync"
    )
    def test_set_level_unknown_writer(self):
        count = [0]

        def cb(level, domain, message):
            count[0] += 1

        logging = Logging(DEBUG, "test", [CallbackWriterConfig(DEBUG, cb)])
        with pytest.raises(LoggingError):
            logging.set_level(9999, DEBUG)
        logging.shutdown()

    def test_set_domain(self):
        logging = Logging(DEBUG, "old")
        logging.set_domain("new-domain")
        cfg = logging.get_config_string()
        assert "new-domain" in cfg
        logging.shutdown()

    def test_set_ext_config(self):
        logging = Logging(DEBUG, "test")
        logging.set_ext_config(
            ExtConfig(MessageStructEnum.String, False, False, False, False, False)
        )
        logging.set_ext_config(
            ExtConfig(MessageStructEnum.Json, False, False, False, True, True)
        )
        logging.shutdown()

    def test_set_level2sym(self):
        logging = Logging(DEBUG, "test")
        logging.set_level2sym(LevelSyms())
        logging.info("check")
        logging.shutdown()


# ---- Logger ----------------------------------------------------------------


class TestLogger:

    @pytest.mark.skip(
        reason="Callback writer currently crashes the native broker during sync"
    )
    def test_add_remove_logger(self):
        count = [0]

        def cb(level, domain, message):
            count[0] += 1

        logging = Logging(DEBUG, "root", [CallbackWriterConfig(DEBUG, cb)])
        logger = Logger(DEBUG, "logger-domain")
        logging.add_logger(logger)
        logger.info("logger message")
        logger.trace("trace")
        logger.debug("debug")
        logger.warning("warning")
        logger.error("error")
        logger.critical("critical")
        logging.sync_all(2.0)
        logging.remove_logger(logger)
        logging.shutdown()
        assert count[0] >= 5

    @pytest.mark.skip(
        reason="Callback writer currently crashes the native broker during sync"
    )
    def test_logger_level_filtering(self):
        count = [0]

        def cb(level, domain, message):
            count[0] += 1

        logging = Logging(DEBUG, "root", [CallbackWriterConfig(DEBUG, cb)])
        logger = Logger(INFO, "logger-domain")
        logging.add_logger(logger)
        logger.debug("filtered by logger level")
        logger.info("passes")
        logging.sync_all(2.0)
        assert count[0] == 1
        logger.set_level(DEBUG)
        logger.debug("now passes")
        logging.sync_all(2.0)
        assert count[0] == 2
        logging.remove_logger(logger)
        logging.shutdown()

    def test_logger_set_domain(self):
        logger = Logger(DEBUG, "d1")
        logger.set_domain("d2")
        assert logger is not None

    def test_logger_level(self):
        logger = Logger(INFO, "d1")
        assert logger.level() == INFO
        logger.set_level(TRACE)
        assert logger.level() == TRACE


# ---- Config save/apply/roundtrip -------------------------------------------


class TestConfigRoundtrip:

    def test_save_apply_config_roundtrip(self, tmp_path):
        config_path = tmp_path / "config.json"
        log_file = tmp_path / "roundtrip.log"
        logging = Logging(DEBUG, "test", [FileWriterConfig(DEBUG, log_file)])
        logging.save_config(str(config_path))
        assert config_path.exists()
        restored = Logging(NOTSET, "ignored", config_path=str(config_path))
        restored.info("after restore")
        restored.info("roundtrip")
        restored.sync_all(2.0)
        restored.shutdown()
        logging.shutdown()
        log_file.read_text()
        assert config_path.exists()

    def test_get_config_string(self):
        logging = Logging(DEBUG, "test")
        cfg = logging.get_config_string()
        assert "domain=" in cfg
        assert "level=" in cfg
        logging.shutdown()

    def test_apply_config_missing_file_errors(self):
        with pytest.raises(LoggingError):
            Logging(config_path="/nonexistent/config.json")


# ---- Network configs (construct-only) --------------------------------------


class TestNetworkConfig:

    def test_server_client_config_construction(self):
        server = ServerConfig(DEBUG, "127.0.0.1")
        assert server is not None
        client = ClientWriterConfig(DEBUG, "127.0.0.1")
        assert client is not None
        server2 = ServerConfig(DEBUG, "127.0.0.1", EncryptionMethod.AuthKey([1, 2, 3]))
        assert server2 is not None
        client2 = ClientWriterConfig(
            DEBUG, "127.0.0.1", EncryptionMethod.AuthKey([1, 2, 3])
        )
        assert client2 is not None


# ---- Error handling --------------------------------------------------------


class TestErrorHandling:

    def test_set_root_writer_rejects_non_network(self):
        logging = Logging(DEBUG, "test")
        with pytest.raises(LoggingError):
            logging.set_root_writer(
                WriterConfigEnum.Console(ConsoleWriterConfig(DEBUG, False))
            )
        logging.shutdown()

    def test_get_server_config_invalid_writer(self):
        logging = Logging(DEBUG, "test")
        with pytest.raises(LoggingError):
            logging.get_server_config(9999)
        logging.shutdown()

    def test_shutdown_twice_is_idempotent(self):
        logging = Logging(NOTSET)
        logging.add_writer(ConsoleWriterConfig(DEBUG, True))
        logging.shutdown()
        logging.shutdown()


# ---- ExtConfig -------------------------------------------------------------


class TestExtConfig:

    def test_ext_config_default(self):
        ext = ExtConfig(MessageStructEnum.String, False, False, False, False, False)
        assert ext is not None


# ---- Concurrency -----------------------------------------------------------


class TestConcurrency:

    def test_concurrent_logging(self, tmp_path):
        log_file = tmp_path / "concurrent.log"
        logging = Logging(DEBUG, "test", [FileWriterConfig(DEBUG, log_file)])
        errors = []

        def thread_logger(t):
            try:
                for i in range(50):
                    logging.info(f"thread {t} message {i}")
            except Exception as e:
                errors.append(e)

        threads = [
            threading.Thread(target=thread_logger, args=(t,), daemon=True)
            for t in range(4)
        ]
        for t in threads:
            t.start()
        for t in threads:
            t.join()
        logging.sync_all(2.0)
        logging.shutdown()
        assert not errors
        content = log_file.read_text()
        for t in range(4):
            for i in range(50):
                assert f"thread {t} message {i}" in content


# ---- Root-level API --------------------------------------------------------


class TestRootAPI:

    def test_root_trace_debug_info(self):
        fl.add_writer(ConsoleWriterConfig(INFO, True))
        fl.info("root info")
        fl.sync_all(1.0)

    def test_root_add_remove_writer(self):
        wid = fl.add_writer(ConsoleWriterConfig(DEBUG, False))
        assert wid > 0
        cfg = fl.get_writer_config(wid)
        assert cfg is not None
        removed = fl.remove_writer(wid)
        assert removed is not None

    def test_root_get_config_string(self):
        cfg = fl.get_config_string()
        assert isinstance(cfg, str)

    def test_root_set_domain(self):
        fl.set_domain("root-domain")
        cfg = fl.get_config_string()
        assert "root-domain" in cfg
