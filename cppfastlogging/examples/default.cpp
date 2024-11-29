#include "h/cppfastlogging.hpp"

using namespace logging;

// File: console.cpp
//
// Sample library usage.
int main(void)
{
    Logging logging = Logging::Default();
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
