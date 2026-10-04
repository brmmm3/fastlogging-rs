# Contributing

Thanks for considering a contribution to `fastlogging-rs`.

## Getting started

The repository is a Cargo workspace with one published crate (`fastlogging`) and several
bindings. `cppfastlogging` and `gofastlogging` are intentionally **not** workspace members — they
are driven by `make` instead.

```bash
cargo build                      # workspace crates
cargo test                       # unit and integration tests
cargo fmt --all --check          # formatting
cargo clippy --all-targets --all-features -- -D warnings
```

## What CI checks

The `CI` workflow runs on every push and pull request:

- `cargo fmt` and `cargo clippy` (with `-D warnings`)
- Python tests via `maturin` + `pytest` on CPython 3.10, 3.14 and PyPy 3.11
- Wheel builds for Linux (glibc and musl), Windows and macOS
- `zizmor` (GitHub Actions security linter) over `.github/`
- `rumdl` (Markdown linter) and `ruff` (Python linter)

The repository uses a `rumdl` configuration in [`.rumdl.toml`](.rumdl.toml); run `rumdl check .`
locally before opening a pull request.

## Pull requests

1. Open an issue first for larger changes so the design can be discussed.
2. Keep pull requests focused — one topic per pull request.
3. Add or update tests and documentation alongside the change. Each binding has runnable
   examples; if you change public API, update `doc/*.md` in the affected package and add an entry
   to its `CHANGELOG.md`.
4. Update the root `CHANGELOG.md` under `## [Unreleased]`.

## Documentation map

| Path | Contents |
|---|---|
| `fastlogging/doc/` | Rust reference: levels, `Logging`, `Logger`, writers, networking, config, root logger |
| `pyfastlogging/doc/` | Python reference |
| `cfastlogging/doc/`, `cppfastlogging/doc/`, `cxxfastlogging/doc/`, `gofastlogging/doc/`, `csharpfastlogging/doc/`, `jfastlogging-jni/doc/`, `jfastlogging-ffm/doc/` | Per-binding reference |
| `docs/configs/` | Example JSON / XML / YAML configuration files |
| `docs/benchmarks/` | Raw benchmark data plus the chart and HTML generators |
| `tools/readme_benchmark.py` | Reproducible benchmark used for the figures in the top-level README |

## Reporting bugs

Please include your platform, Rust (or Python/JDK/.NET/Go) version, the binding you use, and a
minimal reproduction. For a performance question, include the output of the relevant benchmark so
the comparison is meaningful.

## License

By contributing you agree that your contributions are licensed under `MIT OR Apache-2.0`,
matching the rest of the project.
