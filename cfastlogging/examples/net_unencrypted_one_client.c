#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

// File: net_unencrypted_one_client.c
//
// Sample library usage.
int main(void)
{
    // Server
    CWriterConfigEnum server_writers[2];
    server_writers[0] = console_writer_config_new(DEBUG, 1);
    CCompressionMethodEnum compression = CompressionMethodEnum_Store;
    server_writers[1] = file_writer_config_new(DEBUG,
                                               "/tmp/cfastlogging.log",
                                               1024,
                                               3,
                                               -1,
                                               -1,
                                               compression);
    Logging logging_server = logging_new(DEBUG,
                                         "LOGSRV",
                                         server_writers,
                                         2,
                                         NULL,
                                         NULL);
    // Set root writer
    CWriterTypeEnum server = server_config_new(DEBUG, "127.0.0.1", NULL);
    logging_set_root_writer_config(logging_server, server);
    //logging_set_debug(logging_server, 3);
    logging_sync_all(logging_server, 5.0);
    // Client
    const char *address_port = logging_get_root_server_address_port(logging_server);
    printf("address_port=%s\n", address_port);
    CKeyStruct *key = logging_get_server_auth_key(logging_server);
    CWriterConfigEnum client_writers[1];
    client_writers[0] = client_writer_config_new(DEBUG, address_port, key);
    Logging logging_client = logging_new(DEBUG,
                                         "LOGCLIENT",
                                         client_writers,
                                         1,
                                         NULL,
                                         NULL);
    //logging_set_debug(logging_client, 3);
    printf("Send logs\n");
    // Test logging
    logging_trace(logging_client, "Trace Message");
    logging_debug(logging_client, "Debug Message");
    logging_info(logging_client, "Info Message");
    logging_success(logging_client, "Success Message");
    logging_warning(logging_client, "Warning Message");
    logging_error(logging_client, "Error Message");
    logging_fatal(logging_client, "Fatal Message");

    logging_trace(logging_server, "Trace Message");
    logging_debug(logging_server, "Debug Message");
    logging_info(logging_server, "Info Message");
    logging_success(logging_server, "Success Message");
    logging_warning(logging_server, "Warning Message");
    logging_error(logging_server, "Error Message");
    logging_fatal(logging_server, "Fatal Message");

    logging_sync_all(logging_client, 1.0);
    logging_sync_all(logging_server, 1.0);
    printf("Shutdown Loggers\n");
    logging_shutdown(logging_client, 0);
    logging_shutdown(logging_server, 0);
    printf("-------- Finished --------\n");
    return 0;
}
