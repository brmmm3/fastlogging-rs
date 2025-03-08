#pragma once

#include <vector>

#include "def.hpp"

// Root logging module

extern "C"
{
    void root_init();

    int root_shutdown(int8_t now);

    int root_set_level(uint32_t wid, uint8_t level);

    void root_set_domain(const char *domain);

    void root_set_level2sym(uint8_t level2sym);

    void root_set_ext_config(rust::ExtConfig *ext_config);

    void root_add_logger(Logger logger);

    void root_remove_logger(Logger logger);

    int root_set_root_writer_config(rust::WriterConfigEnum config);

    int root_set_root_writer(rust::WriterEnum writer);

    int root_add_writer_config(rust::WriterConfigEnum config);

    int root_add_writer(rust::WriterEnum config);

    int root_remove_writer(uint32_t wid);

    int root_add_writer_configs(rust::WriterConfigEnums *configs, uint32_t config_cnt);

    int root_add_writers(rust::WriterEnums *writers, uint32_t writer_cnt);

    rust::WriterEnums *root_remove_writers(uint32_t *wids, uint32_t wid_cnt);

    int root_enable(uint32_t wid);

    int root_disable(uint32_t wid);

    int root_enable_type(rust::WriterTypeEnum typ);

    int root_disable_type(rust::WriterTypeEnum typ);

    int root_sync(rust::WriterTypeEnum *types, uint32_t type_cnt, double timeout);

    int root_sync_all(double timeout);

    // File writer

    int root_rotate(const char *path);

    // Network

    int root_set_encryption(uint32_t wid, const KeyStruct *key);

    // Config

    void root_set_debug(uint32_t debug);

    const rust::WriterConfigEnum *root_get_writer_config(uint32_t wid);

    const std::vector<rust::WriterConfigEnum> root_get_writer_configs();

    const ServerConfig *root_get_server_config();

    const std::vector<ServerConfig> root_get_server_configs();

    const char *root_get_root_server_address_port();

    const Cu32StringVec *root_get_server_addresses_ports();

    const Cu32StringVec *root_get_server_addresses();

    const Cu32u16Vec *root_get_server_ports();

    KeyStruct *root_get_server_auth_key();

    const char *root_get_config_string();

    int root_save_config(const char *path);

    // Logging calls

    int root_trace(const char *message);

    int root_debug(const char *message);

    int root_info(const char *message);

    int root_success(const char *message);

    int root_warning(const char *message);

    int root_error(const char *message);

    int root_critical(const char *message);

    int root_fatal(const char *message);

    int root_exception(const char *message);
}
