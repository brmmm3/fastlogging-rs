#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>
#include <unistd.h> //Header file for sleep(). man 3 sleep for details.
#include <pthread.h>

void *loggerThreadFun(void *vargp)
{
    Logger logger = (Logger)vargp;
    logger_trace(logger, "Trace Message");
    logger_debug(logger, "Debug Message");
    logger_info(logger, "Info Message");
    logger_success(logger, "Success Message");
    logger_warning(logger, "Warning Message");
    logger_error(logger, "Error Message");
    logger_fatal(logger, "Fatal Message");
    return NULL;
}

// File: threads.c
//
// Sample library usage.
int main(void)
{
    pthread_t thread_id;
    CWriterConfigEnum writers[] = { console_writer_config_new(DEBUG, 1) };
    CExtConfig *ext_config = ext_config_new(CompressionMethodEnum_Store, 1, 1, 1, 1, 1);
    Logging logging = logging_new(DEBUG,
                                  NULL,
                                  writers,
                                  1,
                                  ext_config,
                                  NULL);
    Logger logger = logger_new_ext(DEBUG, "LoggerThread", 1, 1);
    logging_add_logger(logging, logger);
    pthread_create(&thread_id, NULL, loggerThreadFun, (void *)logger);
    logging_trace(logging, "Trace Message");
    logging_debug(logging, "Debug Message");
    logging_info(logging, "Info Message");
    logging_success(logging, "Success Message");
    logging_warning(logging, "Warning Message");
    logging_error(logging, "Error Message");
    logging_fatal(logging, "Fatal Message");
    pthread_join(thread_id, NULL);
    logging_shutdown(logging, 0);
    return 0;
}
