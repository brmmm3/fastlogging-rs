#include "h/cfastlogging.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#include <windows.h>
#undef ERROR
#define THREAD_RETURN DWORD WINAPI
#define THREAD_HANDLE HANDLE
#define THREAD_START(function, argument) \
  CreateThread(NULL, 0, function, argument, 0, NULL)
#define THREAD_JOIN(handle) WaitForSingleObject(handle, INFINITE)
#define THREAD_CLOSE(handle) CloseHandle(handle)
#else
#include <pthread.h>
#define THREAD_RETURN void *
#define THREAD_HANDLE pthread_t
#define THREAD_START(function, argument) \
  pthread_create(&thread_id, NULL, function, argument)
#define THREAD_JOIN(handle) pthread_join(handle, NULL)
#define THREAD_CLOSE(handle) ((void)0)
#endif

THREAD_RETURN loggerThreadFun(void *vargp)
{
  Logger logger = (Logger)vargp;
  logger_trace(logger, "Trace Message");
  logger_debug(logger, "Debug Message");
  logger_info(logger, "Info Message");
  logger_success(logger, "Success Message");
  logger_warning(logger, "Warning Message");
  logger_error(logger, "Error Message");
  logger_fatal(logger, "Fatal Message");
#ifdef _WIN32
  return 0;
#else
  return NULL;
#endif
}

// File: threads.c
//
// Sample library usage.
int main(void)
{
  THREAD_HANDLE thread_id;
  WriterConfigEnum writers[] = {console_writer_config_new(DEBUG, 1)};
  ExtConfig *ext_config =
      ext_config_new(MessageStructEnum_String, 1, 1, 1, 1, 1);
  Logging logging = logging_new(DEBUG, NULL, writers, 1, ext_config, NULL);
  Logger logger = logger_new_ext(DEBUG, "LoggerThread", 1, 1);
  logging_add_logger(logging, logger);
#ifdef _WIN32
  thread_id = THREAD_START(loggerThreadFun, (void *)logger);
#else
  THREAD_START(loggerThreadFun, (void *)logger);
#endif
  logging_trace(logging, "Trace Message");
  logging_debug(logging, "Debug Message");
  logging_info(logging, "Info Message");
  logging_success(logging, "Success Message");
  logging_warning(logging, "Warning Message");
  logging_error(logging, "Error Message");
  logging_fatal(logging, "Fatal Message");
  THREAD_JOIN(thread_id);
  THREAD_CLOSE(thread_id);
  logging_shutdown(logging, 0);
  return 0;
}
