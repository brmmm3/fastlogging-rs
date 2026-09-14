/*
 * benchmark.cpp — Benchmark program comparing cppfastlogging, cxxfastlogging
 *                 and loguru.
 *
 * This benchmark mirrors the structure of pyfastlogging/benches/benchmark.py:
 *   - Short and long messages
 *   - No file, plain file, rotating file
 *   - Multiple log levels (DEBUG, INFO, WARNING, ERROR, CRITICAL)
 *
 * Build:
 *   make build
 * Run:
 *   make run
 *   (or directly: ./bin/benchmark [count])
 *
 * Results are written to doc/benchmarks/ (JSON and HTML)
 *
 * NOTE: cxxfastlogging benchmark functions are in benchmark_cxx.cpp to avoid
 * type name conflicts between cppfastlogging and cxxfastlogging.
 */

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <chrono>

#ifdef _WIN32
#include <windows.h>
#include <direct.h>
#include <sys/stat.h>
#undef ERROR
#define MKDIR(path) _mkdir(path)
#define PATH_SEP '\\'
#else
#include <unistd.h>
#include <sys/stat.h>
#define MKDIR(path) mkdir(path, 0755)
#define PATH_SEP '/'
#endif

/* ------------------------------------------------------------------ */
/* cppfastlogging (C++ wrapper around cfastlogging)                    */
/* ------------------------------------------------------------------ */
#include "h/cppfastlogging.hpp"
using namespace logging;

/* ------------------------------------------------------------------ */
/* loguru (header-only C++ logging library)                           */
/* ------------------------------------------------------------------ */
#define LOGURU_WITH_FILEABS 1
#include "loguru_src/loguru.hpp"
#include "loguru_src/loguru.cpp"

/* ------------------------------------------------------------------ */
/* cxxfastlogging benchmark functions (implemented in benchmark_cxx.cpp) */
/* ------------------------------------------------------------------ */
extern "C"
{
    double bench_cxx_no_file(int cnt, uint8_t level, const char *message);
    double bench_cxx_file(int cnt, uint8_t level, const char *path,
                          const char *message, int rotate);
}

/* ------------------------------------------------------------------ */
/* Constants                                                          */
/* ------------------------------------------------------------------ */

#define MB (1024 * 1024)
#define CNT 5000
#define NUM_ROUNDS 10
#define MAX_PATH_LEN 1024

/* Temp directory */
#ifdef _WIN32
static const char *TMP_DIR = "C:\\temp\\cppfastlogging_bench";
#else
static const char *TMP_DIR = "/tmp/cppfastlogging_bench";
#endif

/* ------------------------------------------------------------------ */
/* Utility helpers                                                    */
/* ------------------------------------------------------------------ */

static double now_sec(void)
{
    auto t = std::chrono::high_resolution_clock::now();
    auto dur = t.time_since_epoch();
    return std::chrono::duration<double>(dur).count();
}

static void ensure_dir(const char *path)
{
    struct stat st;
    if (stat(path, &st) == 0)
    {
#ifdef _WIN32
        char cmd[MAX_PATH_LEN + 32];
        snprintf(cmd, sizeof(cmd), "rmdir /s /q \"%s\"", path);
        system(cmd);
#else
        char cmd[MAX_PATH_LEN + 32];
        snprintf(cmd, sizeof(cmd), "rm -rf \"%s\"", path);
        system(cmd);
#endif
    }
    MKDIR(path);
}

static void get_path(const char *title, const char *filename, char *out, size_t outsz)
{
    char dir[MAX_PATH_LEN];
    snprintf(dir, sizeof(dir), "%s%c%s", TMP_DIR, PATH_SEP, title);
    ensure_dir(dir);
    snprintf(out, outsz, "%s%c%s", dir, PATH_SEP, filename);
}

static void cleanup_dir(const char *title)
{
    char dir[MAX_PATH_LEN];
    snprintf(dir, sizeof(dir), "%s%c%s", TMP_DIR, PATH_SEP, title);
#ifdef _WIN32
    char cmd[MAX_PATH_LEN + 32];
    snprintf(cmd, sizeof(cmd), "rmdir /s /q \"%s\"", dir);
    system(cmd);
#else
    char cmd[MAX_PATH_LEN + 32];
    snprintf(cmd, sizeof(cmd), "rm -rf \"%s\"", dir);
    system(cmd);
#endif
}

/* ------------------------------------------------------------------ */
/* Level mapping                                                      */
/* ------------------------------------------------------------------ */

/*
 * cppfastlogging / cxxfastlogging levels:
 *   DEBUG=10, INFO=20, WARNING=30, ERROR=40, CRITICAL=50
 *
 * loguru levels (NamedVerbosity):
 *   FATAL=-3, ERROR=-2, WARNING=-1, INFO=0
 *   For DEBUG, loguru uses verbosity 1-9; we use 1 for DEBUG.
 *   loguru has no CRITICAL; we map CRITICAL to FATAL.
 */

static const char *level_names[] = {"DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"};
static const uint8_t cfl_levels[] = {DEBUG, INFO, WARNING, ERROR, CRITICAL};

/* loguru verbosity for each level: DEBUG=1, INFO=0, WARNING=-1, ERROR=-2, CRITICAL=-2
 * NOTE: loguru has no CRITICAL level. FATAL (-3) calls abort(), so we use
 * ERROR (-2) for CRITICAL to avoid terminating the benchmark. */
static const int loguru_verbosities[] = {1, 0, -1, -2, -2};

/* ------------------------------------------------------------------ */
/* Logging work functions                                             */
/* Each iteration logs 20 messages (5 levels x 4 rounds)              */
/* ------------------------------------------------------------------ */

static double logging_work_cpp(Logging &logging, int cnt, const char *message)
{
    char buf[512];
    double t1 = now_sec();
    for (int i = 0; i < cnt; i++)
    {
        for (int round = 0; round < 4; round++)
        {
            snprintf(buf, sizeof(buf), "Critical %d %s", i, message);
            logging.critical(buf);
            snprintf(buf, sizeof(buf), "Error %d %s", i, message);
            logging.error(buf);
            snprintf(buf, sizeof(buf), "Warning %s %d", message, i);
            logging.warning(buf);
            snprintf(buf, sizeof(buf), "Info %s %d", message, i);
            logging.info(buf);
            snprintf(buf, sizeof(buf), "Debug %s %d", message, i);
            logging.debug(buf);
        }
    }
    return now_sec() - t1;
}

static double logging_work_loguru(int cnt, const char *message)
{
    char buf[512];
    double t1 = now_sec();
    for (int i = 0; i < cnt; i++)
    {
        for (int round = 0; round < 4; round++)
        {
            snprintf(buf, sizeof(buf), "Critical %d %s", i, message);
            LOG_F(ERROR, "%s", buf);
            snprintf(buf, sizeof(buf), "Error %d %s", i, message);
            LOG_F(ERROR, "%s", buf);
            snprintf(buf, sizeof(buf), "Warning %s %d", message, i);
            LOG_F(WARNING, "%s", buf);
            snprintf(buf, sizeof(buf), "Info %s %d", message, i);
            LOG_F(INFO, "%s", buf);
            snprintf(buf, sizeof(buf), "Debug %s %d", message, i);
            VLOG_F(1, "%s", buf);
        }
    }
    return now_sec() - t1;
}

/* ------------------------------------------------------------------ */
/* cppfastlogging benchmark functions                                  */
/* ------------------------------------------------------------------ */

static double bench_cpp_no_file(int cnt, uint8_t level, const char *message)
{
    Logging logging(level, "bench");
    double dt = logging_work_cpp(logging, cnt, message);
    logging.shutdown(false);
    return dt;
}

static double bench_cpp_file(int cnt, uint8_t level, const char *path,
                             const char *message, int rotate)
{
    uint32_t size = rotate ? MB : 0;
    uint32_t backlog = rotate ? 8 : 0;
    Logging logging(level, "bench");
    logging.add_writer_config(
        FileWriterConfig(level, path, size, backlog));
    double dt = logging_work_cpp(logging, cnt, message);
    logging.sync_all(10.0);
    logging.shutdown(false);
    return dt;
}

/* ------------------------------------------------------------------ */
/* loguru benchmark functions                                          */
/* ------------------------------------------------------------------ */

static double bench_loguru_no_file(int cnt, int verbosity, const char *message)
{
    loguru::g_stderr_verbosity = loguru::Verbosity_OFF;
    double dt = logging_work_loguru(cnt, message);
    return dt;
}

static double bench_loguru_file(int cnt, int verbosity, const char *path,
                                const char *message, int rotate)
{
    loguru::g_stderr_verbosity = loguru::Verbosity_OFF;
    loguru::add_file(path, loguru::Truncate, loguru::Verbosity_MAX);
    double dt = logging_work_loguru(cnt, message);
    loguru::remove_callback(path);
    return dt;
}

/* ------------------------------------------------------------------ */
/* Measurement                                                        */
/* ------------------------------------------------------------------ */

typedef struct
{
    double cpp;
    double cxx;
    double loguru;
} level_result_t;

typedef struct
{
    level_result_t levels[5];
} scenario_result_t;

/* ------------------------------------------------------------------ */
/* Main                                                               */
/* ------------------------------------------------------------------ */

int main(int argc, char *argv[])
{
    int cnt = CNT;
    if (argc > 1)
    {
        cnt = atoi(argv[1]);
    }

#ifdef _WIN32
    WSADATA wsaData;
    WSAStartup(MAKEWORD(2, 2), &wsaData);
#endif

    /* Initialize loguru (required before any LOG_F calls) */
    loguru::g_stderr_verbosity = loguru::Verbosity_OFF;
    {
        char dummy_arg0[] = "benchmark";
        char *dummy_argv[] = {dummy_arg0, nullptr};
        int dummy_argc = 1;
        loguru::init(dummy_argc, dummy_argv);
    }

    printf("cnt: %d\n", cnt);
    ensure_dir(TMP_DIR);

    const char *msg_keys[] = {"short", "long"};
    const char *messages[] = {
        "Message",
        "Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message"};

    const char *scenario_names[] = {"nolog", "file", "rotate"};
    const char *scenario_titles[] = {"No log file", "Log file", "Rotating log file"};
    int rotate_flags[] = {0, 0, 1};
    int has_file[] = {0, 1, 1};

    /* Results storage: [msg_type][scenario][level] */
    scenario_result_t results[2][3];

    /* Run all benchmarks */
    for (int m = 0; m < 2; m++)
    {
        const char *msg_key = msg_keys[m];
        const char *message = messages[m];

        for (int s = 0; s < 3; s++)
        {
            for (int lv = 0; lv < 5; lv++)
            {
                uint8_t level = cfl_levels[lv];
                int verbosity = loguru_verbosities[lv];
                const char *lv_name = level_names[lv];

                printf("\n### %s %s %s\n", msg_key, scenario_names[s], lv_name);

                double dt_cpp, dt_cxx, dt_loguru;

                if (has_file[s])
                {
                    char title_cpp[MAX_PATH_LEN];
                    char title_cxx[MAX_PATH_LEN];
                    char title_loguru[MAX_PATH_LEN];
                    snprintf(title_cpp, sizeof(title_cpp), "cpp_%s_%s_%s",
                             msg_key, scenario_names[s], lv_name);
                    snprintf(title_cxx, sizeof(title_cxx), "cxx_%s_%s_%s",
                             msg_key, scenario_names[s], lv_name);
                    snprintf(title_loguru, sizeof(title_loguru), "loguru_%s_%s_%s",
                             msg_key, scenario_names[s], lv_name);

                    /* cppfastlogging */
                    {
                        double total = 0.0;
                        int rounds = 0;
                        for (int i = 0; i < NUM_ROUNDS; i++)
                        {
                            char path[MAX_PATH_LEN];
                            get_path(title_cpp, "logging.log", path, sizeof(path));
                            double t = bench_cpp_file(cnt, level, path, message, rotate_flags[s]);
                            if (t < 0)
                            {
                                total = -1.0;
                                break;
                            }
                            total += t;
                            rounds++;
                            if (total > 2.0)
                                break;
                        }
                        dt_cpp = rounds > 0 ? total / rounds : -1.0;
                    }
                    /* cxxfastlogging */
                    {
                        double total = 0.0;
                        int rounds = 0;
                        for (int i = 0; i < NUM_ROUNDS; i++)
                        {
                            char path[MAX_PATH_LEN];
                            get_path(title_cxx, "logging.log", path, sizeof(path));
                            double t = bench_cxx_file(cnt, level, path, message, rotate_flags[s]);
                            if (t < 0)
                            {
                                total = -1.0;
                                break;
                            }
                            total += t;
                            rounds++;
                            if (total > 2.0)
                                break;
                        }
                        dt_cxx = rounds > 0 ? total / rounds : -1.0;
                    }
                    /* loguru */
                    {
                        double total = 0.0;
                        int rounds = 0;
                        for (int i = 0; i < NUM_ROUNDS; i++)
                        {
                            char path[MAX_PATH_LEN];
                            get_path(title_loguru, "logging.log", path, sizeof(path));
                            double t = bench_loguru_file(cnt, verbosity, path, message, rotate_flags[s]);
                            if (t < 0)
                            {
                                total = -1.0;
                                break;
                            }
                            total += t;
                            rounds++;
                            if (total > 2.0)
                                break;
                        }
                        dt_loguru = rounds > 0 ? total / rounds : -1.0;
                    }

                    cleanup_dir(title_cpp);
                    cleanup_dir(title_cxx);
                    cleanup_dir(title_loguru);
                }
                else
                {
                    /* nolog: no file, just measure in-memory logging */
                    {
                        double total = 0.0;
                        int rounds = 0;
                        for (int i = 0; i < NUM_ROUNDS; i++)
                        {
                            double t = bench_cpp_no_file(cnt, level, message);
                            if (t < 0)
                            {
                                total = -1.0;
                                break;
                            }
                            total += t;
                            rounds++;
                            if (total > 2.0)
                                break;
                        }
                        dt_cpp = rounds > 0 ? total / rounds : -1.0;
                    }
                    {
                        double total = 0.0;
                        int rounds = 0;
                        for (int i = 0; i < NUM_ROUNDS; i++)
                        {
                            double t = bench_cxx_no_file(cnt, level, message);
                            if (t < 0)
                            {
                                total = -1.0;
                                break;
                            }
                            total += t;
                            rounds++;
                            if (total > 2.0)
                                break;
                        }
                        dt_cxx = rounds > 0 ? total / rounds : -1.0;
                    }
                    {
                        double total = 0.0;
                        int rounds = 0;
                        for (int i = 0; i < NUM_ROUNDS; i++)
                        {
                            double t = bench_loguru_no_file(cnt, verbosity, message);
                            if (t < 0)
                            {
                                total = -1.0;
                                break;
                            }
                            total += t;
                            rounds++;
                            if (total > 2.0)
                                break;
                        }
                        dt_loguru = rounds > 0 ? total / rounds : -1.0;
                    }
                }

                printf("  cppfastlogging: %.4f s\n", dt_cpp);
                printf("  cxxfastlogging: %.4f s\n", dt_cxx);
                printf("  loguru:          %.4f s\n", dt_loguru);

                results[m][s].levels[lv].cpp = dt_cpp;
                results[m][s].levels[lv].cxx = dt_cxx;
                results[m][s].levels[lv].loguru = dt_loguru;
            }
        }
    }

    /* Write JSON output */
    char json_path[MAX_PATH_LEN];
    snprintf(json_path, sizeof(json_path), "doc/benchmarks/cpp_benchmark.json");
    FILE *jf = fopen(json_path, "w");
    if (!jf)
    {
        fprintf(stderr, "Cannot open %s for writing\n", json_path);
        return 1;
    }

    fprintf(jf, "{\n");
    for (int m = 0; m < 2; m++)
    {
        fprintf(jf, "  \"%s\": {\n", msg_keys[m]);
        for (int s = 0; s < 3; s++)
        {
            fprintf(jf, "    \"%s\": {\n", scenario_names[s]);
            fprintf(jf, "      \"title\": \"%s\",\n", scenario_titles[s]);
            for (int lv = 0; lv < 5; lv++)
            {
                fprintf(jf, "      \"%s\": {\"cppfastlogging\": %.6f, \"cxxfastlogging\": %.6f, \"loguru\": %.6f}%s\n",
                        level_names[lv],
                        results[m][s].levels[lv].cpp,
                        results[m][s].levels[lv].cxx,
                        results[m][s].levels[lv].loguru,
                        lv < 4 ? "," : "");
            }
            fprintf(jf, "    }%s\n", s < 2 ? "," : "");
        }
        fprintf(jf, "  }%s\n", m < 1 ? "," : "");
    }
    fprintf(jf, "}\n");
    fclose(jf);
    printf("\nJSON results written to %s\n", json_path);

    /* Generate HTML files */
    char template_path[MAX_PATH_LEN];
    snprintf(template_path, sizeof(template_path), "doc/benchmarks/template.html");
    FILE *tf = fopen(template_path, "r");
    if (!tf)
    {
        fprintf(stderr, "Warning: Cannot open template %s, skipping HTML\n", template_path);
        loguru::shutdown();
        return 0;
    }
    fseek(tf, 0, SEEK_END);
    long tsz = ftell(tf);
    fseek(tf, 0, SEEK_SET);
    char *tmpl = (char *)malloc(tsz + 1);
    fread(tmpl, 1, tsz, tf);
    tmpl[tsz] = '\0';
    fclose(tf);

    /* Generate one HTML per (msg_type, scenario) */
    for (int m = 0; m < 2; m++)
    {
        for (int s = 0; s < 3; s++)
        {
            char html_path[MAX_PATH_LEN];
            snprintf(html_path, sizeof(html_path), "doc/benchmarks/%s_%s.html",
                     scenario_names[s], msg_keys[m]);

            char *content = strdup(tmpl);
            char *p;

            /* Replace TITLE */
            char title_buf[512];
            snprintf(title_buf, sizeof(title_buf), "%s — %s", scenario_titles[s], msg_keys[m]);
            p = strstr(content, "%(TITLE)s");
            if (p)
            {
                char tmp[16384];
                size_t before = p - content;
                snprintf(tmp, sizeof(tmp), "%.*s%s%s",
                         (int)before, content, title_buf, p + 8);
                free(content);
                content = strdup(tmp);
            }

            /* Replace each level placeholder */
            for (int lv = 0; lv < 5; lv++)
            {
                char placeholder[32];
                snprintf(placeholder, sizeof(placeholder), "%%(%s)s", level_names[lv]);
                char value[256];
                snprintf(value, sizeof(value), "%.4f, %.4f, %.4f",
                         results[m][s].levels[lv].cpp,
                         results[m][s].levels[lv].cxx,
                         results[m][s].levels[lv].loguru);

                p = strstr(content, placeholder);
                if (p)
                {
                    char tmp[32768];
                    size_t before = p - content;
                    snprintf(tmp, sizeof(tmp), "%.*s%s%s",
                             (int)before, content, value, p + strlen(placeholder));
                    free(content);
                    content = strdup(tmp);
                }
            }

            FILE *hf = fopen(html_path, "w");
            if (hf)
            {
                fputs(content, hf);
                fclose(hf);
            }
            free(content);
        }
    }

    free(tmpl);
    printf("HTML files written to doc/benchmarks/\n");

    loguru::shutdown();
    return 0;
}
