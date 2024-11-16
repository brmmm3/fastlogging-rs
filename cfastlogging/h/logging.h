#ifndef CFASTLOGGING_LOGGING_H
#define CFASTLOGGING_LOGGING_H

#include "def.h"

CExtConfig *ext_config_new(CMessageStructEnum structured,
                           int8_t hostname,
                           int8_t pname,
                           int8_t pid,
                           int8_t tname,
                           int8_t tid);

// Logging module

void logging_init_root();

Logging logging_new_default();

Logging logging_new(uint8_t level,
                    const char *domain,
                    CWriterConfigEnum *configs_ptr , // This is a Vec<WriterConfigEnum>
                    uint32_t config_cnt,
                    CExtConfig *ext_config,
                    const char *config_path);

int logging_apply_config(Logging logging, const char *path);

int logging_shutdown(Logging logging, int8_t now);

int logging_set_level(Logging logging, uint32_t wid, uint8_t level);

void logging_set_domain(Logging logging, const char *domain);

void logging_set_level2sym(Logging logging, uint8_t level2sym);

void logging_set_ext_config(Logging logging, CExtConfig *ext_config);

void logging_add_logger(Logging logging, Logger logger);

void logging_remove_logger(Logging logging, Logger logger);

int logging_set_root_writer_config(Logging logging, CWriterConfigEnum config);

int logging_set_root_writer(Logging logging, CWriterEnum writer);

int logging_add_writer_config(Logging logging, CWriterConfigEnum config);

int logging_add_writer(Logging logging, CWriterEnum config);

int logging_remove_writer(Logging logging, uint32_t wid);

int logging_add_writer_configs(Logging logging, CWriterConfigEnum **configs, uint32_t config_cnt);

int logging_add_writers(Logging logging, CWriterEnum **writers, uint32_t writer_cnt);

CWriterEnumVec *logging_remove_writers(Logging logging, uint32_t *wids, uint32_t wid_cnt);

int logging_enable(Logging logging, uint32_t wid);

int logging_disable(Logging logging, uint32_t wid);

int logging_enable_type(Logging logging, CWriterTypeEnum typ);

int logging_disable_type(Logging logging, CWriterTypeEnum typ);

int logging_sync(Logging logging, CWriterTypeEnum *types, uint32_t type_cnt, double timeout);

int logging_sync_all(Logging logging, double timeout);

// File writer

int logging_rotate(Logging logging, const char *path);

// Network

int logging_set_encryption(Logging logging, CWriterTypeEnum writer, CEncryptionMethodEnum encryption, char *key);

// Config

void logging_set_debug(Logging logging, uint32_t debug);

const CWriterConfigEnum *logging_get_config(Logging logging, CWriterTypeEnum writer);

const CWriterConfigEnum *logging_get_writer_configs(Logging logging);

const CServerConfig *logging_get_server_config(Logging logging);

const CServerConfigs *logging_get_server_configs(Logging logging);

const char *logging_get_root_server_address_port(Logging logging);

const Cu32StringVec *logging_get_server_addresses_ports(Logging logging);

const Cu32StringVec *logging_get_server_addresses(Logging logging);

const Cu32u16Vec *logging_get_server_ports(Logging logging);

CKeyStruct *logging_get_server_auth_key(Logging logging);

const char *logging_get_config_string(Logging logging);

int logging_save_config(Logging logging, const char *path);

// Logging calls

int logging_trace(Logging logging, const char *message);

int logging_debug(Logging logging, const char *message);

int logging_info(Logging logging, const char *message);

int logging_success(Logging logging, const char *message);

int logging_warning(Logging logging, const char *message);

int logging_error(Logging logging, const char *message);

int logging_critical(Logging logging, const char *message);

int logging_fatal(Logging logging, const char *message);

int logging_exception(Logging logging, const char *message);

#endif
