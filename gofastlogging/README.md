# `gofastlogging`

First install `cbindgen`:

```bash
cargo install cbindgen
```

In directory `lib/gofastlogging` run to create `gofastlogging.h`:

```bash
cbindgen . -o ../gofastlogging.h
```

```bash
go mod init examples
go mod tidy
go mod vendor
```
