#include "h/cppfastlogging.hpp"

using namespace logging;

// File: file.cpp
//
// Sample library usage.
int main(void)
{
    Logging *logging = new Logging();
    FileWriterConfig *file = new FileWriterConfig(DEBUG,
                                                  "/tmp/cfastlogging.log",
                                                  1024,
                                                  3,
                                                  -1,
                                                  -1,
                                                  CompressionMethod::Store);
    logging->add_writer(file);
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
