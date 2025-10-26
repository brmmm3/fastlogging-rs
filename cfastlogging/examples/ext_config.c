#include "h/cfastlogging.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// File: ext_config.c
//
// Sample library usage.
int main(void) {
  CWriterConfigEnum writers[] = {console_writer_config_new(DEBUG, 1)};
  Logging logging = logging_new(DEBUG, NULL,
                                writers, // Pointer to writers array
                                1, NULL, NULL);
  CExtConfig *ext_config = ext_config_new(MessageStructEnum_Xml, 1, 0, 1, 0, 1);
  logging_set_ext_config(logging, ext_config);
  printf("ext_config.structured=%d\n", ext_config->structured);
  printf("ext_config.hostname=%d\n", ext_config->hostname);
  printf("ext_config.pname=%d\n", ext_config->pname);
  printf("ext_config.pid=%d\n", ext_config->pid);
  printf("ext_config.tname=%d\n", ext_config->tname);
  printf("ext_config.tid=%d\n", ext_config->tid);
  logging_trace(logging, "Trace Message");
  logging_debug(logging, "Debug Message");
  logging_info(logging, "Info Message");
  logging_success(logging, "Success Message");
  logging_warning(logging, "Warning Message");
  logging_error(logging, "Error Message");
  logging_fatal(logging, "Fatal Message");
  logging_shutdown(logging, 0);
  printf("-------- Finished --------\n");
  return 0;
}
