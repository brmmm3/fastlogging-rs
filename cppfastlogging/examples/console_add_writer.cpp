#include "h/cppfastlogging.hpp"

using namespace logging;

// File: console.cpp
//
// Sample library usage.
int main(void)
{
    Logging *logging = new Logging();
    ConsoleWriterConfig *console = new ConsoleWriterConfig(DEBUG, 1);
    logging->add_writer(console);
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
