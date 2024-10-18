#include "h/cppfastlogging.hpp"

using namespace logging;

// File: net_unencrypted_one_client.cpp
//
// Sample library usage.
int main(void)
{
    // Server
    WriterConfig configs[] = {ConsoleWriterConfig(DEBUG, 1),
                              ServerConfig(DEBUG, "127.0.0.1", CEncryptionMethodEnum_t::NONE, NULL)};
    Logging *logging_server = new Logging(DEBUG,
                                          "LOGSRV",
                                          configs);
    WriterConfig *server = new ServerConfig(DEBUG, "127.0.0.1", CEncryptionMethodEnum_t::NONE, NULL);
    printf("server_config=%p\n", server);
    logging_server->set_root_writer_config(server);
    logging_server->sync_all(5.0);
    // Show addresses and ports
    const Cu32u16Vec_t *ports = logging_server->get_server_ports();
    printf("ports->cnt=%d\n", ports->cnt);
    for (int i = 0; i < ports->cnt; i++) {
        printf("ports->key[%d]=%d\n", i, ports->keys[i]);
        printf("ports->value[%d]=%d\n", i, ports->values[i]);
    }
    const Cu32StringVec_t *addresses = logging_server->get_server_addresses();
    printf("addresses->cnt=%d\n", addresses->cnt);
    for (int i = 0; i < addresses->cnt; i++) {
        printf("addresses->key[%d]=%d\n", i, addresses->keys[i]);
        printf("addresses->value[%d]=%s\n", i, addresses->values[i]);
    }
    const Cu32StringVec_t *addresses_ports = logging_server->get_server_addresses_ports();
    printf("addresses_ports->cnt=%d\n", addresses_ports->cnt);
    for (int i = 0; i < addresses_ports->cnt; i++) {
        printf("addresses_ports->key[%d]=%d\n", i, addresses_ports->keys[i]);
        printf("addresses_ports->value[%d]=%s\n", i, addresses_ports->values[i]);
    }
    // Test logging
    logging_server->info("Info Message");
    logging_server->sync_all(1.0);
    printf("Shutdown Loggers\n");
    delete logging_server;
    printf("-------- Finished --------\n");
    return 0;
}
