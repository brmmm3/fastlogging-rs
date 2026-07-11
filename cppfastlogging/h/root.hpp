#pragma once

#include "def.hpp"
#include "logger.hpp"
#include "writer.hpp"

extern "C" {

void root_init();
int  root_shutdown(int8_t now);
int  root_set_level(uint32_t wid, uint8_t level);
void root_set_domain(const char *domain);
void root_set_level2sym(uint8_t level2sym);
void root_set_ext_config(rust::ExtConfig *ext_config);
void root_add_logger(rust::Logger *logger);
void root_remove_logger(rust::Logger *logger);

// Config must be a Client or Server writer config.
int root_set_root_writer_config(rust::WriterConfigEnum *config);
int root_add_writer_config(rust::WriterConfigEnum *config);
int root_remove_writer(uint32_t wid);

int root_enable(uint32_t wid);
int root_disable(uint32_t wid);
int root_enable_type(rust::WriterTypeEnum typ);
int root_disable_type(rust::WriterTypeEnum typ);
int root_sync(rust::WriterTypeEnum *types, uint32_t type_cnt, double timeout);
int root_sync_all(double timeout);
int root_rotate(const char *path);
int root_set_encryption(uint32_t wid, const rust::KeyStruct *key);

void root_set_debug(uint32_t debug);

rust::WriterConfigEnum *root_get_writer_config(uint32_t wid);
rust::ServerConfig     *root_get_server_config(uint32_t wid);
rust::ServerConfigs    *root_get_server_configs();
const char             *root_get_root_server_address_port();
const rust::Cu32StringVec *root_get_server_addresses_ports();
const rust::Cu32StringVec *root_get_server_addresses();
const rust::Cu32u16Vec    *root_get_server_ports();
rust::KeyStruct        *root_get_server_auth_key();
const char             *root_get_config_string();
int root_save_config(const char *path);

int root_trace(const char *message);
int root_debug(const char *message);
int root_info(const char *message);
int root_success(const char *message);
int root_warning(const char *message);
int root_error(const char *message);
int root_critical(const char *message);
int root_fatal(const char *message);
int root_exception(const char *message);

} // extern "C"
