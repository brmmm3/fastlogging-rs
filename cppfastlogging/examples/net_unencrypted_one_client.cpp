#include "h/cppfastlogging.hpp"

using namespace logging;

// File: net_unencrypted_one_client.cpp
//
// Sample library usage.
int main(void)
{
    // Server
    WriterConfig server_writers[] = {ConsoleWriterConfig(DEBUG, 1),
                                     FileWriterConfig(DEBUG, "/tmp/cfastlogging.log", 1024, 3)};
    Logging *logging_server = new Logging(DEBUG, "LOGSRV", server_writers);
    WriterConfig *server = new ServerConfig(DEBUG, "127.0.0.1");
    logging_server->set_root_writer_config(server);
    logging_server->sync_all(5.0);
    // Client
    const char *address_port = logging_server->get_root_server_address_port();
    printf("address=%s\n", address_port);
    rust::KeyStruct *key = logging_server->get_server_auth_key();
    WriterConfig client_writers[] = {ClientWriterConfig(DEBUG, address_port, key)};
    Logging *logging_client = new Logging(DEBUG, "LOGCLIENT", client_writers);
    printf("Send logs\n");
    // Test logging
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
