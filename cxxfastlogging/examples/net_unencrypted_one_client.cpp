#include "cxxfastlogging/h/fastlogging.h"
#include <chrono>
#include <thread>

int main()
{
    auto server = Logging::create(DEBUG, "server", {});
    server->add_writer_config(WriterConfig::new_console(DEBUG, true));
    server->set_root_writer_config(WriterConfig::new_server(
        DEBUG, "127.0.0.1", EncryptionMethodEnum::NONE, {}));
    server->sync_all(5.0);

    auto address = server->get_root_server_address_port();
    rust::Vec<rust::Box<WriterConfig>> client_configs;
    client_configs.push_back(WriterConfig::new_client(
        DEBUG, address, EncryptionMethodEnum::NONE, {}));
    auto client = Logging::create(DEBUG, "client", std::move(client_configs));

    client->info("Hello from client");
    server->info("Hello from server");
    client->sync_all(1.0);
    server->sync_all(1.0);
    std::this_thread::sleep_for(std::chrono::milliseconds(50));

    client->shutdown(false);
    server->shutdown(false);
    return 0;
}
