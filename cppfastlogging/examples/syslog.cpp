#include "h/cppfastlogging.hpp"

using namespace logging;

// File: syslog.cpp
//
// Sample library usage.
int main(void)
{
    WriterConfig configs[] = {SyslogWriterConfig(DEBUG,
                                                 "hostname",
                                                 "pname",
                                                 1234)};
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
