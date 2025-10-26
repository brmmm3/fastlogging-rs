#include "h/cfastlogging.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// File: get_server_configs.c
//
// Sample library usage.

int main(void) {
  // Server
  CWriterConfigEnum server_writers[] = {
      console_writer_config_new(DEBUG, 1),
      server_config_new(DEBUG, "127.0.0.1", NULL)};
  Logging logging_server =
      logging_new(DEBUG, "LOGSRV", server_writers, 2, NULL, NULL);
  CWriterTypeEnum server = server_config_new(DEBUG, "127.0.0.1", NULL);
  printf("server_config=%p\n", server);
  logging_set_root_writer_config(logging_server, server);
  logging_sync_all(logging_server, 5.0);
  // Show configs
  const CServerConfigs *configs = logging_get_server_configs(logging_server);
  printf("configs=%p\n", configs);
  printf("configs->cnt=%d\n", configs->cnt);
  for (int i = 0; i < configs->cnt; i++) {
    printf("configs->key[%d]=%d\n", i, configs->keys[i]);
    CServerConfig config = configs->values[i];
    printf("configs->value[%d]:\n", i);
    printf("  level=%d\n", config.level);
    printf("  address=%s\n", config.address);
    printf("  port=%d\n", config.port);
    printf("  key=%p:\n", config.key);
    if (config.key != NULL) {
      printf("    typ=%d\n", config.key->typ);
      printf("    len=%d\n", config.key->len);
      printf("    key_ptr=%p\n", config.key->key);
    }
    printf("  port_file=%p:\n", config.port_file);
  }
  // Remove ROOT writer
  printf("Remove ROOT writer.\n");
  uint32_t remove_writers[] = {0};
  CWriterEnums *writers =
      logging_remove_writers(logging_server, remove_writers, 1);
  // Show configs
  const CServerConfigs *configs2 = logging_get_server_configs(logging_server);
  printf("configs2=%p\n", configs);
  printf("configs2->cnt=%d\n", configs2->cnt);
  for (int i = 0; i < configs2->cnt; i++) {
    printf("configs2->key[%d]=%d\n", i, configs2->keys[i]);
    CServerConfig config = configs2->values[i];
    printf("configs2->value[%d]:\n", i);
    printf("  level=%d\n", config.level);
    printf("  address=%s\n", config.address);
    printf("  port=%d\n", config.port);
    printf("  key=%p:\n", config.key);
    if (config.key != NULL) {
      printf("    typ=%d\n", config.key->typ);
      printf("    len=%d\n", config.key->len);
      printf("    key_ptr=%p\n", config.key->key);
    }
    printf("  port_file=%p:\n", config.port_file);
  }
  printf("Shutdown Logger\n");
  logging_shutdown(logging_server, 0);
  printf("-------- Finished --------\n");
  return 0;
}
