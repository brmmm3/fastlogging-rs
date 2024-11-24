#ifndef CFASTLOGGING_ROOT_H
#define CFASTLOGGING_ROOT_H

#include "def.h"

// Root logging module

void root_init();

int root_shutdown(int8_t now);

int root_set_level(uint32_t wid, uint8_t level);

void root_set_domain(const char *domain);

void root_set_level2sym(uint8_t level2sym);

void root_set_ext_config(CExtConfig *ext_config);

void root_add_logger(Logger logger);

void root_remove_logger(Logger logger);

int root_set_root_writer_config(CWriterConfigEnum config);

int root_set_root_writer(CWriterEnum writer);

int root_add_writer_config(CWriterConfigEnum config);

int root_add_writer(CWriterEnum config);

int root_remove_writer(uint32_t wid);

int root_add_writer_configs(CWriterConfigEnums *configs, uint32_t config_cnt);

int root_add_writers(CWriterEnums *writers, uint32_t writer_cnt);

CWriterEnums *root_remove_writers(uint32_t *wids, uint32_t wid_cnt);

int root_enable(uint32_t wid);

int root_disable(uint32_t wid);

int root_enable_type(CWriterTypeEnum typ);

int root_disable_type(CWriterTypeEnum typ);

int root_sync(CWriterTypeEnum *types, uint32_t type_cnt, double timeout);

int root_sync_all(double timeout);

// File writer

int root_rotate(const char *path);

// Network

int root_set_encryption(uint32_t wid, const CKeyStruct *key);

// Config

void root_set_debug(uint32_t debug);

const CWriterConfigEnum *root_get_writer_config(uint32_t wid);

const CWriterConfigEnums *root_get_writer_configs(Logging logging);

const CServerConfig *root_get_server_config(Logging logging);

const CServerConfigs *root_get_server_configs(Logging logging);

const char *root_get_root_server_address_port(Logging logging);

const Cu32StringVec *root_get_server_addresses_ports(Logging logging);

const Cu32StringVec *root_get_server_addresses(Logging logging);

const Cu32u16Vec *root_get_server_ports(Logging logging);

CKeyStruct *root_get_server_auth_key(Logging logging);

const char *root_get_config_string(Logging logging);

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

#endif
