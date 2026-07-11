# `cxxfastlogging`

C++ bindings for the [`fastlogging`](../fastlogging) crate, generated with the
[`cxx`](https://cxx.rs) crate. It exposes `Logging`, `Logger` and `WriterConfig`
(console, file, network client/server, and syslog writers) as opaque types
usable from C++, plus a set of `root_*` free functions mirroring
`fastlogging::root` for the process-wide singleton logger.

Unlike [`cfastlogging`](../cfastlogging) (raw `extern "C"` FFI) this crate uses
`cxx`'s statically type-checked bridge, so the generated C++ API is type-safe
and does not require manual memory management of raw pointers.

## Building

```sh
cargo build -p cxxfastlogging
```
