#include "cppfastlogging.hpp"

using namespace logging;

// File: console.cpp
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
    ConsoleWriterConfigEnum *console = new ConsoleWriterConfigEnum(DEBUG, 1);
    logging->add_writer(console->writer);
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
