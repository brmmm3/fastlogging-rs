# `fastlogging-rs`

`fastlogging-rs` is a very fast and versatile logging framework. It supports the following programming languages with very similar APIs:

- [Rust](fastlogging/README.md) (of course, as it is written in Rust ;-) )
- [Python](pyfastlogging/README.md) >=3.7
- [C](cfastlogging/README.md)
- [C++](cppfastlogging/README.md)
- [Go](gofastlogging/README.md)
- [Java](jfastlogging/README.md)

## Writers

Writers are sinks for the logging data. Following writers are available:

- Console (optional colored)
- File (optional rotation and compression)
- TCP client (optional authentication key and AES encryption)
- Syslog (Linux), EventLog (Windows)
- Callback

All writers are running in background threads. So the speed/slowness of the writers don't slow down the application.

## Threads

Logging calls are thread safe.

## Processes

`fastlogging-rs` supports logging from sub processes to the main process automatically. So if a sub process logs messages then theses messages are forwarded to the main process. This also works with higher nesting levels.

## Configuration

As an alterantive through API calls configuration can be done through a configuration file. The configuration file can be a JSON, XML or YAML file.

## Benchmarks

To give you an idea how fast this module is some benchmarks here:

### Writing to a file

```text
Python logging 29.37s
log4j          1.48s
fastlogging-rs 0.2s
```

### Rotating file logging

```text
Python logging 35.24s
jog4j          1.56s
fastlogging-rs 0.17s
```

More benchmarks can be found in `doc/benchmarks`.
