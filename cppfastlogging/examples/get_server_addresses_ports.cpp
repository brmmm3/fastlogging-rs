#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    Logging logging_server(DEBUG, "LOGSRV");
    logging_server.add_writer_config(ConsoleWriterConfig(DEBUG, true));
    ServerConfig srv(DEBUG, "127.0.0.1");
    logging_server.add_writer_config(srv);
    logging_server.set_root_writer_config(srv);
    logging_server.sync_all(5.0);

    const Cu32u16Vec_t *ports = logging_server.get_server_ports();
    if (ports) {
        printf("ports->cnt=%d\n", ports->cnt);
        for (uint32_t i = 0; i < ports->cnt; i++) {
            printf("ports->key[%u]=%u\n",   i, ports->keys[i]);
            printf("ports->value[%u]=%u\n", i, ports->values[i]);
        }
    }
    const Cu32StringVec_t *addresses = logging_server.get_server_addresses();
    if (addresses) {
        printf("addresses->cnt=%d\n", addresses->cnt);
        for (uint32_t i = 0; i < addresses->cnt; i++) {
            printf("addresses->key[%u]=%u\n",   i, addresses->keys[i]);
            printf("addresses->value[%u]=%s\n", i, addresses->values[i]);
        }
    }
    const Cu32StringVec_t *ap = logging_server.get_server_addresses_ports();
    if (ap) {
        printf("addresses_ports->cnt=%d\n", ap->cnt);
        for (uint32_t i = 0; i < ap->cnt; i++) {
            printf("addresses_ports->key[%u]=%u\n",   i, ap->keys[i]);
            printf("addresses_ports->value[%u]=%s\n", i, ap->values[i]);
        }
    }
    logging_server.info("Info Message");
    logging_server.sync_all(1.0);
    printf("-------- Finished --------\n");
    return 0;
}
