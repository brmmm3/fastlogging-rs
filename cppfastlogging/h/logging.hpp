#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <new>
#include <vector>

using namespace std;

#include "def.hpp"
#include "logger.hpp"
#include "writer.hpp"

namespace rust
{
    /// Forward-declaration of opaque type to use as pointer to the Rust object.
    struct Logging;
} // namespace logging::rust

extern "C"
{
    rust::ExtConfig *ext_config_new(rust::MessageStructEnum structured,
                                    int8_t hostname,
                                    int8_t pname,
                                    int8_t pid,
                                    int8_t tname,
                                    int8_t tid);

    rust::Logging *logging_new_default();

    /// For further reading ...
    /// #[no_mangle] - // https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098
    rust::Logging *logging_new(uint8_t level,
                               const char *domain,
                               rust::WriterConfigEnum *configs_ptr,
                               uint config_cnt,
                               rust::ExtConfig *ext_config,
                               const char *config_path);

    int logging_apply_config(rust::Logging *logging, const char *path);

    int logging_shutdown(rust::Logging *logging, uint8_t now);

    int logging_set_level(rust::Logging *logging, uint8_t level);

    void logging_set_domain(rust::Logging *logging, const char *domain);

    void logging_set_level2sym(rust::Logging *logging, uint8_t level2sym);

    void logging_set_ext_config(rust::Logging *logging, rust::ExtConfig *ext_config);

    void logging_add_logger(rust::Logging *logging, rust::Logger *logger);

    void logging_remove_logger(rust::Logging *logging, rust::Logger *logger);

    int logging_set_root_writer_config(rust::Logging *logging, rust::WriterConfigEnum *config);

    int logging_set_root_writer(rust::Logging *logging, rust::WriterEnum writer);

    int logging_add_writer_config(rust::Logging *logging, rust::WriterConfigEnum *config);

    int logging_add_writer(rust::Logging *logging, rust::WriterEnum *writer);

    void logging_remove_writer(rust::Logging *logging, uint32_t wid);

    int logging_add_writer_configs(rust::Logging *logging, rust::WriterConfigEnums *configs);

    int logging_add_writers(rust::Logging *logging, rust::WriterEnums *writers);

    WriterEnumVec *logging_remove_writers(rust::Logging *logging, uint32_t *wids, uint32_t wid_cnt);

    int logging_enable(rust::Logging *logging, uint32_t wid);

    int logging_disable(rust::Logging *logging, uint32_t wid);

    int logging_enable_type(rust::Logging *logging, rust::WriterTypeEnum typ);

    int logging_disable_type(rust::Logging *logging, rust::WriterTypeEnum typ);

    intptr_t logging_sync(rust::Logging *logging, rust::WriterTypeEnums *types, double timeout);

    intptr_t logging_sync_all(rust::Logging *logging, double timeout);

    // File writer

    intptr_t logging_rotate(rust::Logging *logging, const char *path);

    // Network

    intptr_t logging_set_encryption(rust::Logging *logging, rust::WriterTypeEnum writer, rust::EncryptionMethodEnum encryption, char *key);

    // Config

    void logging_set_debug(rust::Logging *logging, uint32_t debug);

    rust::WriterConfigEnum *logging_get_writer_config(rust::Logging *logging, uint32_t wid);

    rust::WriterConfigEnums *logging_get_writer_configs(rust::Logging *logging);

    rust::ServerConfig *logging_get_server_config(rust::Logging *logging);

    rust::ServerConfigs *logging_get_server_configs(rust::Logging *logging);

    const char *logging_get_root_server_address_port(rust::Logging *logging);

    const Cu32StringVec_t *logging_get_server_addresses_ports(rust::Logging *logging);

    const Cu32StringVec_t *logging_get_server_addresses(rust::Logging *logging);

    const Cu32u16Vec_t *logging_get_server_ports(rust::Logging *logging);

    rust::KeyStruct *logging_get_server_auth_key(rust::Logging *logging);

    const char *logging_get_config_string(rust::Logging *logging);

    int logging_save_config(rust::Logging *logging, const char *path);

    // Logging calls

    intptr_t logging_trace(const rust::Logging *logging, const char *message);

    intptr_t logging_debug(const rust::Logging *logging, const char *message);

    intptr_t logging_info(const rust::Logging *logging, const char *message);

    intptr_t logging_success(const rust::Logging *logging, const char *message);

    intptr_t logging_warning(const rust::Logging *logging, const char *message);

    intptr_t logging_error(const rust::Logging *logging, const char *message);

    intptr_t logging_critical(const rust::Logging *logging, const char *message);

    intptr_t logging_fatal(const rust::Logging *logging, const char *message);

    intptr_t logging_exception(const rust::Logging *logging, const char *message);
} // extern "C"

namespace logging
{
    enum class MessageStruct: uint8_t
    {
        String = 0,
        Json = 1,
        Xml = 2
    };

    class ExtConfig
    {
    public:
        rust::ExtConfig *config = NULL;

        ExtConfig(MessageStruct structured,
                  int8_t hostname,
                  int8_t pname,
                  int8_t pid,
                  int8_t tname,
                  int8_t tid)
        {
            rust::MessageStructEnum structured_enum = rust::MessageStructEnum::String;
            switch (structured) {
                case MessageStruct::String:
                    // Already handled above
                    break;
                case MessageStruct::Json:
                    structured_enum = rust::MessageStructEnum::Json;
                    break;
                case MessageStruct::Xml:
                    structured_enum = rust::MessageStructEnum::Xml;
            }
            config = ext_config_new(structured_enum, hostname, pname, pid, tname, tid);
        }

        ~ExtConfig()
        {
            config = NULL;
        }
    };

    class Logging
    {
        rust::Logging *logging = NULL;

    public:
        Logging(uint8_t level = NOTSET,
                const char *domain = NULL,
                WriterConfig configs[] = NULL,
                ExtConfig *ext_config = NULL,
                const char *config_path = NULL)
        {
            if (domain == NULL) {
                domain = "root";
            }
            uint32_t config_cnt = 0;
            if (configs != NULL) {
                config_cnt = sizeof(*configs);
                if (config_cnt > 0) {
                    config_cnt /= sizeof(configs[0]);
                }
            }
            rust::ExtConfig *ext_config_ptr = NULL;
            if (ext_config != NULL) {
                ext_config_ptr = ext_config->config;
            }
            logging = logging_new(level,
                                  domain,
                                  NULL,
                                  0,
                                  ext_config_ptr,
                                  config_path);
            //rust::WriterConfigEnum *configs_ptr = (rust::WriterConfigEnum *)malloc(config_cnt * sizeof(rust::WriterConfigEnum *));
            for (uint32_t i = 0; i < config_cnt; i++) {
                //configs_ptr[i] = configs[i].config;
                logging_add_writer_config(logging, configs[i].config);
            }
        }

        ~Logging()
        {
            logging_shutdown(logging, 0);
            logging = NULL;
        }

        void _intern_set_logging(rust::Logging *logging)
        {
            this->logging = logging;
        }

        int apply_config(const char *path)
        {
            return logging_apply_config(logging, path);
        }

        int shutdown(bool now)
        {
            return logging_shutdown(logging, (uint8_t)now);
        }

        void set_level(uint8_t level)
        {
            logging_set_level(logging, level);
        }

        void set_domain(char *domain)
        {
            logging_set_domain(logging, domain);
        }

        void set_level2sym(uint8_t level2sym)
        {
            logging_set_level2sym(logging, level2sym);
        }

        void set_ext_config(ExtConfig *ext_config)
        {
            logging_set_ext_config(logging, ext_config->config);
        }

        void add_logger(Logger *logger)
        {
            logging_add_logger(logging, logger->logger);
        }

        void remove_logger(Logger *logger)
        {
            logging_remove_logger(logging, logger->logger);
        }

        void set_root_writer_config(WriterConfig *writer)
        {
            logging_set_root_writer_config(logging, writer->config);
        }

        int add_writer_config(WriterConfig *config)
        {
            return logging_add_writer_config(logging, config);
        }

        int add_writer(rust::WriterEnum *writer)
        {
            return logging_add_writer(logging, writer);
        }

        void remove_writer(uint32_t wid)
        {
            logging_remove_writer(logging, wid);
        }

        int add_writer_configs(rust::WriterConfigEnum **configs, uint32_t config_cnt)
         {
            return logging_add_writer_configs(logging, configs, config_cnt);
        }

        int add_writers(rust::WriterEnum **writers, uint32_t writer_cnt)
        {
            return logging_add_writers(logging, writers, writer_cnt);
        }

        int remove_writers(uint32_t *wids, uint32_t wid_cnt)
        {
            return logging_remove_writers(logging, wids, wid_cnt);
        }

        int enable(uint32_t wid)
        {
            return logging_enable(logging, wid);
        }

        int disable(uint32_t wid) {
            return logging_disable(logging, wid);
        }

        int enable_type(rust::WriterTypeEnum typ) {
            return logging_enable_type(logging, typ);
        }

        int disable_type(rust::WriterTypeEnum typ) {
            return logging_disable_type(logging, typ);
        }

        int sync(rust::WriterTypeEnum *types, uint32_t type_cnt, double timeout)
        {
            return logging_sync(logging, types, type_cnt, timeout);
        }

        int sync_all(double timeout)
        {
            return logging_sync_all(logging, timeout);
        }

        // File writer

        int rotate(const char *path)
        {
            return logging_rotate(logging, path);
        }

        // Network

        int set_encryption(rust::WriterTypeEnum writer, rust::EncryptionMethodEnum encryption, char *key)
        {
            return logging_set_encryption(logging, writer, encryption, key);
        }

        // Config

        void set_debug(uint8_t debug)
        {
            logging_set_debug(debug);
        }

        rust::WriterConfigEnum *get_writer_config(uint32_t wid)
        {
            return logging_get_writer_config(logging, wid);
        }

        rust::WriterConfigEnums *get_writer_configs()
        {
            return logging_get_writer_configs(logging);
        }

        rust::ServerConfig *get_server_config(uint32_t wid)
        {
            return logging_get_server_config(logging, wid);
        }

        rust::ServerConfigs *get_server_configs()
        {
            return logging_get_server_configs(logging);
        }

        const char *get_root_server_address_port()
        {
            return logging_get_root_server_address_port(logging);
        }

        const Cu32StringVec_t *get_server_addresses_ports()
        {
            return logging_get_server_addresses_ports(logging);
        }

        const Cu32StringVec_t *get_server_addresses()
        {
            return logging_get_server_addresses(logging);
        }

        const Cu32u16Vec_t *get_server_ports()
        {
            return logging_get_server_ports(logging);
        }

        rust::KeyStruct *get_server_auth_key()
        {
            return logging_get_server_auth_key(logging);
        }

        const char *get_config_string()
        {
            return logging_get_config_string(logging);
        }

        int save_config(const char *path)
        {
            return logging_save_config(logging, path);
        }

        // Logging calls

        int trace(std::string message)
        {
            return logging_trace(logging, message.c_str());
        }

        int debug(std::string message)
        {
            return logging_debug(logging, message.c_str());
        }

        int info(std::string message)
        {
            return logging_info(logging, message.c_str());
        }

        int success(std::string message)
        {
            return logging_success(logging, message.c_str());
        }

        int warn(std::string message)
        {
            return logging_warning(logging, message.c_str());
        }

        int warning(std::string message)
        {
            return logging_warning(logging, message.c_str());
        }

        int error(std::string message)
        {
            return logging_error(logging, message.c_str());
        }

        int critical(std::string message)
        {
            return logging_critical(logging, message.c_str());
        }

        int fatal(std::string message)
        {
            return logging_fatal(logging, message.c_str());
        }

        int exception(std::string message)
        {
            return logging_exception(logging, message.c_str());
        }
    };

    Logging Default()
    {
        Logging logging = Logging();
        logging._intern_set_logging(logging_new_default());
        return logging;
    }
}
