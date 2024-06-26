#include <stdlib.h>
#include <stdio.h>
#include "cfastlogging.h"
#include <string.h>

// File: net_unencrypted_one_client.c
//
// Sample library usage.
int main(void)
{
    ServerConfig server = server_config_new(DEBUG, "127.0.0.1", String, NONE);
    printf("server_config=%p\n", server);
    ConsoleWriterConfig console = console_writer_config_new(DEBUG, 1);
    printf("console=%p\n", console);
    Logging logging_server = logging_new(DEBUG,
                                         "LOGSRV",
                                         NULL,
                                         console,
                                         NULL,
                                         server,
                                         NULL,
                                         -1,
                                         NULL);
    logging_sync_all(logging_server, 5.0);
    const char *address = logging_get_server_address(logging_server);
    printf("address=%s\n", address);
    const char *key = logging_get_server_auth_key(logging_server);
    printf("key=%s\n", key);
    ClientWriterConfig client_writer = client_writer_config_new(DEBUG, address, String, key);
    Logging logging_client = logging_new(DEBUG,
                                         "LOGCLIENT",
                                         NULL,
                                         NULL,
                                         NULL,
                                         NULL,
                                         client_writer,
                                         -1,
                                         NULL);
    printf("Send logs\n");
    logging_trace(logging_client, "Trace Message");
    logging_debug(logging_client, "Debug Message");
    logging_info(logging_client, "Info Message");
    logging_success(logging_client, "Success Message");
    logging_warning(logging_client, "Warning Message");
    logging_error(logging_client, "Error Message");
    logging_fatal(logging_client, "Fatal Message");
    logging_sync_all(logging_client, 1.0);
    logging_sync_all(logging_server, 1.0);
    printf("Shutdown Loggers\n");
    logging_shutdown(logging_client, 0);
    logging_shutdown(logging_server, 0);
    printf("-------- Finished --------\n");
    return 0;
}
