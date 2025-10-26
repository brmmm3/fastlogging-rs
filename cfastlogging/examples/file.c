#include "h/cfastlogging.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// File: file.c
//
// Sample library usage.
int main(void) {
  CWriterConfigEnum writers[] = {
      file_writer_config_new(DEBUG, "/tmp/cfastlogging.log", 1024, 3, -1, -1,
                             CompressionMethodEnum_Store)};
  Logging logging = logging_new(DEBUG, NULL,
                                writers, // Pointer to writers array
                                1, NULL, NULL);
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
