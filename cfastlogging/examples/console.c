#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

// File: console.c
//
// Sample library usage.
int main(void)
{
    struct WriterConfigEnum *configs[] = { console_writer_config_new(DEBUG, 1) };
    struct WriterConfigEnums writers = { .cnt=1, .wids=NULL, .configs=configs };
    printf("C writers.cnt=%d\n", writers.cnt);
    printf("C writers.configs=%p\n", writers.configs);
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
