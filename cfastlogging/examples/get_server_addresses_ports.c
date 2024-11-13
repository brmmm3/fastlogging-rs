#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

// File: get_server_addresses_ports.c
//
// Sample library usage.
int main(void)
{
    // Server
    struct WriterConfigEnum *server_configs[] = { console_writer_config_new(DEBUG, 1),
                                                  server_config_new(DEBUG, "127.0.0.1", NULL) };
    struct WriterConfigEnums server_writers = { .cnt=1, .wids=NULL, .configs=server_configs };
    Logging logging_server = logging_new(DEBUG,
                                         "LOGSRV",
                                         &server_writers,
                                         NULL,
                                         NULL);
    WriterConfigEnum *server = server_config_new(DEBUG, "127.0.0.1", NULL);
    printf("server_config=%p\n", server);
    logging_set_root_writer_config(logging_server, server);
    logging_sync_all(logging_server, 5.0);
    // Show addresses and ports
    const Cu32u16Vec *ports = logging_get_server_ports(logging_server);
    printf("ports->cnt=%d\n", ports->cnt);
    for (int i = 0; i < ports->cnt; i++) {
        printf("ports->key[%d]=%d\n", i, ports->keys[i]);
        printf("ports->value[%d]=%d\n", i, ports->values[i]);
    }
    const Cu32StringVec *addresses = logging_get_server_addresses(logging_server);
    printf("addresses->cnt=%d\n", addresses->cnt);
    for (int i = 0; i < addresses->cnt; i++) {
        printf("addresses->key[%d]=%d\n", i, addresses->keys[i]);
        printf("addresses->value[%d]=%s\n", i, addresses->values[i]);
    }
    const Cu32StringVec *addresses_ports = logging_get_server_addresses_ports(logging_server);
    printf("addresses_ports->cnt=%d\n", addresses_ports->cnt);
    for (int i = 0; i < addresses_ports->cnt; i++) {
        printf("addresses_ports->key[%d]=%d\n", i, addresses_ports->keys[i]);
        printf("addresses_ports->value[%d]=%s\n", i, addresses_ports->values[i]);
    }
    // Test logging
    logging_info(logging_server, "Info Message");
    logging_sync_all(logging_server, 1.0);
    printf("Shutdown Logger\n");
    logging_shutdown(logging_server, 0);
    printf("-------- Finished --------\n");
    return 0;
}
