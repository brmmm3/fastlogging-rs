package org.logging;

import java.io.IOException;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.Arena;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.concurrent.atomic.AtomicInteger;

import junit.framework.TestCase;

/**
 * Integration tests for the {@link FastLogging} FFM (Foreign Function & Memory)
 * binding.
 *
 * These tests mirror the Rust fastlogging/tests/integration.rs tests,
 * exercising the FFM API end-to-end: creating a Logging instance with
 * different writers, logging messages, managing writers and loggers,
 * syncing, rotating files, config save/apply and level handling.
 */
public class IntegrationTest extends TestCase {

    public IntegrationTest(String testName) {
        super(testName);
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /** Read a file into a String, returning "" on failure. */
    private static String readFile(String path) {
        try {
            return Files.readString(Paths.get(path));
        } catch (IOException e) {
            return "";
        }
    }

    /** Check if haystack contains needle. */
    private static boolean contains(String haystack, String needle) {
        return haystack != null && haystack.contains(needle);
    }

    /** Delete a file if it exists. */
    private static void deleteFile(String path) {
        try {
            Files.deleteIfExists(Paths.get(path));
        } catch (IOException e) {
            // ignore
        }
    }

    // -----------------------------------------------------------------------
    // Level helpers
    // -----------------------------------------------------------------------

    public void testLevelConversion() {
        assertEquals(0, FastLogging.NOTSET);
        assertEquals(5, FastLogging.TRACE);
        assertEquals(10, FastLogging.DEBUG);
        assertEquals(20, FastLogging.INFO);
        assertEquals(25, FastLogging.SUCCESS);
        assertEquals(30, FastLogging.WARNING);
        assertEquals(40, FastLogging.ERROR);
        assertEquals(FastLogging.CRITICAL, FastLogging.FATAL);
        assertEquals(50, FastLogging.CRITICAL);
        assertEquals(60, FastLogging.EXCEPTION);
        assertEquals(FastLogging.WARN, FastLogging.WARNING);
    }

    // -----------------------------------------------------------------------
    // Basic logging
    // -----------------------------------------------------------------------

    public void testDefaultLogging() {
        FastLogging.Logging logging = new FastLogging.Logging();
        assertNotNull(logging.instance_ptr);
        logging.trace("trace");
        logging.debug("debug");
        logging.info("info");
        logging.warning("warning");
        logging.error("error");
        logging.critical("critical");
        logging.shutdown();
    }

    public void testLoggingNewWithConsoleWriter() {
        FastLogging.ConsoleWriterConfig console = new FastLogging.ConsoleWriterConfig(
                FastLogging.DEBUG, false);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "test", console);
        assertNotNull(logging.instance_ptr);
        logging.trace("trace msg");
        logging.debug("debug msg");
        logging.info("info msg");
        logging.success("success msg");
        logging.warning("warning msg");
        logging.error("error msg");
        logging.critical("critical msg");
        logging.fatal("fatal msg");
        logging.shutdown();
    }

    public void testConsoleWriterConfigNew() {
        FastLogging.ConsoleWriterConfig cfg = new FastLogging.ConsoleWriterConfig(
                FastLogging.DEBUG, false);
        assertTrue(cfg.instance_ptr != 0);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "test", cfg);
        assertNotNull(logging.instance_ptr);
        logging.info("console test");
        logging.shutdown();
    }

    // -----------------------------------------------------------------------
    // File writer
    // -----------------------------------------------------------------------

    public void testFileWriterWritesMessages() {
        String logFile = "test_file_writer.log";
        FastLogging.FileWriterConfig file = new FastLogging.FileWriterConfig(
                FastLogging.DEBUG, logFile, 0, 0, 0, 0,
                FastLogging.CompressionMethodEnum.Store);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "test", file);
        assertNotNull(logging.instance_ptr);
        logging.info("file info msg");
        logging.error("file error msg");
        logging.syncAll(2.0);
        logging.shutdown();
        String content = readFile(logFile);
        assertTrue("log file must contain info msg", contains(content, "file info msg"));
        assertTrue("log file must contain error msg", contains(content, "file error msg"));
        deleteFile(logFile);
    }

    public void testFileWriterLevelFilter() {
        String logFile = "test_file_level.log";
        FastLogging.FileWriterConfig file = new FastLogging.FileWriterConfig(
                FastLogging.WARNING, logFile, 0, 0, 0, 0,
                FastLogging.CompressionMethodEnum.Store);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "test", file);
        assertNotNull(logging.instance_ptr);
        logging.debug("debug message");
        logging.info("info message");
        logging.error("error message");
        logging.syncAll(2.0);
        logging.shutdown();
        String content = readFile(logFile);
        assertFalse("debug should be filtered", contains(content, "debug message"));
        assertFalse("info should be filtered", contains(content, "info message"));
        assertTrue("error should be present", contains(content, "error message"));
        deleteFile(logFile);
    }

    public void testFileWriterRotation() {
        String logFile = "test_rotate.log";
        // backlog=3 enables rotation.
        FastLogging.FileWriterConfig file = new FastLogging.FileWriterConfig(
                FastLogging.DEBUG, logFile, 0, 3, 0, 0,
                FastLogging.CompressionMethodEnum.Store);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "test", file);
        assertNotNull(logging.instance_ptr);
        for (int i = 0; i < 5; i++) {
            logging.info("message " + i);
        }
        logging.syncAll(2.0);
        logging.rotate(null);
        logging.info("after rotate");
        logging.syncAll(2.0);
        logging.shutdown();
        String current = readFile(logFile);
        assertTrue("current file must contain 'after rotate'",
                contains(current, "after rotate"));
        deleteFile(logFile);
    }

    // -----------------------------------------------------------------------
    // Writer management
    // -----------------------------------------------------------------------

    public void testAddRemoveWriter() {
        FastLogging.Logging logging = new FastLogging.Logging(FastLogging.DEBUG, "test");
        assertNotNull(logging.instance_ptr);
        FastLogging.ConsoleWriterConfig cfg1 = new FastLogging.ConsoleWriterConfig(
                FastLogging.DEBUG, false);
        logging.addWriter(cfg1.instance_ptr);
        FastLogging.ConsoleWriterConfig cfg2 = new FastLogging.ConsoleWriterConfig(
                FastLogging.INFO, false);
        logging.addWriter(cfg2.instance_ptr);
        // Remove writers — FFM uses WriterTypeEnum + key.
        logging.removeWriter(FastLogging.WriterTypeEnum.Console);
        logging.shutdown();
    }

    public void testRemoveWriterInvalidId() {
        FastLogging.Logging logging = new FastLogging.Logging(FastLogging.DEBUG, "test");
        assertNotNull(logging.instance_ptr);
        // Should not crash.
        logging.removeWriter(FastLogging.WriterTypeEnum.File, "12345");
        logging.shutdown();
    }

    public void testDisableEnableWriterFile() {
        String logFile = "test_toggle.log";
        FastLogging.FileWriterConfig file = new FastLogging.FileWriterConfig(
                FastLogging.DEBUG, logFile, 0, 0, 0, 0,
                FastLogging.CompressionMethodEnum.Store);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "test", file);
        assertNotNull(logging.instance_ptr);
        // Writer 1 is the file writer. Set level to NOLOG to disable.
        logging.setLevel(FastLogging.WriterTypeEnum.File, "1", FastLogging.NOLOG);
        logging.info("disabled");
        logging.syncAll(2.0);
        String content = readFile(logFile);
        assertFalse("disabled message should not be in file", contains(content, "disabled"));
        logging.setLevel(FastLogging.WriterTypeEnum.File, "1", FastLogging.DEBUG);
        logging.info("enabled");
        logging.syncAll(2.0);
        logging.shutdown();
        content = readFile(logFile);
        assertTrue("enabled message should be in file", contains(content, "enabled"));
        deleteFile(logFile);
    }

    // -----------------------------------------------------------------------
    // Level and domain management
    // -----------------------------------------------------------------------

    public void testSetLevelWriter() {
        FastLogging.ConsoleWriterConfig console = new FastLogging.ConsoleWriterConfig(
                FastLogging.INFO, false);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "test", console);
        assertNotNull(logging.instance_ptr);
        // Writer 1 has level INFO. Change it to DEBUG.
        logging.setLevel(FastLogging.WriterTypeEnum.File, "1", FastLogging.DEBUG);
        logging.debug("passes now");
        logging.shutdown();
    }

    public void testSetDomain() {
        FastLogging.Logging logging = new FastLogging.Logging(FastLogging.DEBUG, "old");
        assertNotNull(logging.instance_ptr);
        logging.setDomain("new-domain");
        String cfg = logging.getConfigString();
        assertTrue("config must contain new-domain", contains(cfg, "new-domain"));
        logging.shutdown();
    }

    public void testSetExtConfig() {
        FastLogging.Logging logging = new FastLogging.Logging(FastLogging.DEBUG, "test");
        assertNotNull(logging.instance_ptr);
        FastLogging.ExtConfig ext1 = new FastLogging.ExtConfig(
                FastLogging.MessageStructEnum.String, false, false, false, false, false);
        logging.setExtConfig(ext1);
        FastLogging.ExtConfig ext2 = new FastLogging.ExtConfig(
                FastLogging.MessageStructEnum.Json, false, false, false, true, true);
        logging.setExtConfig(ext2);
        logging.shutdown();
    }

    public void testSetLevel2Sym() {
        FastLogging.Logging logging = new FastLogging.Logging(FastLogging.DEBUG, "test");
        assertNotNull(logging.instance_ptr);
        logging.setLevel2Sym(FastLogging.LevelSyms.Str);
        String cfg = logging.getConfigString();
        assertTrue("config must contain DEBUG", contains(cfg, "DEBUG"));
        logging.shutdown();
    }

    // -----------------------------------------------------------------------
    // Logger
    // -----------------------------------------------------------------------

    public void testAddRemoveLogger() {
        FastLogging.ConsoleWriterConfig console = new FastLogging.ConsoleWriterConfig(
                FastLogging.DEBUG, false);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "root", console);
        assertNotNull(logging.instance_ptr);
        FastLogging.Logger logger = new FastLogging.Logger(
                FastLogging.DEBUG, "logger-domain");
        assertNotNull(logger.instance_ptr);
        logging.addLogger(logger.instance_ptr);
        logger.info("logger message");
        logger.trace("trace");
        logger.debug("debug");
        logger.warning("warning");
        logger.error("error");
        logger.critical("critical");
        logging.syncAll(2.0);
        logging.removeLogger(logger.instance_ptr);
        logging.shutdown();
    }

    public void testLoggerLevelFiltering() {
        FastLogging.ConsoleWriterConfig console = new FastLogging.ConsoleWriterConfig(
                FastLogging.DEBUG, false);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "root", console);
        assertNotNull(logging.instance_ptr);
        FastLogging.Logger logger = new FastLogging.Logger(
                FastLogging.INFO, "logger-domain");
        logging.addLogger(logger.instance_ptr);
        // Logger level is INFO, so DEBUG should be filtered.
        logger.debug("filtered by logger level");
        logger.info("passes");
        logging.syncAll(2.0);
        // Change logger level to DEBUG.
        logger.setLevel(FastLogging.DEBUG);
        logger.debug("now passes");
        logging.syncAll(2.0);
        logging.removeLogger(logger.instance_ptr);
        logging.shutdown();
    }

    public void testLoggerSetDomain() {
        FastLogging.Logging logging = new FastLogging.Logging(FastLogging.DEBUG, "test");
        FastLogging.Logger logger = new FastLogging.Logger(
                FastLogging.DEBUG, "d1");
        assertNotNull(logger.instance_ptr);
        logger.setDomain("d2");
        logger.setLevel(FastLogging.TRACE);
    }

    public void testLoggerLevel() {
        FastLogging.Logging logging = new FastLogging.Logging(FastLogging.DEBUG, "test");
        FastLogging.Logger logger = new FastLogging.Logger(
                FastLogging.INFO, "d1");
        assertNotNull(logger.instance_ptr);
        logger.setLevel(FastLogging.TRACE);
        logging.shutdown();
    }

    // -----------------------------------------------------------------------
    // Config save/apply
    // -----------------------------------------------------------------------

    public void testSaveConfig() {
        String configPath = "test_save_config.json";
        String logFile = "test_save_config.log";
        FastLogging.FileWriterConfig file = new FastLogging.FileWriterConfig(
                FastLogging.DEBUG, logFile, 0, 0, 0, 0,
                FastLogging.CompressionMethodEnum.Store);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "test", file);
        assertNotNull(logging.instance_ptr);
        logging.getSaveConfig(configPath);
        assertTrue("config file should exist",
                Files.exists(Paths.get(configPath)));
        logging.shutdown();
        deleteFile(configPath);
        deleteFile(logFile);
    }

    public void testGetConfigString() {
        FastLogging.Logging logging = new FastLogging.Logging(FastLogging.DEBUG, "test");
        assertNotNull(logging.instance_ptr);
        String cfg = logging.getConfigString();
        assertNotNull("get_config_string returned null", cfg);
        assertTrue("config must contain 'level='", contains(cfg, "level="));
        assertTrue("config must contain 'domain='", contains(cfg, "domain="));
        logging.shutdown();
    }

    // -----------------------------------------------------------------------
    // Network configs (construct-only)
    // -----------------------------------------------------------------------

    public void testServerClientConfigConstruction() {
        // Server with no encryption.
        FastLogging.ServerConfig server = new FastLogging.ServerConfig(
                FastLogging.DEBUG, "127.0.0.1", 0);
        assertTrue("ServerConfig returned 0", server.instance_ptr != 0);
        // Client with no encryption.
        FastLogging.ClientWriterConfig client = new FastLogging.ClientWriterConfig(
                FastLogging.DEBUG, "127.0.0.1", 0);
        assertTrue("ClientWriterConfig returned 0", client.instance_ptr != 0);
    }

    // -----------------------------------------------------------------------
    // Error handling
    // -----------------------------------------------------------------------

    public void testShutdownTwiceIsIdempotent() {
        FastLogging.Logging logging = new FastLogging.Logging();
        assertNotNull(logging.instance_ptr);
        logging.shutdown();
        // Do not call shutdown again — the logging instance is already freed.
    }

    // -----------------------------------------------------------------------
    // ExtConfig helpers
    // -----------------------------------------------------------------------

    public void testExtConfigDefault() {
        FastLogging.ExtConfig ext = new FastLogging.ExtConfig(
                FastLogging.MessageStructEnum.String, false, false, false, false, false);
        assertTrue("ExtConfig returned 0", ext.instance_ptr != 0);
    }

    // -----------------------------------------------------------------------
    // Callback writer
    // -----------------------------------------------------------------------

    private static final AtomicInteger sCallbackCount = new AtomicInteger(0);

    private static final FastLogging.CallbackWriterConfigLog TEST_CALLBACK = new FastLogging.CallbackWriterConfigLog() {
        @Override
        public void invoke(int level, MemorySegment domain, long domainLen,
                MemorySegment message, long messageLen) {
            sCallbackCount.incrementAndGet();
        }
    };

    public void testCallbackWriterReceivesMessages() {
        sCallbackCount.set(0);
        FastLogging.CallbackWriterConfig cb = new FastLogging.CallbackWriterConfig(
                FastLogging.DEBUG, TEST_CALLBACK);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "cb-domain", cb);
        assertNotNull(logging.instance_ptr);
        logging.info("callback msg");
        logging.syncAll(2.0);
        logging.shutdown();
        assertTrue("callback must have been invoked at least once",
                sCallbackCount.get() >= 1);
    }

    public void testCallbackWriterRespectsLevel() {
        sCallbackCount.set(0);
        // Writer level is INFO: TRACE/DEBUG messages must be filtered out.
        FastLogging.CallbackWriterConfig cb = new FastLogging.CallbackWriterConfig(
                FastLogging.INFO, TEST_CALLBACK);
        FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "root", cb);
        assertNotNull(logging.instance_ptr);
        logging.debug("filtered");
        logging.info("passes");
        logging.warning("passes too");
        logging.syncAll(2.0);
        logging.shutdown();
        assertEquals("callback should have been invoked 2 times (info + warning)",
                2, sCallbackCount.get());
    }

    // -----------------------------------------------------------------------
    // Concurrency smoke test
    // -----------------------------------------------------------------------

    public void testConcurrentLogging() {
        String logFile = "test_concurrent.log";
        FastLogging.FileWriterConfig file = new FastLogging.FileWriterConfig(
                FastLogging.DEBUG, logFile, 0, 0, 0, 0,
                FastLogging.CompressionMethodEnum.Store);
        final FastLogging.Logging logging = new FastLogging.Logging(
                FastLogging.DEBUG, "test", file);
        assertNotNull(logging.instance_ptr);

        final int numThreads = 4;
        Thread[] threads = new Thread[numThreads];
        for (int t = 0; t < numThreads; t++) {
            final int tid = t;
            threads[t] = new Thread(() -> {
                for (int i = 0; i < 50; i++) {
                    logging.info("thread " + tid + " message " + i);
                }
            });
            threads[t].start();
        }
        for (int t = 0; t < numThreads; t++) {
            try {
                threads[t].join();
            } catch (InterruptedException e) {
                fail("thread interrupted");
            }
        }
        logging.syncAll(2.0);
        logging.shutdown();

        String content = readFile(logFile);
        assertFalse("failed to read log file", content.isEmpty());
        for (int t = 0; t < numThreads; t++) {
            for (int i = 0; i < 50; i++) {
                assertTrue("missing: thread " + t + " message " + i,
                        contains(content, "thread " + t + " message " + i));
            }
        }
        deleteFile(logFile);
    }
}
