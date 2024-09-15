#include "cppfastlogging.hpp"

using namespace logging;

// File: file.cpp
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
    FileWriterConfigEnum *file = new FileWriterConfigEnum(DEBUG,
                                                          "/tmp/cfastlogging.log",
                                                          1024,
                                                          3,
                                                          -1,
                                                          -1,
                                                          CompressionMethodEnum::Store);
    logging->add_writer(file->writer);
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
