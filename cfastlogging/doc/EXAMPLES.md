
# Examples

This section demonstrates how to use cfastlogging in C projects. All examples include basic error handling. See the end for build and run instructions.

## Logging to Console with Logging instance

```c
#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

int main(void)
{
    WriterConfigEnum writers[] = { console_writer_config_new(DEBUG, 1) };
    Logging logging = logging_new(DEBUG,
                                  NULL,
                                  writers, // Pointer to writers array
                                  1, // Array size / Number of writers
                                  NULL,
                                  NULL);
    if (logging_trace(logging, "Trace Message") != 0) fprintf(stderr, "Trace failed\n");
    if (logging_debug(logging, "Debug Message") != 0) fprintf(stderr, "Debug failed\n");
    if (logging_info(logging, "Info Message") != 0) fprintf(stderr, "Info failed\n");
    logging_shutdown(logging, 0);
    return 0;
}
```

## Logging to File using root logger

```c
#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

int main(void)
{
    WriterConfigEnum writers[] = { file_writer_config_new(DEBUG,
                                                           "/tmp/cfastlogging.log",
                                                           1024,
                                                           3,
                                                           -1,
                                                           -1,
                                                           CompressionMethodEnum_Store) };
    Logging logging = logging_new(DEBUG,
                                  NULL,
                                  writers, // Pointer to writers array
                                  1, // Array size / Number of writers
                                  NULL,
                                  NULL);
    if (logging_trace(logging, "Trace Message") != 0) fprintf(stderr, "Trace failed\n");
    if (logging_debug(logging, "Debug Message") != 0) fprintf(stderr, "Debug failed\n");
    if (logging_info(logging, "Info Message") != 0) fprintf(stderr, "Info failed\n");
    logging_shutdown(logging, 0);
    return 0;
}
```

## Logging via network sockets

```c
#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

int main(void)
{
    // Server
    WriterConfigEnum server_writers[] = { console_writer_config_new(DEBUG, 1),
                                           file_writer_config_new(DEBUG,
                                                                  "/tmp/cfastlogging.log",
                                                                  1024,
                                                                  3,
                                                                  -1,
                                                                  -1,
                                                                  CompressionMethodEnum_Store) };
    Logging logging_server = logging_new(DEBUG,
                                         "LOGSRV",
                                         server_writers,
                                         2,
                                         NULL,
                                         NULL);
    // Set root writer
    CWriterTypeEnum server = server_config_new(DEBUG, "127.0.0.1", NULL);
    logging_set_root_writer_config(logging_server, server);
    logging_sync_all(logging_server, 5.0);
    // Client
    const char *address_port = logging_get_root_server_address_port(logging_server);
    printf("address_port=%s\n", address_port);
    CKeyStruct *key = logging_get_server_auth_key(logging_server);
    WriterConfigEnum client_writers[1];
    client_writers[0] = client_writer_config_new(DEBUG, address_port, key);
    Logging logging_client = logging_new(DEBUG,
                                         "LOGCLIENT",
                                         client_writers,
                                         1,
                                         NULL,
                                         NULL);
    printf("Send logs\n");
    // Test logging
    if (logging_trace(logging_client, "Trace Message") != 0) fprintf(stderr, "Client trace failed\n");
    if (logging_debug(logging_client, "Debug Message") != 0) fprintf(stderr, "Client debug failed\n");
    if (logging_info(logging_client, "Info Message") != 0) fprintf(stderr, "Client info failed\n");

    if (logging_trace(logging_server, "Trace Message") != 0) fprintf(stderr, "Server trace failed\n");
    if (logging_debug(logging_server, "Debug Message") != 0) fprintf(stderr, "Server debug failed\n");
    if (logging_info(logging_server, "Info Message") != 0) fprintf(stderr, "Server info failed\n");

    logging_sync_all(logging_client, 1.0);
    logging_sync_all(logging_server, 1.0);
    printf("Shutdown Loggers\n");
    logging_shutdown(logging_client, 0);
    logging_shutdown(logging_server, 0);
    printf("-------- Finished --------\n");
    return 0;
}
```

## Logging using callback

```c
#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

void writer_callback(uint8_t level, const char *domain, const char *message) {
    printf("MAIN C-CB %d %s: %s\n", level, domain, message);
}

int main(void)
{
    WriterConfigEnum writers[] = { callback_writer_config_new(DEBUG, writer_callback) };
    Logging logging = logging_new(DEBUG,
                                  NULL,
                                  writers,
                                  1,
                                  NULL,
                                  NULL);
    if (logging_trace(logging, "Trace Message") != 0) fprintf(stderr, "Trace failed\n");
    if (logging_debug(logging, "Debug Message") != 0) fprintf(stderr, "Debug failed\n");
    if (logging_info(logging, "Info Message") != 0) fprintf(stderr, "Info failed\n");
    logging_shutdown(logging, 0);
    return 0;
}
```

## Logging to syslog

```c
#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

int main(void)
{
    WriterConfigEnum writers[] = { syslog_writer_config_new(DEBUG, "HOSTNAME", "PNAME", 1234) };
    Logging logging = logging_new(DEBUG,
                                  NULL,
                                  writers,
                                  1,
                                  NULL,
                                  NULL);
    if (logging_trace(logging, "Trace Message") != 0) fprintf(stderr, "Trace failed\n");
    if (logging_debug(logging, "Debug Message") != 0) fprintf(stderr, "Debug failed\n");
    if (logging_info(logging, "Info Message") != 0) fprintf(stderr, "Info failed\n");
    logging_shutdown(logging, 0);
    return 0;
}
```

## Logging and threads

```c
#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>
#include <unistd.h>
#include <pthread.h>

void *loggerThreadFun(void *vargp)
{
    Logger logger = (Logger)vargp;
    if (logger_trace(logger, "Trace Message") != 0) fprintf(stderr, "Logger trace failed\n");
    if (logger_debug(logger, "Debug Message") != 0) fprintf(stderr, "Logger debug failed\n");
    if (logger_info(logger, "Info Message") != 0) fprintf(stderr, "Logger info failed\n");
    return NULL;
}

int main(void)
{
    pthread_t thread_id;
    WriterConfigEnum writers[] = { console_writer_config_new(DEBUG, 1) };
    ExtConfig *ext_config = ext_config_new(CompressionMethodEnum_Store, 1, 1, 1, 1, 1);
    Logging logging = logging_new(DEBUG,
                                  NULL,
                                  writers,
                                  1,
                                  ext_config,
                                  NULL);
    Logger logger = logger_new_ext(DEBUG, "LoggerThread", 1, 1);
    logging_add_logger(logging, logger);
    pthread_create(&thread_id, NULL, loggerThreadFun, (void *)logger);
    if (logging_trace(logging, "Trace Message") != 0) fprintf(stderr, "Trace failed\n");
    if (logging_debug(logging, "Debug Message") != 0) fprintf(stderr, "Debug failed\n");
    if (logging_info(logging, "Info Message") != 0) fprintf(stderr, "Info failed\n");
    pthread_join(thread_id, NULL);
    logging_shutdown(logging, 0);
    return 0;

---

## Building and Running Examples

To build the examples, use your C compiler and link against the cfastlogging library. For example (GCC):

```sh
gcc -I./h -L./build/Debug -lcfastlogging -o example_console examples/console.c
./example_console
```

Replace `console.c` with the desired example file. On Windows, use `.dll` and `.exe` as appropriate.

**Note:** Ensure the cfastlogging shared library is in your library path (e.g., `LD_LIBRARY_PATH` or copy DLL to the executable directory on Windows).

---

## Error Handling

All logging functions return 0 on success and a negative value on error. Always check return values in production code.
}
```
