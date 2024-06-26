#include "cppfastlogging.hpp"

using namespace logging;

// File: console.cpp
//
// Sample library usage.
int main(void)
{
    ConsoleWriterConfig *console = new ConsoleWriterConfig(DEBUG, 1);
    Logging *logging = new Logging(DEBUG,
                                   NULL,
                                   NULL,
                                   console,
                                   NULL,
                                   NULL,
                                   NULL,
                                   -1,
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
