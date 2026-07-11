#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    // Server
    Logging *logging_server = new Logging(DEBUG, "LOGSRV");
    logging_server->add_writer_config(ConsoleWriterConfig(DEBUG, true));
    logging_server->add_writer_config(
        FileWriterConfig(DEBUG, "/tmp/cfastlogging.log", 1024, 3));
    ServerConfig srv(DEBUG, "127.0.0.1");
    logging_server->add_writer_config(srv);
    logging_server->set_root_writer_config(srv);
    logging_server->sync_all(5.0);

    // Client
    const char *address_port = logging_server->get_root_server_address_port();
    printf("address=%s\n", address_port ? address_port : "(null)");
    rust::KeyStruct *key = logging_server->get_server_auth_key();

    Logging *logging_client = new Logging(DEBUG, "LOGCLIENT");
    logging_client->add_writer_config(ClientWriterConfig(DEBUG, address_port, key));

    printf("Send logs\n");
    logging_client->trace("Trace Message");
    logging_client->debug("Debug Message");
    logging_client->info("Info Message");
    logging_client->success("Success Message");
    logging_client->warning("Warning Message");
    logging_client->error("Error Message");
    logging_client->fatal("Fatal Message");

    logging_client->sync_all(1.0);
    logging_server->sync_all(1.0);
    printf("Shutdown Loggers\n");
    delete logging_client;
    delete logging_server;
    printf("-------- Finished --------\n");
    return 0;
}
