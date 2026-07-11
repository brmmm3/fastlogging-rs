#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

void writer_callback(uint8_t level, const char *domain, const char *message)
{
    printf("MAIN C-CB %d %s: %s\n", level, domain, message);
}

int main(void)
{
    Logging logging(DEBUG, "root");
    logging.add_writer_config(CallbackWriterConfig(DEBUG, writer_callback));
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
