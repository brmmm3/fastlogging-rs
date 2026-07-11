# cfastlogging C Examples Documentation

This document describes each example in the `cfastlogging/examples` folder. Each example demonstrates a specific feature or usage pattern of the cfastlogging C API.

---

## callback.c
**Demonstrates:** Logging with a callback writer.
- Registers a C callback function to receive log messages.
- Shows how to use `callback_writer_config_new` and log at all levels.

## console.c
**Demonstrates:** Logging to the console.
- Uses a console writer with color enabled.
- Logs messages at all standard levels.

## console_add_writer.c
**Demonstrates:** Adding a writer after logger creation.
- Creates a logging instance with no writers.
- Adds a console writer at runtime using `logging_add_writer_config`.

## console_root.c
**Demonstrates:** Using the root logger API.
- Initializes the root logger and logs messages at all levels.
- Shuts down the root logger.

## default.c
**Demonstrates:** Using the default logger configuration.
- Creates a logging instance with default settings.
- Logs messages at all levels.

## ext_config.c
**Demonstrates:** Using extended configuration.
- Shows how to create and set an `ExtConfig` for advanced formatting.
- Prints out the ext_config fields.

## file.c
**Demonstrates:** Logging to a file.
- Configures a file writer with rotation and compression.
- Logs messages at all levels.

## file_add_writer.c
**Demonstrates:** Adding a file writer after logger creation.
- Creates a logging instance with no writers.
- Adds a file writer at runtime.

## get_server_addresses_ports.c
**Demonstrates:** Querying server addresses and ports.
- Sets up a server writer and queries addresses/ports using the API.
- Prints all server addresses, ports, and address:port pairs.

## get_server_configs.c
**Demonstrates:** Querying server writer configurations.
- Sets up a server writer and prints all server configs.
- Shows how to remove writers and re-query configs.

## net_unencrypted_one_client.c
**Demonstrates:** Network logging (unencrypted, one client).
- Sets up a server and a client logger.
- Logs messages from both client and server.
- Demonstrates networked log delivery.

## syslog.c
**Demonstrates:** Logging to syslog.
- Configures a syslog writer.
- Logs messages at all levels.

## threads.c
**Demonstrates:** Logging from multiple threads.
- Sets up a logger for use in a thread.
- Logs messages from both the main thread and a worker thread.

---

For build and run instructions, see the [EXAMPLES.md](../doc/EXAMPLES.md) file.
