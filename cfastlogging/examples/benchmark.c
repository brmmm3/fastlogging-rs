/*
 * benchmark.c — Benchmark program comparing cfastlogging with zlog.
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
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/stat.h>

#ifdef _WIN32
#include <windows.h>
#include <direct.h>
#undef ERROR
#define MKDIR(path) _mkdir(path)
#else
#include <unistd.h>
#define MKDIR(path) mkdir(path, 0755)
#endif

/* zlog header (built from source, see Makefile target 'zlog') */
#include "zlog.h"

/* cfastlogging header */
#include "h/cfastlogging.h"

/* ------------------------------------------------------------------ */
/* Constants                                                          */
/* ------------------------------------------------------------------ */

#define MB (1024 * 1024)
#define CNT 5000
#define NUM_ROUNDS 10
#define MAX_PATH_LEN 1024

/* Temp directory */
#ifdef _WIN32
static const char *TMP_DIR = "C:\\temp\\cfastlogging_bench";
#define PATH_SEP '\\'
#else
static const char *TMP_DIR = "/tmp/cfastlogging_bench";
#define PATH_SEP '/'
#endif

/* ------------------------------------------------------------------ */
/* Utility helpers                                                    */
/* ------------------------------------------------------------------ */

static double now_sec(void)
{
#ifdef _WIN32
    static LARGE_INTEGER freq = {0};
    if (freq.QuadPart == 0)
    {
        QueryPerformanceFrequency(&freq);
    }
    LARGE_INTEGER t;
    QueryPerformanceCounter(&t);
    return (double)t.QuadPart / (double)freq.QuadPart;
#else
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (double)ts.tv_sec + (double)ts.tv_nsec / 1e9;
#endif
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

static void write_text_file(const char *path, const char *content)
{
    FILE *f = fopen(path, "w");
    if (f)
    {
        fputs(content, f);
        fclose(f);
    }
}

/* ------------------------------------------------------------------ */
/* Logging work functions                                             */
/* Each iteration logs 20 messages (5 levels x 4 rounds)              */
/* ------------------------------------------------------------------ */

static double logging_work_cfl(Logging logging, int cnt, const char *message)
{
    char buf[512];
    double t1 = now_sec();
    for (int i = 0; i < cnt; i++)
    {
        for (int round = 0; round < 4; round++)
        {
            snprintf(buf, sizeof(buf), "Critical %d %s", i, message);
            logging_critical(logging, buf);
            snprintf(buf, sizeof(buf), "Error %d %s", i, message);
            logging_error(logging, buf);
            snprintf(buf, sizeof(buf), "Warning %s %d", message, i);
            logging_warning(logging, buf);
            snprintf(buf, sizeof(buf), "Info %s %d", message, i);
            logging_info(logging, buf);
            snprintf(buf, sizeof(buf), "Debug %s %d", message, i);
            logging_debug(logging, buf);
        }
    }
    return now_sec() - t1;
}

static double logging_work_zlog(zlog_category_t *cat, int cnt, const char *message)
{
    char buf[512];
    double t1 = now_sec();
    for (int i = 0; i < cnt; i++)
    {
        for (int round = 0; round < 4; round++)
        {
            snprintf(buf, sizeof(buf), "Critical %d %s", i, message);
            zlog_fatal(cat, "%s", buf);
            snprintf(buf, sizeof(buf), "Error %d %s", i, message);
            zlog_error(cat, "%s", buf);
            snprintf(buf, sizeof(buf), "Warning %s %d", message, i);
            zlog_warn(cat, "%s", buf);
            snprintf(buf, sizeof(buf), "Info %s %d", message, i);
            zlog_info(cat, "%s", buf);
            snprintf(buf, sizeof(buf), "Debug %s %d", message, i);
            zlog_debug(cat, "%s", buf);
        }
    }
    return now_sec() - t1;
}

/* ------------------------------------------------------------------ */
/* cfastlogging benchmark functions                                    */
/* ------------------------------------------------------------------ */

static double bench_cfastlogging_no_file(int cnt, uint8_t level, const char *message)
{
    Logging logging = logging_new(level, "bench", NULL, 0, NULL, NULL);
    if (!logging)
    {
        fprintf(stderr, "logging_new failed\n");
        return -1.0;
    }
    double dt = logging_work_cfl(logging, cnt, message);
    logging_shutdown(logging, 0);
    return dt;
}

static double bench_cfastlogging_file(int cnt, uint8_t level, const char *path,
                                      const char *message, int rotate)
{
    uint32_t size = rotate ? MB : 0;
    uint32_t backlog = rotate ? 8 : 0;
    CCompressionMethodEnum compression = CompressionMethodEnum_Store;
    WriterConfigEnum writer = file_writer_config_new(
        level, path, size, backlog, -1, -1, &compression);
    Logging logging = logging_new(level, "bench", &writer, 1, NULL, NULL);
    if (!logging)
    {
        fprintf(stderr, "logging_new failed\n");
        return -1.0;
    }
    double dt = logging_work_cfl(logging, cnt, message);
    logging_sync_all(logging, 10.0);
    logging_shutdown(logging, 0);
    return dt;
}

/* ------------------------------------------------------------------ */
/* zlog benchmark functions                                           */
/* ------------------------------------------------------------------ */

/*
 * zlog level mapping:
 *   cfastlogging DEBUG    -> zlog DEBUG  (20)
 *   cfastlogging INFO     -> zlog INFO   (40)
 *   cfastlogging WARNING  -> zlog WARN   (80)
 *   cfastlogging ERROR    -> zlog ERROR  (100)
 *   cfastlogging CRITICAL -> zlog FATAL (120)
 *
 * zlog uses a config string with [rules] section.
 * Format: category.level output
 * Levels: DEBUG, INFO, NOTICE, WARN, ERROR, FATAL
 * We use "*" to match all categories.
 */

static const char *zlog_level_str(uint8_t level)
{
    switch (level)
    {
    case DEBUG:
        return "DEBUG";
    case INFO:
        return "INFO";
    case WARNING:
        return "WARN";
    case ERROR:
        return "ERROR";
    case CRITICAL:
        return "FATAL";
    default:
        return "DEBUG";
    }
}

static int build_zlog_config_str(char *out, size_t outsz,
                                 uint8_t level, const char *log_path, int rotate)
{
    const char *lvl = zlog_level_str(level);

    if (log_path == NULL)
    {
        snprintf(out, outsz,
                 "[global]\n"
                 "strict init = true\n"
                 "[formats]\n"
                 "default = \"%%d %%c %%V %%m\"\n"
                 "[rules]\n"
                 "*.%s >stdout\n",
                 lvl);
    }
    else if (rotate)
    {
        snprintf(out, outsz,
                 "[global]\n"
                 "strict init = true\n"
                 "[formats]\n"
                 "default = \"%%d %%c %%V %%m\"\n"
                 "[rules]\n"
                 "*.%s \"%s\", 1MB * 8 ~ \"%s.#r\"\n",
                 lvl, log_path, log_path);
    }
    else
    {
        snprintf(out, outsz,
                 "[global]\n"
                 "strict init = true\n"
                 "[formats]\n"
                 "default = \"%%d %%c %%V %%m\"\n"
                 "[rules]\n"
                 "*.%s \"%s\"\n",
                 lvl, log_path);
    }
    return 0;
}

static double bench_zlog_no_file(int cnt, uint8_t level, const char *message)
{
    char config[2048];
    build_zlog_config_str(config, sizeof(config), level, NULL, 0);

    /* Write config to a temp file and use zlog_init */
    char conf_path[MAX_PATH_LEN];
    snprintf(conf_path, sizeof(conf_path), "%s%czlog.conf", TMP_DIR, PATH_SEP);
    write_text_file(conf_path, config);

    int rc = zlog_init(conf_path);
    if (rc)
    {
        fprintf(stderr, "zlog_init failed: %d\n", rc);
        return -1.0;
    }
    zlog_category_t *cat = zlog_get_category("my_cat");
    if (!cat)
    {
        fprintf(stderr, "zlog_get_category failed\n");
        zlog_fini();
        return -1.0;
    }

    double dt = logging_work_zlog(cat, cnt, message);

    zlog_fini();
    return dt;
}

static double bench_zlog_file(int cnt, uint8_t level, const char *path,
                              const char *message, int rotate)
{
    char config[2048];
    build_zlog_config_str(config, sizeof(config), level, path, rotate);

    /* Write config to a temp file and use zlog_init */
    char conf_path[MAX_PATH_LEN];
    snprintf(conf_path, sizeof(conf_path), "%s%czlog.conf", TMP_DIR, PATH_SEP);
    write_text_file(conf_path, config);

    int rc = zlog_init(conf_path);
    if (rc)
    {
        fprintf(stderr, "zlog_init failed: %d\n", rc);
        return -1.0;
    }
    zlog_category_t *cat = zlog_get_category("my_cat");
    if (!cat)
    {
        fprintf(stderr, "zlog_get_category failed\n");
        zlog_fini();
        return -1.0;
    }

    double dt = logging_work_zlog(cat, cnt, message);

    zlog_fini();
    return dt;
}

/* ------------------------------------------------------------------ */
/* Measurement                                                        */
/* ------------------------------------------------------------------ */

static const char *level_names[] = {"DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"};
static const uint8_t levels[] = {DEBUG, INFO, WARNING, ERROR, CRITICAL};

typedef struct
{
    double cfl;
    double zlog;
} level_result_t;

typedef struct
{
    level_result_t levels[5];
} scenario_result_t;

static double measure_no_file(double (*fn)(int, uint8_t, const char *),
                              int cnt, uint8_t level, const char *message)
{
    double total = 0.0;
    int rounds = 0;
    for (int i = 0; i < NUM_ROUNDS; i++)
    {
        double t = fn(cnt, level, message);
        if (t < 0)
            return -1.0;
        total += t;
        rounds++;
        if (total > 2.0)
            break;
    }
    return total / rounds;
}

static double measure_file(double (*fn)(int, uint8_t, const char *, const char *, int),
                           int cnt, uint8_t level, const char *message,
                           const char *title, const char *filename, int rotate)
{
    double total = 0.0;
    int rounds = 0;
    for (int i = 0; i < NUM_ROUNDS; i++)
    {
        char path[MAX_PATH_LEN];
        get_path(title, filename, path, sizeof(path));
        double t = fn(cnt, level, path, message, rotate);
        if (t < 0)
            return -1.0;
        total += t;
        rounds++;
        if (total > 2.0)
            break;
    }
    return total / rounds;
}

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
    /* Initialize Winsock for zlog's gethostname */
    WSADATA wsaData;
    WSAStartup(MAKEWORD(2, 2), &wsaData);
#endif

    printf("cnt: %d\n", cnt);
    ensure_dir(TMP_DIR);

    const char *msg_keys[] = {"short", "long"};
    const char *messages[] = {
        "Message",
        "Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message"};

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
                uint8_t level = levels[lv];
                const char *lv_name = level_names[lv];

                printf("\n### %s %s %s\n", msg_key, scenario_names[s], lv_name);

                double dt_cfl, dt_zlog;

                if (has_file[s])
                {
                    char title_cfl[MAX_PATH_LEN];
                    char title_zlog[MAX_PATH_LEN];
                    snprintf(title_cfl, sizeof(title_cfl), "cfl_%s_%s_%s",
                             msg_key, scenario_names[s], lv_name);
                    snprintf(title_zlog, sizeof(title_zlog), "zlog_%s_%s_%s",
                             msg_key, scenario_names[s], lv_name);

                    dt_cfl = measure_file(bench_cfastlogging_file, cnt, level,
                                          message, title_cfl, "logging.log", rotate_flags[s]);
                    dt_zlog = measure_file(bench_zlog_file, cnt, level,
                                           message, title_zlog, "logging.log", rotate_flags[s]);

                    cleanup_dir(title_cfl);
                    cleanup_dir(title_zlog);
                }
                else
                {
                    dt_cfl = measure_no_file(bench_cfastlogging_no_file, cnt, level, message);
                    dt_zlog = measure_no_file(bench_zlog_no_file, cnt, level, message);
                }

                printf("  cfastlogging: %.4f s\n", dt_cfl);
                printf("  zlog:         %.4f s\n", dt_zlog);

                results[m][s].levels[lv].cfl = dt_cfl;
                results[m][s].levels[lv].zlog = dt_zlog;
            }
        }
    }

    /* Write JSON output */
    char json_path[MAX_PATH_LEN];
    snprintf(json_path, sizeof(json_path), "doc/benchmarks/c_benchmark.json");
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
                fprintf(jf, "      \"%s\": {\"cfastlogging\": %.6f, \"zlog\": %.6f}%s\n",
                        level_names[lv],
                        results[m][s].levels[lv].cfl,
                        results[m][s].levels[lv].zlog,
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
                char value[128];
                snprintf(value, sizeof(value), "%.4f, %.4f",
                         results[m][s].levels[lv].cfl,
                         results[m][s].levels[lv].zlog);

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

    return 0;
}
