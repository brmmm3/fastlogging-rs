#include "cxxfastlogging/h/fastlogging.h"

int main() {
    auto console = WriterConfig::new_console(10 /* DEBUG */, true);
    rust::Vec<rust::Box<WriterConfig>> configs;
    configs.push_back(std::move(console));
    auto logging = Logging::create(0 /* NOTSET */, "root", std::move(configs));
    logging->info("Hello from C++!");
    logging->shutdown(false);
    return 0;
}
