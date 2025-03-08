# `fastlogging-rs`

`fastlogging-rs` is a very fast and versatile logging module. It supports the following programming languages with very similar APIs:

- Rust (of course, as it is written in Rust ;-) )
- Python >=3.7
- C
- C++
- Go
- Java

## Writers

Writers are sinks for the logging data. Following writers are available:

- Console (optional colored)
- File (optional rotation and compression)
- TCP client (optional authentication key and AES encryption)
- Syslog (Linux), EventLog (Windows)
- Callback

## Configuration

As an alterantive through API calls configuration can be done through a configuration file. The configuration file can be a JSON, XML or YAML file.

## Benchmarks

To give you an idea how fast this module is some benchmarks here:

### Writing to a file

> Python logging 29.37s
> log4j 1.48s
> fastlogging-rs 0.2s

### Rotating file logging

> Python logging 35.24s
> jog4j 1.56s
> fastlogging-rs 0.17s

More benchmarks can be found in `doc/benchmarks`.
