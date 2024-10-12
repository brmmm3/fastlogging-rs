#include <stdlib.h>
#include <stdio.h>
#include "cfastlogging.h"
#include <string.h>

// File: console.c
//
// Sample library usage.
int main(void)
{
    WriterConfigEnum writers[1];
    writers[0] = console_writer_config_new(DEBUG, 1);
    Logging logging = logging_new(DEBUG,
                                  NULL,
                                  writers, // Pointer to writers array
                                  1, // Array size / Number of writers
                                  NULL,
                                  NULL);
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
