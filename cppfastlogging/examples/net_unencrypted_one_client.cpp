#include "cppfastlogging.hpp"

using namespace logging;

// File: net_unencrypted_one_client.cpp
//
// Sample library usage.
int main(void)
{
    ServerConfig *server = new ServerConfig(DEBUG, "127.0.0.1", EncryptionMethod::NONE, NULL);
    printf("server_config=%p\n", server);
    ConsoleWriterConfig *console = new ConsoleWriterConfig(DEBUG, 1);
    printf("console=%p\n", console);
    Logging *logging_server = new Logging(DEBUG,
                                          "LOGSRV",
                                          NULL,
                                          console,
                                          NULL,
                                          server,
                                          NULL,
                                          -1,
                                          NULL);
    logging_server->sync_all(5.0);
    const char *address = logging_server->get_server_address();
    printf("address=%s\n", address);
    const char *key = logging_server->get_server_auth_key();
    printf("key=%s\n", key);
    ClientWriterConfig *client_writer = new ClientWriterConfig(DEBUG, address, EncryptionMethod::NONE, key);
    Logging *logging_client = new Logging(DEBUG,
                                          "LOGCLIENT",
                                          NULL,
                                          NULL,
                                          NULL,
                                          NULL,
                                          client_writer,
                                          -1,
                                          NULL);
    printf("Send logs\n");
    logging_client->trace("Trace Message");
    logging_client->debug("Debug Message");
    logging_client->info("Info Message");
    logging_client->success("Success Message");
    logging_client->warn("Warning Message");
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
