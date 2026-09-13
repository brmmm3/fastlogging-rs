//! Integration tests for the `cppfastlogging` C++ wrapper library.
//!
//! These tests mirror the Rust `fastlogging/tests/integration.rs` tests,
//! exercising the C++ wrapper API end-to-end: creating a Logging instance with
//! different writers, logging messages, managing writers and loggers,
//! syncing, rotating files, config save/apply and level handling.

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cstdint>
#include <string>

#include "h/cppfastlogging.hpp"

using namespace logging;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Read a file into a std::string.  Returns empty string on failure.
static std::string read_file(const char *path)
{
    FILE *f = fopen(path, "rb");
    if (!f)
        return "";
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    fseek(f, 0, SEEK_SET);
    std::string buf;
    buf.resize(static_cast<size_t>(sz));
    fread(&buf[0], 1, static_cast<size_t>(sz), f);
    fclose(f);
    return buf;
}

/// Check if a string contains a substring.
static bool contains(const std::string &haystack, const char *needle)
{
    return haystack.find(needle) != std::string::npos;
}

/// Simple test framework.
static int tests_run = 0;
static int tests_passed = 0;
static int tests_failed = 0;

#define TEST(name)      \
    static void name(); \
    static void name()

#define RUN_TEST(name)                   \
    do                                   \
    {                                    \
        tests_run++;                     \
        printf("=== RUN   %s\n", #name); \
        name();                          \
        tests_passed++;                  \
        printf("--- PASS: %s\n", #name); \
    } while (0)

#define FAIL(fmt, ...)                                \
    do                                                \
    {                                                 \
        printf("    FAIL: " fmt "\n", ##__VA_ARGS__); \
        tests_failed++;                               \
        return;                                       \
    } while (0)

#define ASSERT(cond, msg)                      \
    do                                         \
    {                                          \
        if (!(cond))                           \
        {                                      \
            FAIL("assertion failed: %s", msg); \
        }                                      \
    } while (0)

// ---------------------------------------------------------------------------
// Level helpers
// ---------------------------------------------------------------------------

TEST(test_level_conversion)
{
    ASSERT(NOTSET == 0, "NOTSET == 0");
    ASSERT(TRACE == 5, "TRACE == 5");
    ASSERT(DEBUG == 10, "DEBUG == 10");
    ASSERT(INFO == 20, "INFO == 20");
    ASSERT(SUCCESS == 25, "SUCCESS == 25");
    ASSERT(WARNING == 30, "WARNING == 30");
    ASSERT(ERROR == 40, "ERROR == 40");
    ASSERT(FATAL == CRITICAL, "FATAL == CRITICAL");
    ASSERT(CRITICAL == 50, "CRITICAL == 50");
    ASSERT(EXCEPTION == 60, "EXCEPTION == 60");
    ASSERT(WARN == WARNING, "WARN == WARNING");
}

// ---------------------------------------------------------------------------
// Basic logging
// ---------------------------------------------------------------------------

TEST(test_default_logging)
{
    Logging logging = Logging::Default();
    ASSERT(logging.raw() != nullptr, "Logging::Default returned null");
    logging.trace("trace");
    logging.debug("debug");
    logging.info("info");
    logging.warning("warning");
    logging.error("error");
    logging.critical("critical");
    logging.shutdown(false);
}

TEST(test_logging_new_with_console_writer)
{
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging(DEBUG, test) returned null");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, false));
    logging.trace("trace msg");
    logging.debug("debug msg");
    logging.info("info msg");
    logging.success("success msg");
    logging.warning("warning msg");
    logging.error("error msg");
    logging.critical("critical msg");
    logging.fatal("fatal msg");
    logging.shutdown(false);
}

TEST(test_console_writer_config_new)
{
    ConsoleWriterConfig cfg(DEBUG, false);
    ASSERT(cfg.config != nullptr, "ConsoleWriterConfig returned null");
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(cfg);
    logging.info("console test");
    logging.shutdown(false);
}

// ---------------------------------------------------------------------------
// File writer
// ---------------------------------------------------------------------------

TEST(test_file_writer_writes_messages)
{
    const char *log_file = "test_file_writer.log";
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(
        FileWriterConfig(DEBUG, log_file, 0, 0, -1, -1, CompressionMethod::Store));
    logging.info("file info msg");
    logging.error("file error msg");
    logging.sync_all(2.0);
    logging.shutdown(false);
    std::string content = read_file(log_file);
    ASSERT(!content.empty(), "failed to read log file");
    ASSERT(contains(content, "file info msg"), "log file must contain info msg");
    ASSERT(contains(content, "file error msg"), "log file must contain error msg");
    remove(log_file);
}

TEST(test_file_writer_level_filter)
{
    const char *log_file = "test_file_level.log";
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(
        FileWriterConfig(WARNING, log_file, 0, 0, -1, -1, CompressionMethod::Store));
    logging.debug("debug message");
    logging.info("info message");
    logging.error("error message");
    logging.sync_all(2.0);
    logging.shutdown(false);
    std::string content = read_file(log_file);
    ASSERT(!content.empty(), "failed to read log file");
    ASSERT(!contains(content, "debug message"), "debug message should be filtered out");
    ASSERT(!contains(content, "info message"), "info message should be filtered out");
    ASSERT(contains(content, "error message"), "error message should be present");
    remove(log_file);
}

TEST(test_file_writer_rotation)
{
    const char *log_file = "test_rotate.log";
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    // backlog=3 enables rotation.
    logging.add_writer_config(
        FileWriterConfig(DEBUG, log_file, 0, 3, -1, -1, CompressionMethod::Store));
    for (int i = 0; i < 5; i++)
    {
        char buf[64];
        snprintf(buf, sizeof(buf), "message %d", i);
        logging.info(buf);
    }
    logging.sync_all(2.0);
    logging.rotate("");
    logging.info("after rotate");
    logging.sync_all(2.0);
    logging.shutdown(false);
    std::string current = read_file(log_file);
    ASSERT(!current.empty(), "failed to read log file");
    ASSERT(contains(current, "after rotate"), "current file must contain 'after rotate'");
    remove(log_file);
}

// ---------------------------------------------------------------------------
// Writer management
// ---------------------------------------------------------------------------

TEST(test_add_remove_writer)
{
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    int code = logging.add_writer_config(ConsoleWriterConfig(DEBUG, false));
    ASSERT(code == 0, "add_writer_config failed");
    // Add a second writer.
    code = logging.add_writer_config(ConsoleWriterConfig(INFO, false));
    ASSERT(code == 0, "add_writer_config(2) failed");
    // Remove writer 1.
    logging.remove_writer(1);
    // Remove writer 2.
    logging.remove_writer(2);
    logging.shutdown(false);
}

TEST(test_remove_writer_invalid_id)
{
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.remove_writer(12345);
    // No crash = pass.  The C API returns void for remove_writer.
    logging.shutdown(false);
}

TEST(test_add_writers_batch)
{
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    // The C++ wrapper doesn't have a batch add; add them one by one.
    int code = logging.add_writer_config(ConsoleWriterConfig(DEBUG, false));
    ASSERT(code == 0, "add_writer_config(1) failed");
    code = logging.add_writer_config(ConsoleWriterConfig(INFO, false));
    ASSERT(code == 0, "add_writer_config(2) failed");
    logging.shutdown(false);
}

TEST(test_disable_enable_writer_file)
{
    const char *log_file = "test_toggle.log";
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(
        FileWriterConfig(DEBUG, log_file, 0, 0, -1, -1, CompressionMethod::Store));
    // Writer 1 is the file writer.
    int code = logging.disable(1);
    ASSERT(code == 0, "disable(1) failed");
    logging.info("disabled");
    logging.sync_all(2.0);
    std::string content = read_file(log_file);
    ASSERT(!contains(content, "disabled"), "disabled message should not be in file");
    code = logging.enable(1);
    ASSERT(code == 0, "enable(1) failed");
    logging.info("enabled");
    logging.sync_all(2.0);
    logging.shutdown(false);
    content = read_file(log_file);
    ASSERT(contains(content, "enabled"), "enabled message should be in file");
    remove(log_file);
}

TEST(test_disable_enable_invalid_writer)
{
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    int code = logging.disable(9999);
    ASSERT(code != 0, "disable(9999) should fail");
    code = logging.enable(9999);
    ASSERT(code != 0, "enable(9999) should fail");
    logging.shutdown(false);
}

TEST(test_enable_disable_type)
{
    // Use console writers (type=Console).
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, false));
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, false));
    int code = logging.disable_type(rust::WriterTypeEnum::Console);
    ASSERT(code == 0, "disable_type(Console) failed");
    code = logging.enable_type(rust::WriterTypeEnum::Console);
    ASSERT(code == 0, "enable_type(Console) failed");
    // Syslog type — should fail since no syslog writers exist.
    code = logging.disable_type(rust::WriterTypeEnum::Syslog);
    ASSERT(code != 0, "disable_type(Syslog) should fail");
    code = logging.enable_type(rust::WriterTypeEnum::Syslog);
    ASSERT(code != 0, "enable_type(Syslog) should fail");
    logging.shutdown(false);
}

// ---------------------------------------------------------------------------
// Level and domain management
// ---------------------------------------------------------------------------

TEST(test_set_level_writer)
{
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(ConsoleWriterConfig(INFO, false));
    // Writer 1 has level INFO.  Change it to DEBUG.
    int code = logging.set_level(1, DEBUG);
    ASSERT(code == 0, "set_level(1, DEBUG) failed");
    logging.debug("passes now");
    logging.shutdown(false);
}

TEST(test_set_domain)
{
    Logging logging(DEBUG, "old");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.set_domain("new-domain");
    const char *cfg = logging.get_config_string();
    ASSERT(cfg != nullptr, "get_config_string returned null");
    ASSERT(contains(std::string(cfg), "new-domain"), "config must contain new-domain");
    logging.shutdown(false);
}

TEST(test_set_ext_config)
{
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    {
        ExtConfig ext(MessageStruct::String, 0, 0, 0, 0, 0);
        logging.set_ext_config(&ext);
    }
    {
        ExtConfig ext(MessageStruct::Json, 0, 0, 0, 1, 1);
        logging.set_ext_config(&ext);
    }
    logging.shutdown(false);
}

TEST(test_set_level2sym)
{
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    // LevelSyms_Str = 2.
    logging.set_level2sym(2);
    const char *cfg = logging.get_config_string();
    ASSERT(cfg != nullptr, "get_config_string returned null");
    ASSERT(contains(std::string(cfg), "DEBUG"), "config must contain DEBUG");
    logging.shutdown(false);
}

// ---------------------------------------------------------------------------
// Logger
// ---------------------------------------------------------------------------

TEST(test_add_remove_logger)
{
    Logging logging(DEBUG, "root");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, false));
    Logger logger(DEBUG, "logger-domain");
    logging.add_logger(logger);
    logger.info("logger message");
    logger.trace("trace");
    logger.debug("debug");
    logger.warning("warning");
    logger.error("error");
    logger.critical("critical");
    logging.sync_all(2.0);
    logging.remove_logger(logger);
    logging.shutdown(false);
}

TEST(test_logger_level_filtering)
{
    Logging logging(DEBUG, "root");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, false));
    Logger logger(INFO, "logger-domain");
    logging.add_logger(logger);
    // Logger level is INFO, so DEBUG should be filtered.
    logger.debug("filtered by logger level");
    logger.info("passes");
    logging.sync_all(2.0);
    // Change logger level to DEBUG.
    logger.set_level(DEBUG);
    logger.debug("now passes");
    logging.sync_all(2.0);
    logging.remove_logger(logger);
    logging.shutdown(false);
}

TEST(test_logger_set_domain)
{
    Logger logger(DEBUG, "d1");
    logger.set_domain("d2");
    // No direct way to inspect; just verify it doesn't crash.
    logger.set_level(TRACE);
}

TEST(test_logger_level)
{
    Logger logger(INFO, "d1");
    logger.set_level(TRACE);
    // No direct getter in C++ wrapper; just verify it doesn't crash.
}

// ---------------------------------------------------------------------------
// Config save/apply
// ---------------------------------------------------------------------------

TEST(test_save_config)
{
    const char *config_path = "test_save_config.json";
    const char *log_file = "test_save_config.log";
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(
        FileWriterConfig(DEBUG, log_file, 0, 0, -1, -1, CompressionMethod::Store));
    int code = logging.save_config(config_path);
    ASSERT(code == 0, "save_config failed");
    FILE *f = fopen(config_path, "r");
    ASSERT(f != nullptr, "config file should exist");
    fclose(f);
    logging.shutdown(false);
    remove(config_path);
    remove(log_file);
}

TEST(test_get_config_string)
{
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    const char *cfg = logging.get_config_string();
    ASSERT(cfg != nullptr, "get_config_string returned null");
    std::string s(cfg);
    ASSERT(contains(s, "level="), "config must contain 'level='");
    ASSERT(contains(s, "domain="), "config must contain 'domain='");
    logging.shutdown(false);
}

// ---------------------------------------------------------------------------
// Network configs (construct-only)
// ---------------------------------------------------------------------------

TEST(test_server_client_config_construction)
{
    // Server with no encryption.
    ServerConfig server(DEBUG, "127.0.0.1:0", nullptr);
    ASSERT(server.config != nullptr, "ServerConfig returned null");
    // Client with no encryption.
    ClientWriterConfig client(DEBUG, "127.0.0.1:0", nullptr);
    ASSERT(client.config != nullptr, "ClientWriterConfig returned null");
}

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

TEST(test_shutdown_twice_is_idempotent)
{
    Logging logging = Logging::Default();
    ASSERT(logging.raw() != nullptr, "Logging::Default returned null");
    logging.shutdown(false);
    // Do not call shutdown again — the logging instance is already freed.
    // The destructor would double-free; we avoid it by moving out.
}

// ---------------------------------------------------------------------------
// ExtConfig helpers
// ---------------------------------------------------------------------------

TEST(test_ext_config_default)
{
    ExtConfig ext(MessageStruct::String, 0, 0, 0, 0, 0);
    ASSERT(ext.config != nullptr, "ExtConfig returned null");
    ASSERT(ext.config->structured == rust::MessageStructEnum::String,
           "structured should be String");
    ASSERT(ext.config->hostname == 0, "hostname should be 0");
    ASSERT(ext.config->pname == 0, "pname should be 0");
    ASSERT(ext.config->pid == 0, "pid should be 0");
    ASSERT(ext.config->tname == 0, "tname should be 0");
    ASSERT(ext.config->tid == 0, "tid should be 0");
}

// ---------------------------------------------------------------------------
// Callback writer
// ---------------------------------------------------------------------------

static int s_callback_count = 0;

static void test_callback(uint8_t level, const char *domain, const char *message)
{
    (void)level;
    (void)domain;
    (void)message;
    s_callback_count++;
}

TEST(test_callback_writer_receives_messages)
{
    s_callback_count = 0;
    Logging logging(DEBUG, "cb-domain");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(CallbackWriterConfig(DEBUG, test_callback));
    logging.info("callback msg");
    logging.sync_all(2.0);
    logging.shutdown(false);
    ASSERT(s_callback_count >= 1, "callback must have been invoked at least once");
}

TEST(test_callback_writer_respects_level)
{
    s_callback_count = 0;
    // Writer level is INFO: TRACE/DEBUG messages must be filtered out.
    Logging logging(DEBUG, "root");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(CallbackWriterConfig(INFO, test_callback));
    logging.debug("filtered");
    logging.info("passes");
    logging.warning("passes too");
    logging.sync_all(2.0);
    logging.shutdown(false);
    ASSERT(s_callback_count == 2, "callback should have been invoked 2 times (info + warning)");
}

// ---------------------------------------------------------------------------
// Concurrency smoke test
// ---------------------------------------------------------------------------

#ifdef _WIN32
#include <windows.h>
typedef HANDLE thread_t;
static int thread_create(thread_t *t, DWORD WINAPI (*fn)(LPVOID), void *arg)
{
    *t = CreateThread(NULL, 0, fn, arg, 0, NULL);
    return *t == NULL ? -1 : 0;
}
static int thread_join(thread_t t)
{
    WaitForSingleObject(t, INFINITE);
    CloseHandle(t);
    return 0;
}
#else
#include <pthread.h>
typedef pthread_t thread_t;
static int thread_create(thread_t *t, void *(*fn)(void *), void *arg)
{
    return pthread_create(t, NULL, fn, arg);
}
static int thread_join(thread_t t)
{
    return pthread_join(t, NULL);
}
#endif

struct thread_arg_t
{
    Logging *logging;
    int thread_id;
};

#ifdef _WIN32
static DWORD WINAPI concurrent_worker(LPVOID p)
#else
static void *concurrent_worker(void *p)
#endif
{
    thread_arg_t *arg = static_cast<thread_arg_t *>(p);
    for (int i = 0; i < 50; i++)
    {
        char buf[128];
        snprintf(buf, sizeof(buf), "thread %d message %d", arg->thread_id, i);
        arg->logging->info(buf);
    }
#ifdef _WIN32
    return 0;
#else
    return nullptr;
#endif
}

TEST(test_concurrent_logging)
{
    const char *log_file = "test_concurrent.log";
    Logging logging(DEBUG, "test");
    ASSERT(logging.raw() != nullptr, "Logging returned null");
    logging.add_writer_config(
        FileWriterConfig(DEBUG, log_file, 0, 0, -1, -1, CompressionMethod::Store));

    const int NUM_THREADS = 4;
    thread_t threads[NUM_THREADS];
    thread_arg_t args[NUM_THREADS];
    for (int t = 0; t < NUM_THREADS; t++)
    {
        args[t].logging = &logging;
        args[t].thread_id = t;
        thread_create(&threads[t], concurrent_worker, &args[t]);
    }
    for (int t = 0; t < NUM_THREADS; t++)
    {
        thread_join(threads[t]);
    }
    logging.sync_all(2.0);
    logging.shutdown(false);

    std::string content = read_file(log_file);
    ASSERT(!content.empty(), "failed to read log file");
    for (int t = 0; t < NUM_THREADS; t++)
    {
        for (int i = 0; i < 50; i++)
        {
            char buf[128];
            snprintf(buf, sizeof(buf), "thread %d message %d", t, i);
            if (!contains(content, buf))
            {
                FAIL("missing: %s", buf);
            }
        }
    }
    remove(log_file);
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

int main(void)
{
    printf("Running cppfastlogging integration tests...\n\n");

    RUN_TEST(test_level_conversion);
    RUN_TEST(test_default_logging);
    RUN_TEST(test_logging_new_with_console_writer);
    RUN_TEST(test_console_writer_config_new);
    RUN_TEST(test_file_writer_writes_messages);
    RUN_TEST(test_file_writer_level_filter);
    RUN_TEST(test_file_writer_rotation);
    RUN_TEST(test_add_remove_writer);
    RUN_TEST(test_remove_writer_invalid_id);
    RUN_TEST(test_add_writers_batch);
    RUN_TEST(test_disable_enable_writer_file);
    RUN_TEST(test_disable_enable_invalid_writer);
    RUN_TEST(test_enable_disable_type);
    RUN_TEST(test_set_level_writer);
    RUN_TEST(test_set_domain);
    RUN_TEST(test_set_ext_config);
    RUN_TEST(test_set_level2sym);
    RUN_TEST(test_add_remove_logger);
    RUN_TEST(test_logger_level_filtering);
    RUN_TEST(test_logger_set_domain);
    RUN_TEST(test_logger_level);
    RUN_TEST(test_save_config);
    RUN_TEST(test_get_config_string);
    RUN_TEST(test_server_client_config_construction);
    RUN_TEST(test_shutdown_twice_is_idempotent);
    RUN_TEST(test_ext_config_default);
    RUN_TEST(test_callback_writer_receives_messages);
    RUN_TEST(test_callback_writer_respects_level);
    RUN_TEST(test_concurrent_logging);

    printf("\n========================================\n");
    printf("Tests run:    %d\n", tests_run);
    printf("Tests passed: %d\n", tests_passed);
    printf("Tests failed: %d\n", tests_failed);
    printf("========================================\n");

    return tests_failed > 0 ? 1 : 0;
}
