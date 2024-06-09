# `gofastlogging`

First install `cbindgen`:

```bash
cargo install cbindgen
```

In directory `cfastlogging` run:

```bash
cargo build --release`
```

to build `libcfastlogging.so`.

In directory `lib/gofastlogging` run:

```bash
cbindgen . -o ../gofastlogging.h
```

to create `gofastlogging.h`.

```bash
go mod init examples
go mod tidy
go mod vendor
```
