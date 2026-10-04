# Changelog

## [Unreleased]

## [0.9.1] - 2026-??-??

### Performance

- pyfastlogging: Cache reference to ``sys._getframe``, shared by loggers.
- pyfastlogging: Optimize `do_indent`.
- pyfastlogging: Optimize `Logging::new` and `Logger::new`.
- pyfastlogging: Remove some `println!`, I forgot to remove.
- pyfastlogging: `LevelSyms.value` use match instead of cloning.

## [0.9.0] - 2026-09-17

### Added

- Add support for OpenTelemetry.
- Add scripts for building JAR files.

### Fixed

- Fix Java builds.

## [0.8.1] - 2026-09-16

### Fixed

- Fix Cargo.toml and pyproject.toml.

## [0.8.0] - 2026-09-06

- Initial release
