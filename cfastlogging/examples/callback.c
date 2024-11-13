#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

void writer_callback(uint8_t level, const char *domain, const char *message) {
    printf("MAIN C-CB %d %s: %s\n", level, domain, message);
}

// File: callback.c
//
// Sample library usage.
int main(void)
{
    struct WriterConfigEnum *configs[] = { callback_writer_config_new(DEBUG, writer_callback) };
    struct WriterConfigEnums writers = { .cnt=1, .wids=NULL, .configs=configs };
    Logging logging = logging_new(DEBUG,
                                  NULL,
                                  &writers,
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
