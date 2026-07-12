# Example programs

The `cppfastlogging/examples/` directory contains small, self-contained
programs demonstrating the main features of the C++ wrapper. All examples
include `h/cppfastlogging.hpp` and link against `libcfastlogging.so`.

## console.cpp

Demonstrates console logging with colored output. Creates a `Logging` instance
at `DEBUG` level, adds a `ConsoleWriterConfig` with colors enabled, and emits
one message at each log level.

Key API features:

- `Logging(uint8_t level, const char *domain)` constructor
- `add_writer_config(ConsoleWriterConfig(DEBUG, true))`
- Log methods: `trace`, `debug`, `info`, `success`, `warn`, `error`, `fatal`

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    Logging logging(DEBUG, "root");
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, true));
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
```

Build:

```sh
make build      # release
make build-debug # debug
```

## console_static.cpp

Same as `console.cpp` but uses the default `Logging` constructor (no
arguments). This creates a logger with default level/domain and no writers;
the console writer is added afterwards.

Key API features:

- `Logging()` default constructor
- `add_writer_config()` after construction

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    Logging logging;
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, true));
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
```

Build:

```sh
make build      # release
make build-debug # debug
```

## console_add_writer.cpp

Identical to `console_static.cpp`. It demonstrates that writers can be added
after a `Logging` instance is constructed with the default constructor.

Key API features:

- `Logging()` default constructor
- `add_writer_config()` after construction

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    Logging logging;
    logging.add_writer_config(ConsoleWriterConfig(DEBUG, true));
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
```

Build:

```sh
make build      # release
make build-debug # debug
```

## default.cpp

Demonstrates `Logging::Default()`, which returns a logger pre-configured with a
console writer at `DEBUG` level. No additional writer setup is needed.

Key API features:

- `Logging::Default()` static factory
- Log methods at all levels

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    Logging logging = Logging::Default();
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
```

Build:

```sh
make build      # release
make build-debug # debug
```

## file.cpp

Demonstrates file logging with rotation. The `FileWriterConfig` is configured
with a path, a maximum file size (`1024` bytes), and a backlog of `3` rotated
files.

Key API features:

- `FileWriterConfig(DEBUG, path, size, backlog)`
- Log methods at all levels

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    Logging logging(DEBUG, "root");
    logging.add_writer_config(
        FileWriterConfig(DEBUG, "/tmp/cppfastlogging.log", 1024, 3));
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
```

Build:

```sh
make build      # release
make build-debug # debug
```

## file_add_writer.cpp

Same as `file.cpp` but starts from `Logging::Default()` and adds the file
writer afterwards, showing that writers can be mixed on a default-configured
logger.

Key API features:

- `Logging::Default()`
- `add_writer_config(FileWriterConfig(...))` after construction

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    Logging logging = Logging::Default();
    logging.add_writer_config(
        FileWriterConfig(DEBUG, "/tmp/cppfastlogging.log", 1024, 3));
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
```

Build:

```sh
make build      # release
make build-debug # debug
```

## syslog.cpp

Demonstrates syslog logging (Unix only). The `SyslogWriterConfig` takes a
hostname, a process name, and a pid.

Key API features:

- `SyslogWriterConfig(DEBUG, hostname, pname, pid)`

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    Logging logging(DEBUG, "root");
    logging.add_writer_config(
        SyslogWriterConfig(DEBUG, "hostname", "pname", 1234));
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
```

Build:

```sh
make build      # release
make build-debug # debug
```

## callback.cpp

Demonstrates a custom callback writer. A plain C function is invoked for each
log message with the level, domain, and message text.

Key API features:

- `CallbackWriterConfig(DEBUG, callback)` — the callback has signature
  `void(uint8_t level, const char *domain, const char *message)`

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

void writer_callback(uint8_t level, const char *domain, const char *message)
{
    printf("MAIN C-CB %d %s: %s\n", level, domain, message);
}

int main(void)
{
    Logging logging(DEBUG, "root");
    logging.add_writer_config(CallbackWriterConfig(DEBUG, writer_callback));
    logging.trace("Trace Message");
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.success("Success Message");
    logging.warn("Warning Message");
    logging.error("Error Message");
    logging.fatal("Fatal Message");
    return 0;
}
```

Build:

```sh
make build      # release
make build-debug # debug
```

## threads.cpp

Demonstrates multi-threaded logging. A separate `Logger` is created and
registered with the `Logging` instance, then a pthread runs log calls on that
logger while the main thread logs concurrently.

Key API features:

- `ExtConfig(MessageStruct::String, hostname, pname, pid, tname, tid)`
- Array-of-writers constructor: `Logging(level, domain, configs[], &ext_config)`
- `Logger(level, domain, tname, tid)` extended constructor
- `add_logger(Logger *)`
- Concurrent logging from multiple threads

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>
#include <pthread.h>

using namespace logging;

void *loggerThreadFun(void *vargp)
{
    Logger *logger = static_cast<Logger *>(vargp);
    logger->trace("Trace Message");
    logger->debug("Debug Message");
    logger->info("Info Message");
    logger->success("Success Message");
    logger->warning("Warning Message");
    logger->error("Error Message");
    logger->fatal("Fatal Message");
    return nullptr;
}

int main(void)
{
    pthread_t thread_id;
    ExtConfig ext_config(MessageStruct::String, 1, 1, 1, 1, 1);
    WriterConfig configs[] = { ConsoleWriterConfig(DEBUG, true) };
    Logging *logging = new Logging(DEBUG, "root", configs, &ext_config);
    Logger  *logger  = new Logger(DEBUG, "LoggerThread", 1, 1);
    logging->add_logger(logger);
    pthread_create(&thread_id, nullptr, loggerThreadFun, static_cast<void *>(logger));
    logging->trace("Trace Message");
    logging->debug("Debug Message");
    logging->info("Info Message");
    logging->success("Success Message");
    logging->warn("Warning Message");
    logging->error("Error Message");
    logging->fatal("Fatal Message");
    pthread_join(thread_id, nullptr);
    delete logging;
    delete logger;
    return 0;
}
```

Build (note the additional `-lpthread`):

```sh
make build      # release
make build-debug # debug
```

## net_unencrypted_one_client.cpp

Demonstrates an unencrypted network logging setup with one server and one
client in the same process. The server adds a console writer, a file writer,
and a `ServerConfig`, then sets the server config as the root writer. The
client retrieves the server address:port and auth key, creates a
`ClientWriterConfig`, and sends log messages over the network.

Key API features:

- `ServerConfig(DEBUG, address)`
- `set_root_writer_config(ServerConfig &)`
- `get_root_server_address_port()`
- `get_server_auth_key()`
- `ClientWriterConfig(DEBUG, address_port, key)`
- `sync_all(timeout)`

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    // Server
    Logging *logging_server = new Logging(DEBUG, "LOGSRV");
    logging_server->add_writer_config(ConsoleWriterConfig(DEBUG, true));
    logging_server->add_writer_config(
        FileWriterConfig(DEBUG, "/tmp/cfastlogging.log", 1024, 3));
    ServerConfig srv(DEBUG, "127.0.0.1");
    logging_server->add_writer_config(srv);
    logging_server->set_root_writer_config(srv);
    logging_server->sync_all(5.0);

    // Client
    const char *address_port = logging_server->get_root_server_address_port();
    printf("address=%s\n", address_port ? address_port : "(null)");
    rust::KeyStruct *key = logging_server->get_server_auth_key();

    Logging *logging_client = new Logging(DEBUG, "LOGCLIENT");
    logging_client->add_writer_config(ClientWriterConfig(DEBUG, address_port, key));

    printf("Send logs\n");
    logging_client->trace("Trace Message");
    logging_client->debug("Debug Message");
    logging_client->info("Info Message");
    logging_client->success("Success Message");
    logging_client->warning("Warning Message");
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
```

Build:

```sh
make build      # release
make build-debug # debug
```

## get_server_addresses_ports.cpp

Demonstrates how to retrieve server address and port information after a
server writer has been configured and synced. The returned structs
(`Cu32u16Vec_t`, `Cu32StringVec_t`) are parallel arrays keyed by writer id.

Key API features:

- `ServerConfig` + `set_root_writer_config`
- `get_server_ports()` → `Cu32u16Vec_t *` (`cnt`, `keys[]`, `values[]`)
- `get_server_addresses()` → `Cu32StringVec_t *`
- `get_server_addresses_ports()` → `Cu32StringVec_t *`
- Iterating the returned vectors

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    Logging logging_server(DEBUG, "LOGSRV");
    logging_server.add_writer_config(ConsoleWriterConfig(DEBUG, true));
    ServerConfig srv(DEBUG, "127.0.0.1");
    logging_server.add_writer_config(srv);
    logging_server.set_root_writer_config(srv);
    logging_server.sync_all(5.0);

    const Cu32u16Vec_t *ports = logging_server.get_server_ports();
    if (ports) {
        printf("ports->cnt=%d\n", ports->cnt);
        for (uint32_t i = 0; i < ports->cnt; i++) {
            printf("ports->key[%u]=%u\n",   i, ports->keys[i]);
            printf("ports->value[%u]=%u\n", i, ports->values[i]);
        }
    }
    const Cu32StringVec_t *addresses = logging_server.get_server_addresses();
    if (addresses) {
        printf("addresses->cnt=%d\n", addresses->cnt);
        for (uint32_t i = 0; i < addresses->cnt; i++) {
            printf("addresses->key[%u]=%u\n",   i, addresses->keys[i]);
            printf("addresses->value[%u]=%s\n", i, addresses->values[i]);
        }
    }
    const Cu32StringVec_t *ap = logging_server.get_server_addresses_ports();
    if (ap) {
        printf("addresses_ports->cnt=%d\n", ap->cnt);
        for (uint32_t i = 0; i < ap->cnt; i++) {
            printf("addresses_ports->key[%u]=%u\n",   i, ap->keys[i]);
            printf("addresses_ports->value[%u]=%s\n", i, ap->values[i]);
        }
    }
    logging_server.info("Info Message");
    logging_server.sync_all(1.0);
    printf("-------- Finished --------\n");
    return 0;
}
```

Build:

```sh
make build      # release
make build-debug # debug
```

## ext_config.cpp

Demonstrates the `ExtConfig` class, which controls structured message format
and which metadata fields are included in log messages. The example uses
`MessageStruct::Xml` and enables hostname, pid, and tid (disabling pname and
tname). It then prints the raw `rust::ExtConfig` fields to verify the values.

Key API features:

- `ExtConfig(MessageStruct::Xml, hostname, pname, pid, tname, tid)`
- `MessageStruct` enum: `String`, `Json`, `Xml`
- Accessing the raw `rust::ExtConfig *` via `ext_config.config`

```cpp
#include "h/cppfastlogging.hpp"
#include <cstdio>

using namespace logging;

int main(void)
{
    ExtConfig ext_config(MessageStruct::Xml, 1, 0, 1, 0, 1);
    printf("config.structured=%d\n", (int)ext_config.config->structured);
    printf("config.hostname=%d\n",   ext_config.config->hostname);
    printf("config.pname=%d\n",      ext_config.config->pname);
    printf("config.pid=%d\n",        ext_config.config->pid);
    printf("config.tname=%d\n",      ext_config.config->tname);
    printf("config.tid=%d\n",        ext_config.config->tid);
    printf("-------- Finished --------\n");
    return 0;
}
```

Build:

```sh
make build      # release
make build-debug # debug
```

## Build instructions

All examples are built via the `Makefile` in the `cppfastlogging/` directory.
Run the commands from that directory:

```sh
cd cppfastlogging
```

### Release build

```sh
make build
```

This runs `cargo build --release` and then compiles every example with:

```sh
g++ -std=c++17 -I. -L../target/release -l:libcfastlogging.so
```

The `threads` example additionally links `-lpthread`.

### Debug build

```sh
make build-debug
```

This runs `cargo build` (debug profile) and compiles every example with:

```sh
g++ -std=c++17 -I. -L../target/debug -l:libcfastlogging.so
```

### Run

```sh
make run
```

This is an alias for `make build-debug`.

### Clean

```sh
make clean
```

Removes the `bin/` directory containing compiled example binaries.

### Manual build (single example)

If you want to compile a single example without the Makefile, run from the
`cppfastlogging/` directory:

```sh
# Release
g++ -std=c++17 -I. -o ./bin/console ./examples/console.cpp \
    -L../target/release -l:libcfastlogging.so

# Debug
g++ -std=c++17 -I. -o ./bin/console ./examples/console.cpp \
    -L../target/debug -l:libcfastlogging.so

# threads (add -lpthread)
g++ -std=c++17 -I. -o ./bin/threads ./examples/threads.cpp \
    -L../target/release -l:libcfastlogging.so -lpthread
```

The compiled binaries are placed in `cppfastlogging/bin/`.

> **Note:** Because the examples link dynamically against `libcfastlogging.so`,
> you may need to set `LD_LIBRARY_PATH` when running them outside the build
> directory:
>
> ```
> LD_LIBRARY_PATH=../target/release ./bin/console
> ```
