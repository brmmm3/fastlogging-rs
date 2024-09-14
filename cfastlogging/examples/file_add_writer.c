#include <stdlib.h>
#include <stdio.h>
#include "cfastlogging.h"
#include <string.h>

// File: file.c
//
// Sample library usage.
int main(void)
{
    Logging logging = logging_new(DEBUG,
                                  NULL,
                                  NULL,
                                  NULL,
                                  NULL,
                                  NULL,
                                  NULL,
                                  -1,
                                  NULL);
    CompressionMethodEnum compression = Store;
    WriterConfigEnum file = file_writer_config_enum_new(DEBUG,
                                                   "/tmp/cfastlogging.log",
                                                   1024,
                                                   3,
                                                   -1,
                                                   -1,
                                                   compression);

    logging_add_writer(logging, file);
    printf("ADDED\n");
    logging_trace(logging, "Trace Message");
    logging_debug(logging, "Debug Message");
    logging_info(logging, "Info Message");
    logging_success(logging, "Success Message");
    logging_warning(logging, "Warning Message");
    logging_error(logging, "Error Message");
    logging_fatal(logging, "Fatal Message");
    logging_shutdown(logging, 0);
    return 0;
}
