#include "cxxfastlogging/h/fastlogging.h"

int main()
{
    rust::Vec<rust::Box<WriterConfig>> configs;
    configs.push_back(WriterConfig::new_otel(
        DEBUG,
        "http://localhost:4318",
        "my-cxx-app"));

    auto log = Logging::create(DEBUG, "root", std::move(configs));
    log->trace("Trace Message");
    log->debug("Debug Message");
    log->info("Info Message");
    log->success("Success Message");
    log->warning("Warning Message");
    log->error("Error Message");
    log->fatal("Fatal Message");
    log->shutdown(false);
    return 0;
}