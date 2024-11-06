#include "h/cppfastlogging.hpp"

using namespace logging;

void writer_callback(uint8_t level, const char *domain, const char *message) {
    printf("MAIN C-CB %d %s: %s\n", level, domain, message);
}

// File: callback.cpp
//
// Sample library usage.
int main(void)
{
    WriterConfig configs[] = {CallbackWriterConfig(DEBUG, writer_callback)};
    Logging *logging = new Logging(DEBUG, "root", configs);
    logging->trace("Trace Message");
    logging->debug("Debug Message");
    logging->info("Info Message");
    logging->success("Success Message");
    logging->warn("Warning Message");
    logging->error("Error Message");
    logging->fatal("Fatal Message");
    delete logging;
    return 0;
}
