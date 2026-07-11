# cppfastlogging C++ Examples Documentation

This document describes the example programs in the `cppfastlogging/examples/` folder. Each example demonstrates a specific feature or usage pattern of the cppfastlogging C++ API.

---

## Example List

### callback.cpp
Demonstrates using a callback writer to handle log messages in user code.
- Registers a C++ callback function to receive log messages.
- Shows all log levels.

### console.cpp
Demonstrates logging to the console using the ConsoleWriterConfig.
- Adds a console writer with log level DEBUG.
- Shows all log levels.

### console_add_writer.cpp
Demonstrates adding a console writer to a Logging instance.
- Equivalent to console.cpp; shows how to add writers dynamically.

### console_static.cpp
Demonstrates static initialization of a console writer.
- Adds a console writer with log level DEBUG.
- Shows all log levels.

### default.cpp
Demonstrates using the default Logging instance.
- Uses `Logging::Default()` to get a default logger.
- Shows all log levels.

### ext_config.cpp
Demonstrates configuring extended logging options.
- Creates an `ExtConfig` with various options (structured, hostname, pname, pid, tname, tid).
- Prints the configuration fields.

### file.cpp
Demonstrates logging to a file.
- Adds a file writer with log level DEBUG.
- Shows all log levels.

### file_add_writer.cpp
Demonstrates adding a file writer to a Logging instance.
- Equivalent to file.cpp; shows how to add writers dynamically.

### get_server_addresses_ports.cpp
Demonstrates retrieving server addresses and ports from a Logging instance.
- Shows how to access server addresses, ports, and address:port pairs.

### net_unencrypted_one_client.cpp
Demonstrates server and client logging over the network (unencrypted).
- Sets up a server with console and file writers.
- Sets up a client that sends logs to the server.
- Shows how to retrieve server address and authentication key.

### syslog.cpp
Demonstrates logging to syslog.
- Adds a syslog writer with log level DEBUG.
- Shows all log levels.

### threads.cpp
Demonstrates multi-threaded logging.
- Creates a logger and logs from both the main thread and a separate thread.
- Shows all log levels.

---

For code details, see each example source file in `cppfastlogging/examples/`.
