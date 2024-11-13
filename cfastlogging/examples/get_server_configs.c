#include <stdlib.h>
#include <stdio.h>
#include "h/cfastlogging.h"
#include <string.h>

// File: get_server_configs.c
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
    // Show configs
    const ServerConfigs *configs = logging_get_server_configs(logging_server);
    printf("configs=%p\n", configs);
    printf("configs->cnt=%d\n", configs->cnt);
    for (int i = 0; i < configs->cnt; i++) {
        printf("configs->key[%d]=%ld\n", i, configs->keys[i]);
        ServerConfig config = configs->values[i];
        printf("configs->value[%d]:\n", i);
        printf("  level=%d\n", config.level);
        printf("  address=%s\n", config.address);
        printf("  port=%d\n", config.port);
        printf("  key=%p:\n", config.key);
        printf("    typ=%d\n", config.key->typ);
        printf("    len=%d\n", config.key->len);
        printf("    key_ptr=%p\n", config.key->key);
    }
    uint remove_writers[] = { 0 };
    const WriterConfigEnums *writers = logging_remove_writers(logging_server, remove_writers, 1);
    printf("Shutdown Logger\n");
    logging_shutdown(logging_server, 0);
    printf("-------- Finished --------\n");
    return 0;
}
