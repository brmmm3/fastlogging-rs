#include "cppfastlogging.hpp"

using namespace logging;

// File: syslog.cpp
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
                                   0,
                                   NULL);
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
