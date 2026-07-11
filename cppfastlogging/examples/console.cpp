#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    Logging logging(DEBUG, "root");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, true));
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
