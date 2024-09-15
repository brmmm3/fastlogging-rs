#include "cppfastlogging.hpp"

using namespace logging;

void writer_callback(uint8_t level, const char *domain, const char *message) {
    printf("MAIN C-CB %d %s: %s\n", level, domain, message);
}

// File: callback.cpp
//
// Sample library usage.
int main(void)
{
    Logging *logging = new Logging(DEBUG,
                                   NULL,
                                   NULL,
                                   NULL,
                                   NULL,
                                   NULL,
                                   NULL,
                                   -1,
                                   NULL);
    CallbackWriterConfigEnum *callback = new CallbackWriterConfigEnum(DEBUG, writer_callback);
    logging->add_writer(callback->writer);
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
