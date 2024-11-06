#include "h/cppfastlogging.hpp"

using namespace logging;

// File: console.cpp
//
// Sample library usage.
int main(void)
{
    WriterConfig configs[] = {ConsoleWriterConfig(DEBUG, 1)};
    Logging logging = Logging(DEBUG, "root", configs);
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
