#include "cxxfastlogging/h/fastlogging.h"
#include <thread>

void log_from_worker(const rust::Box<Logger> &logger)
{
    logger->trace("Trace Message");
    logger->debug("Debug Message");
    logger->info("Info Message");
    logger->success("Success Message");
    logger->warning("Warning Message");
    logger->error("Error Message");
    logger->fatal("Fatal Message");
}

int main()
{
    rust::Vec<rust::Box<WriterConfig>> configs;
    configs.push_back(WriterConfig::new_console(DEBUG, true));
    auto log = Logging::create(DEBUG, "root", std::move(configs));
    log->set_ext_config(ExtConfigFfi{
        MessageStructEnum::String,
        true,
        true,
        true,
        true,
        true,
    });

    auto logger = Logger::new_ext(DEBUG, "worker", true, true);
    log->add_logger(*logger);
    std::thread worker(log_from_worker, std::cref(logger));

    log->trace("Trace Message");
    log->debug("Debug Message");
    log->info("Info Message");
    log->success("Success Message");
    log->warning("Warning Message");
    log->error("Error Message");
    log->fatal("Fatal Message");

    worker.join();
    log->shutdown(false);
    return 0;
}
