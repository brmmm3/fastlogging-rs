//! Integration tests for the `cfastlogging` C library.
//!
//! These tests mirror the Rust `fastlogging/tests/integration.rs` tests,
//! exercising the C FFI API end-to-end: creating a Logging instance with
//! different writers, logging messages, managing writers and loggers,
//! syncing, rotating files, config save/apply and level handling.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>

#include "h/cfastlogging.h"

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Read a file into a newly allocated buffer.  Caller must free.
/// Returns NULL on failure.
static char *read_file(const char *path)
{
    FILE *f = fopen(path, "rb");
    if (!f)
        return NULL;
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    fseek(f, 0, SEEK_SET);
    char *buf = (char *)malloc(sz + 1);
    if (!buf)
    {
        fclose(f);
        return NULL;
    }
    fread(buf, 1, sz, f);
    buf[sz] = '\0';
    fclose(f);
    return buf;
}

/// Check if a string contains a substring.
static int contains(const char *haystack, const char *needle)
{
    return haystack && needle && strstr(haystack, needle) != NULL;
}

/// Simple test framework.
static int tests_run = 0;
static int tests_passed = 0;
static int tests_failed = 0;

#define TEST(name)          \
    static void name(void); \
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
    // The C header defines these as preprocessor macros.
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
    // Aliases.
    ASSERT(WARN == WARNING, "WARN == WARNING");
    ASSERT(FATAL == CRITICAL, "FATAL == CRITICAL");
}

// ---------------------------------------------------------------------------
// Basic logging
// ---------------------------------------------------------------------------

TEST(test_default_logging)
{
    Logging logging = logging_new_default();
    ASSERT(logging != NULL, "logging_new_default returned NULL");
    logging_trace(logging, "trace");
    logging_debug(logging, "debug");
    logging_info(logging, "info");
    logging_warning(logging, "warning");
    logging_error(logging, "error");
    logging_critical(logging, "critical");
    logging_shutdown(logging, 0);
}

TEST(test_logging_new_with_console_writer)
{
    WriterConfigEnum writers[] = {
        console_writer_config_new(DEBUG, 0)};
    Logging logging = logging_new(DEBUG, "test", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    logging_trace(logging, "trace msg");
    logging_debug(logging, "debug msg");
    logging_info(logging, "info msg");
    logging_success(logging, "success msg");
    logging_warning(logging, "warning msg");
    logging_error(logging, "error msg");
    logging_critical(logging, "critical msg");
    logging_fatal(logging, "fatal msg");
    logging_shutdown(logging, 0);
}

TEST(test_console_writer_config_new)
{
    WriterConfigEnum cfg = console_writer_config_new(DEBUG, 0);
    ASSERT(cfg != NULL, "console_writer_config_new returned NULL");
    WriterConfigEnum writers[] = {cfg};
    Logging logging = logging_new(DEBUG, "test", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    logging_info(logging, "console test");
    logging_shutdown(logging, 0);
}

// ---------------------------------------------------------------------------
// File writer
// ---------------------------------------------------------------------------

TEST(test_file_writer_writes_messages)
{
    const char *log_file = "test_file_writer.log";
    CCompressionMethodEnum compression = CompressionMethodEnum_Store;
    WriterConfigEnum writers[] = {
        file_writer_config_new(DEBUG, log_file, 0, 0, -1, -1, &compression)};
    Logging logging = logging_new(DEBUG, "test", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    logging_info(logging, "file info msg");
    logging_error(logging, "file error msg");
    logging_sync_all(logging, 2.0);
    logging_shutdown(logging, 0);
    char *content = read_file(log_file);
    ASSERT(content != NULL, "failed to read log file");
    ASSERT(contains(content, "file info msg"), "log file must contain info msg");
    ASSERT(contains(content, "file error msg"), "log file must contain error msg");
    free(content);
    remove(log_file);
}

TEST(test_file_writer_level_filter)
{
    const char *log_file = "test_file_level.log";
    CCompressionMethodEnum compression = CompressionMethodEnum_Store;
    WriterConfigEnum writers[] = {
        file_writer_config_new(WARNING, log_file, 0, 0, -1, -1, &compression)};
    Logging logging = logging_new(DEBUG, "test", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    logging_debug(logging, "debug message");
    logging_info(logging, "info message");
    logging_error(logging, "error message");
    logging_sync_all(logging, 2.0);
    logging_shutdown(logging, 0);
    char *content = read_file(log_file);
    ASSERT(content != NULL, "failed to read log file");
    ASSERT(!contains(content, "debug message"), "debug message should be filtered out");
    ASSERT(!contains(content, "info message"), "info message should be filtered out");
    ASSERT(contains(content, "error message"), "error message should be present");
    free(content);
    remove(log_file);
}

TEST(test_file_writer_rotation)
{
    const char *log_file = "test_rotate.log";
    CCompressionMethodEnum compression = CompressionMethodEnum_Store;
    // backlog=3 enables rotation.
    WriterConfigEnum writers[] = {
        file_writer_config_new(DEBUG, log_file, 0, 3, -1, -1, &compression)};
    Logging logging = logging_new(DEBUG, "test", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    for (int i = 0; i < 5; i++)
    {
        char buf[64];
        snprintf(buf, sizeof(buf), "message %d", i);
        logging_info(logging, buf);
    }
    logging_sync_all(logging, 2.0);
    logging_rotate(logging, NULL);
    logging_info(logging, "after rotate");
    logging_sync_all(logging, 2.0);
    logging_shutdown(logging, 0);
    char *current = read_file(log_file);
    ASSERT(current != NULL, "failed to read log file");
    ASSERT(contains(current, "after rotate"), "current file must contain 'after rotate'");
    free(current);
    remove(log_file);
}

// ---------------------------------------------------------------------------
// Writer management
// ---------------------------------------------------------------------------

TEST(test_add_remove_writer)
{
    Logging logging = logging_new(DEBUG, "test", NULL, 0, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    WriterConfigEnum cfg = console_writer_config_new(DEBUG, 0);
    ASSERT(cfg != NULL, "console_writer_config_new returned NULL");
    int code = (int)logging_add_writer_config(logging, cfg);
    ASSERT(code == 0, "add_writer_config failed");
    // Add a second writer.
    WriterConfigEnum cfg2 = console_writer_config_new(INFO, 0);
    code = (int)logging_add_writer_config(logging, cfg2);
    ASSERT(code == 0, "add_writer_config(2) failed");
    // Remove writer 1.
    code = (int)logging_remove_writer(logging, 1);
    ASSERT(code == 0, "remove_writer(1) failed");
    // Remove writer 2.
    code = (int)logging_remove_writer(logging, 2);
    ASSERT(code == 0, "remove_writer(2) failed");
    logging_shutdown(logging, 0);
}

TEST(test_remove_writer_invalid_id)
{
    Logging logging = logging_new(DEBUG, "test", NULL, 0, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    int code = (int)logging_remove_writer(logging, 12345);
    ASSERT(code != 0, "remove_writer(12345) should fail");
    logging_shutdown(logging, 0);
}

TEST(test_add_writers_batch)
{
    Logging logging = logging_new(DEBUG, "test", NULL, 0, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    WriterConfigEnum cfgs[2] = {
        console_writer_config_new(DEBUG, 0),
        console_writer_config_new(INFO, 0)};
    WriterConfigEnums wce;
    wce.cnt = 2;
    wce.values = cfgs;
    int code = (int)logging_add_writer_configs(logging, &wce);
    ASSERT(code == 0, "add_writer_configs failed");
    logging_shutdown(logging, 0);
}

TEST(test_disable_enable_writer_file)
{
    const char *log_file = "test_toggle.log";
    CCompressionMethodEnum compression = CompressionMethodEnum_Store;
    WriterConfigEnum writers[] = {
        file_writer_config_new(DEBUG, log_file, 0, 0, -1, -1, &compression)};
    Logging logging = logging_new(DEBUG, "test", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    // Writer 1 is the file writer.
    int code = (int)logging_disable(logging, 1);
    ASSERT(code == 0, "disable(1) failed");
    logging_info(logging, "disabled");
    logging_sync_all(logging, 2.0);
    char *content = read_file(log_file);
    ASSERT(content != NULL, "failed to read log file");
    ASSERT(!contains(content, "disabled"), "disabled message should not be in file");
    free(content);
    code = (int)logging_enable(logging, 1);
    ASSERT(code == 0, "enable(1) failed");
    logging_info(logging, "enabled");
    logging_sync_all(logging, 2.0);
    logging_shutdown(logging, 0);
    content = read_file(log_file);
    ASSERT(content != NULL, "failed to read log file");
    ASSERT(contains(content, "enabled"), "enabled message should be in file");
    free(content);
    remove(log_file);
}

TEST(test_disable_enable_invalid_writer)
{
    Logging logging = logging_new(DEBUG, "test", NULL, 0, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    int code = (int)logging_disable(logging, 9999);
    ASSERT(code != 0, "disable(9999) should fail");
    code = (int)logging_enable(logging, 9999);
    ASSERT(code != 0, "enable(9999) should fail");
    logging_shutdown(logging, 0);
}

TEST(test_enable_disable_type)
{
    // Use console writers (type=1).
    WriterConfigEnum writers[] = {
        console_writer_config_new(DEBUG, 0),
        console_writer_config_new(DEBUG, 0)};
    Logging logging = logging_new(DEBUG, "test", writers, 2, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    // Console type = 1.
    int code = (int)logging_disable_type(logging, 1);
    ASSERT(code == 0, "disable_type(Console) failed");
    code = (int)logging_enable_type(logging, 1);
    ASSERT(code == 0, "enable_type(Console) failed");
    // Syslog type = 8 — should fail since no syslog writers exist.
    code = (int)logging_disable_type(logging, 8);
    ASSERT(code != 0, "disable_type(Syslog) should fail");
    code = (int)logging_enable_type(logging, 8);
    ASSERT(code != 0, "enable_type(Syslog) should fail");
    logging_shutdown(logging, 0);
}

// ---------------------------------------------------------------------------
// Level and domain management
// ---------------------------------------------------------------------------

TEST(test_set_level_writer)
{
    WriterConfigEnum writers[] = {
        console_writer_config_new(INFO, 0)};
    Logging logging = logging_new(DEBUG, "test", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    // Writer 1 has level INFO.  Change it to DEBUG.
    int code = (int)logging_set_level(logging, 1, DEBUG);
    ASSERT(code == 0, "set_level(1, DEBUG) failed");
    logging_debug(logging, "passes now");
    logging_shutdown(logging, 0);
}

TEST(test_set_domain)
{
    Logging logging = logging_new(DEBUG, "old", NULL, 0, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    logging_set_domain(logging, "new-domain");
    const char *cfg = logging_get_config_string(logging);
    ASSERT(contains(cfg, "new-domain"), "config must contain new-domain");
    logging_shutdown(logging, 0);
}

TEST(test_set_ext_config)
{
    Logging logging = logging_new(DEBUG, "test", NULL, 0, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    ExtConfig *ext = ext_config_new(MessageStructEnum_String, 0, 0, 0, 0, 0);
    ASSERT(ext != NULL, "ext_config_new returned NULL");
    logging_set_ext_config(logging, ext);
    ExtConfig *ext2 = ext_config_new(MessageStructEnum_Json, 0, 0, 0, 1, 1);
    ASSERT(ext2 != NULL, "ext_config_new(2) returned NULL");
    logging_set_ext_config(logging, ext2);
    logging_shutdown(logging, 0);
}

TEST(test_set_level2sym)
{
    Logging logging = logging_new(DEBUG, "test", NULL, 0, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    // LevelSyms_Str = 2.
    logging_set_level2sym(logging, 2);
    const char *cfg = logging_get_config_string(logging);
    ASSERT(contains(cfg, "DEBUG"), "config must contain DEBUG");
    logging_shutdown(logging, 0);
}

// ---------------------------------------------------------------------------
// Logger
// ---------------------------------------------------------------------------

TEST(test_add_remove_logger)
{
    WriterConfigEnum writers[] = {
        console_writer_config_new(DEBUG, 0)};
    Logging logging = logging_new(DEBUG, "root", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    Logger logger = logger_new(DEBUG, "logger-domain");
    ASSERT(logger != NULL, "logger_new returned NULL");
    logging_add_logger(logging, logger);
    logger_info(logger, "logger message");
    logger_trace(logger, "trace");
    logger_debug(logger, "debug");
    logger_warning(logger, "warning");
    logger_error(logger, "error");
    logger_critical(logger, "critical");
    logging_sync_all(logging, 2.0);
    logging_remove_logger(logging, logger);
    logging_shutdown(logging, 0);
}

TEST(test_logger_level_filtering)
{
    WriterConfigEnum writers[] = {
        console_writer_config_new(DEBUG, 0)};
    Logging logging = logging_new(DEBUG, "root", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    Logger logger = logger_new(INFO, "logger-domain");
    ASSERT(logger != NULL, "logger_new returned NULL");
    logging_add_logger(logging, logger);
    // Logger level is INFO, so DEBUG should be filtered.
    logger_debug(logger, "filtered by logger level");
    logger_info(logger, "passes");
    logging_sync_all(logging, 2.0);
    // Change logger level to DEBUG.
    logger_set_level(logger, DEBUG);
    logger_debug(logger, "now passes");
    logging_sync_all(logging, 2.0);
    logging_remove_logger(logging, logger);
    logging_shutdown(logging, 0);
}

TEST(test_logger_set_domain)
{
    Logger logger = logger_new(DEBUG, "d1");
    ASSERT(logger != NULL, "logger_new returned NULL");
    logger_set_domain(logger, "d2");
    // No direct way to inspect; just verify it doesn't crash.
    logger_set_level(logger, TRACE);
}

TEST(test_logger_level)
{
    Logger logger = logger_new(INFO, "d1");
    ASSERT(logger != NULL, "logger_new returned NULL");
    logger_set_level(logger, TRACE);
    // No direct getter in C API; just verify it doesn't crash.
}

// ---------------------------------------------------------------------------
// Config save/apply
// ---------------------------------------------------------------------------

TEST(test_save_config)
{
    const char *config_path = "test_save_config.json";
    const char *log_file = "test_save_config.log";
    CCompressionMethodEnum compression = CompressionMethodEnum_Store;
    WriterConfigEnum writers[] = {
        file_writer_config_new(DEBUG, log_file, 0, 0, -1, -1, &compression)};
    Logging logging = logging_new(DEBUG, "test", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    int code = (int)logging_save_config(logging, config_path);
    ASSERT(code == 0, "save_config failed");
    FILE *f = fopen(config_path, "r");
    ASSERT(f != NULL, "config file should exist");
    fclose(f);
    logging_shutdown(logging, 0);
    remove(config_path);
    remove(log_file);
}

TEST(test_get_config_string)
{
    Logging logging = logging_new(DEBUG, "test", NULL, 0, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    const char *cfg = logging_get_config_string(logging);
    ASSERT(cfg != NULL, "get_config_string returned NULL");
    ASSERT(contains(cfg, "level="), "config must contain 'level='");
    ASSERT(contains(cfg, "domain="), "config must contain 'domain='");
    logging_shutdown(logging, 0);
}

// ---------------------------------------------------------------------------
// Network configs (construct-only)
// ---------------------------------------------------------------------------

TEST(test_server_client_config_construction)
{
    // Server with no encryption.
    WriterConfigEnum server = server_config_new(DEBUG, "127.0.0.1:0", NULL);
    ASSERT(server != NULL, "server_config_new returned NULL");
    // Client with no encryption.
    WriterConfigEnum client = client_writer_config_new(DEBUG, "127.0.0.1:0", NULL);
    ASSERT(client != NULL, "client_writer_config_new returned NULL");
}

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

TEST(test_shutdown_twice_is_idempotent)
{
    Logging logging = logging_new_default();
    ASSERT(logging != NULL, "logging_new_default returned NULL");
    logging_shutdown(logging, 0);
    // Do not call shutdown again — the logging instance is already freed.
    // In C, calling it again would be use-after-free.
}

// ---------------------------------------------------------------------------
// ExtConfig helpers
// ---------------------------------------------------------------------------

TEST(test_ext_config_default)
{
    ExtConfig *ext = ext_config_new(MessageStructEnum_String, 0, 0, 0, 0, 0);
    ASSERT(ext != NULL, "ext_config_new returned NULL");
    ASSERT(ext->structured == MessageStructEnum_String, "structured should be String");
    ASSERT(ext->hostname == 0, "hostname should be 0");
    ASSERT(ext->pname == 0, "pname should be 0");
    ASSERT(ext->pid == 0, "pid should be 0");
    ASSERT(ext->tname == 0, "tname should be 0");
    ASSERT(ext->tid == 0, "tid should be 0");
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
    WriterConfigEnum writers[] = {
        callback_writer_config_new(DEBUG, test_callback)};
    Logging logging = logging_new(DEBUG, "cb-domain", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    logging_info(logging, "callback msg");
    logging_sync_all(logging, 2.0);
    logging_shutdown(logging, 0);
    ASSERT(s_callback_count >= 1, "callback must have been invoked at least once");
}

TEST(test_callback_writer_respects_level)
{
    s_callback_count = 0;
    // Writer level is INFO: TRACE/DEBUG messages must be filtered out.
    WriterConfigEnum writers[] = {
        callback_writer_config_new(INFO, test_callback)};
    Logging logging = logging_new(DEBUG, "root", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");
    logging_debug(logging, "filtered");
    logging_info(logging, "passes");
    logging_warning(logging, "passes too");
    logging_sync_all(logging, 2.0);
    logging_shutdown(logging, 0);
    ASSERT(s_callback_count == 2, "callback should have been invoked 2 times (info + warning)");
}

// ---------------------------------------------------------------------------
// Concurrency smoke test
// ---------------------------------------------------------------------------

#ifdef _WIN32
#include <windows.h>
typedef HANDLE thread_t;
static int thread_create(thread_t *t, void *(*fn)(void *), void *arg)
{
    *t = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)fn, arg, 0, NULL);
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

typedef struct
{
    Logging logging;
    int thread_id;
} thread_arg_t;

static void *concurrent_worker(void *p)
{
    thread_arg_t *arg = (thread_arg_t *)p;
    for (int i = 0; i < 50; i++)
    {
        char buf[128];
        snprintf(buf, sizeof(buf), "thread %d message %d", arg->thread_id, i);
        logging_info(arg->logging, buf);
    }
    return NULL;
}

TEST(test_concurrent_logging)
{
    const char *log_file = "test_concurrent.log";
    CCompressionMethodEnum compression = CompressionMethodEnum_Store;
    WriterConfigEnum writers[] = {
        file_writer_config_new(DEBUG, log_file, 0, 0, -1, -1, &compression)};
    Logging logging = logging_new(DEBUG, "test", writers, 1, NULL, NULL);
    ASSERT(logging != NULL, "logging_new returned NULL");

    enum
    {
        NUM_THREADS = 4
    };
    thread_t threads[NUM_THREADS];
    thread_arg_t args[NUM_THREADS];
    for (int t = 0; t < NUM_THREADS; t++)
    {
        args[t].logging = logging;
        args[t].thread_id = t;
        thread_create(&threads[t], concurrent_worker, &args[t]);
    }
    for (int t = 0; t < NUM_THREADS; t++)
    {
        thread_join(threads[t]);
    }
    logging_sync_all(logging, 2.0);
    logging_shutdown(logging, 0);

    char *content = read_file(log_file);
    ASSERT(content != NULL, "failed to read log file");
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
    free(content);
    remove(log_file);
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

int main(void)
{
    printf("Running cfastlogging integration tests...\n\n");

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
