#include "h/cfastlogging.h"
#include <stdio.h>
#include <stdlib.h>

// File: otel.c
//
// Sample usage of the OpenTelemetry writer.
// Logs messages to a local OpenTelemetry Collector at http://localhost:4318/v1/logs.
int main(void)
{
    WriterConfigEnum writers[] = {
        otel_writer_config_new(DEBUG, "http://localhost:4318", "my-c-app")};
    Logging logging = logging_new(DEBUG, NULL, writers, 1, NULL, NULL);
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
