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
    const char *address = logging_server->get_root_server_address_port();
    printf("address=%s\n", address);
    const char *key = logging_server->get_server_auth_key();
    printf("key=%s\n", key);
    // Client
    Logging *logging_client = new Logging(DEBUG,
                                          "LOGCLIENT",
                                          configs);
    CWriterTypeEnum_t client_writer = client_writer_config_new(DEBUG, address, CEncryptionMethodEnum_t::AuthKey, key);
    printf("client_writer=%p\n", client_writer);
    logging_server->set_root_writer_config(client_writer);
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
