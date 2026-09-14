/*
 * benchmark_cxx.cpp — cxxfastlogging benchmark functions.
 *
 * Separate translation unit to avoid type name conflicts between
 * cppfastlogging and cxxfastlogging (both define WriterConfig, Logging
 * etc. in the global namespace).
 */

#include "lib.rs.h"
#include <cstdio>
#include <cstring>
#include <chrono>

#define MB (1024 * 1024)

static double now_sec_cxx(void)
{
    auto t = std::chrono::high_resolution_clock::now();
    auto dur = t.time_since_epoch();
    return std::chrono::duration<double>(dur).count();
}

static double logging_work_cxx(Logging &logging, int cnt, const char *message)
{
    char buf[512];
    double t1 = now_sec_cxx();
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
    return now_sec_cxx() - t1;
}

extern "C"
{

    double bench_cxx_no_file(int cnt, uint8_t level, const char *message)
    {
        rust::Vec<rust::Box<WriterConfig>> configs;
        auto logging = Logging::create(level, "bench", std::move(configs));
        double dt = logging_work_cxx(*logging, cnt, message);
        logging->shutdown(false);
        return dt;
    }

    double bench_cxx_file(int cnt, uint8_t level, const char *path,
                          const char *message, int rotate)
    {
        uint64_t size = rotate ? MB : 0;
        uint64_t backlog = rotate ? 8 : 0;
        auto wc = WriterConfig::new_file(level, path, size, backlog, -1, -1,
                                         CompressionMethodEnum::Store);
        rust::Vec<rust::Box<WriterConfig>> configs;
        configs.push_back(std::move(wc));
        auto logging = Logging::create(level, "bench", std::move(configs));
        double dt = logging_work_cxx(*logging, cnt, message);
        logging->sync_all(10.0);
        logging->shutdown(false);
        return dt;
    }

} /* extern "C" */
