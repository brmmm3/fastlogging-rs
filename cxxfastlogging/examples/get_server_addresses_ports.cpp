#include "cxxfastlogging/h/fastlogging.h"
#include <cstdio>

int main()
{
    auto log = Logging::create(DEBUG, "server", {});
    log->add_writer_config(WriterConfig::new_console(DEBUG, true));
    auto server = WriterConfig::new_server(
        DEBUG, "127.0.0.1", EncryptionMethodEnum::NONE, {});
    log->add_writer_config(std::move(server));
    log->set_root_writer_config(WriterConfig::new_server(
        DEBUG, "127.0.0.1", EncryptionMethodEnum::NONE, {}));
    log->sync_all(5.0);

    for (const auto &port : log->get_server_ports())
    {
        std::printf("port[%llu]=%u\n",
                    static_cast<unsigned long long>(port.id), port.value);
    }
    auto addresses = log->get_server_addresses();
    for (auto &address : addresses)
    {
        std::printf("address[%llu]=%s\n",
                    static_cast<unsigned long long>(address.id), address.value.c_str());
    }
    auto addresses_ports = log->get_server_addresses_ports();
    for (auto &address_port : addresses_ports)
    {
        std::printf("address_port[%llu]=%s\n",
                    static_cast<unsigned long long>(address_port.id),
                    address_port.value.c_str());
    }

    log->info("Info Message");
    log->sync_all(1.0);
    log->shutdown(false);
    return 0;
}
