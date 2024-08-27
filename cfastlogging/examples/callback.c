#include <stdlib.h>
#include <stdio.h>
#include "cfastlogging.h"
#include <string.h>

void writer_callback(uint8_t level, *const char domain, *const char message) {
    printf("CB %d %s: %s\n", level, domain, message);
}

// File: console.c
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
    CallbackWriterConfig callback_writer = callback_writer_config_new(DEBUG, writer_callback);
    logging_add_writer(logging, callback_writer);
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
