# jfastlogging (JNI) — Documentation

Java bindings for the fastlogging Rust logging library via JNI (Java Native Interface). Messages are routed through an asynchronous background thread to one or more independent writers (console, file, network client/server), keeping the hot path cheap.

## Table of Contents

- [DEF.md](doc/DEF.md) — Constants, enums, and config class definitions
- [LEVELS.md](doc/LEVELS.md) — Log level reference
- [LOGGING.md](doc/LOGGING.md) — The `Logging` class (main entry point)
- [LOGGER.md](doc/LOGGER.md) — The `Logger` inner class
- [WRITERS.md](doc/WRITERS.md) — Writer configuration classes
- [NETWORK.md](doc/NETWORK.md) — Client and server networking
- [CONFIG.md](doc/CONFIG.md) — Extended formatting configuration
- [EXAMPLES.md](doc/EXAMPLES.md) — End-to-end usage examples

## Quick Start

### Build

The native library is built with Rust/Cargo and wired into the Java side via Make targets in `jfastlogging-jni/`.

> **Requirements:** JDK 25 (the Maven project compiles with `source`/`target` 25) and a recent Maven 3.9+.

#### 1. Build the JNI shared library

```sh
cargo build --release
```

Run from the repo root or from `jfastlogging-jni/`. This produces `target/release/libjfastlogging_jni.so`.

#### 2. Copy the library into the Maven project's `lib/` directory under the expected name

   ```sh
   make lib
   ```

   This copies the `.so` to `FastLogging/lib/libjfastlogging.so`.

#### 3. Compile the Java sources

   ```sh
   make java_build
   ```

   This compiles `org/logging/FastLogging.java`.

#### 4. For debug builds, use the debug Make target and a plain (non-release) cargo build

   ```sh
   make lib-debug
   cargo build
   ```

#### 5. Build and test the Maven project

   ```sh
   mvn clean test
   ```

   Run from `jfastlogging-jni/FastLogging/`. The Maven build compiles the Java sources, runs the JUnit test suite and packages the JAR. The tests load the native library from `FastLogging/lib/` via `-Djava.library.path=${project.basedir}/lib` (already configured in the POM).

The Java source lives at `jfastlogging-jni/org/logging/FastLogging.java`. The Maven project lives at `jfastlogging-jni/FastLogging/`.

### Build a JAR

From `jfastlogging-jni/`, use the platform script to build the native library,
copy it into `FastLogging/lib/` with the name expected by
`System.loadLibrary("jfastlogging")`, and create the Maven JAR:

```sh
bash build-jar.sh
```

On Windows PowerShell:

```powershell
.\build-jar.ps1
```

The JAR is written to
`FastLogging/target/FastLogging-0.0.1-SNAPSHOT.jar`. The PowerShell script also
accepts `-SkipNativeBuild` when the native DLL has already been built.

### Minimal Console Example

```java
import org.logging.FastLogging;
import org.logging.FastLogging.ConsoleWriterConfig;
import org.logging.FastLogging.Logging;

public class Main {
    public static void main(String[] args) {
        ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
        Logging logging = new Logging(FastLogging.DEBUG, "root", console);
        logging.info("Hello from jfastlogging");
        logging.shutdown();
    }
}
```

## Architecture

All log calls on the hot path do a cheap level check, then hand the message off to a flume channel (`server_tx`). A background `LoggingThread` drains the channel and dispatches to the configured writers, each of which may run on its own thread (file rotation, network client, logging server).

```mermaid
flowchart TD
    A["Your code"] --> B["Level check\n(hot path — single\ninteger compare,\nno lock)"]
    B -->|"below threshold"| X["no-op return"]
    B -->|"passes"| C["server_tx\n(flume channel)"]
    C --> D["LoggingThread\n(background)"]
    D --> E["ConsoleWriter\n(thread)"]
    D --> F["FileWriter\n(thread)"]
    D --> G["ClientWriter\n(thread)"]
    D --> H["LoggingServer\n(thread)"]
    H --> I["Remote clients"]
    G --> J["Remote logging server"]
```

## Important Notes

- **All classes are nested inside `org.logging.FastLogging`.** There is one top-level Java class; every config class, enum, `Logging`, and `Logger` is a static nested class or enum of it.
- **Requires JDK 25.** The Maven project compiles with `maven.compiler.source`/`target` = 25 (see `FastLogging/pom.xml`).
- **The native library `libjfastlogging.so` must be on `java.library.path`.** It is loaded via `System.loadLibrary("jfastlogging")`; ensure the directory containing the `.so` is passed with `-Djava.library.path=...`. The Maven build already sets this to `${project.basedir}/lib` for tests via the Surefire plugin.
- **`Logging` constructors take individual writer config objects, not a list.** Each writer is passed as its own parameter (see [LOGGING.md](doc/LOGGING.md) for the full set of overloads).
- **Client-side level filtering happens in Java.** `Logging` methods compare the message level against `instance_level` before invoking JNI, so filtered-out messages never cross the JNI boundary.
- **`Logger` is a non-static inner class** and must be created from a `FastLogging` instance (i.e. you need a `FastLogging` instance to create a `Logger`).

## OpenTelemetry

The JNI binding supports OTLP/HTTP with `OpenTelemetryWriterConfig`. Records
are sent to `{endpoint}/v1/logs`; use `http://localhost:4318` for a local
OpenTelemetry Collector.

```java
OpenTelemetryWriterConfig otel = new OpenTelemetryWriterConfig(
   FastLogging.DEBUG, "http://localhost:4318", "my-java-jni-app");
Logging logging = new Logging(FastLogging.DEBUG, "root", otel);
logging.info("exported to OpenTelemetry");
logging.shutdown();
```

See `FastLogging/src/main/java/org/logging/examples/OtelExample.java`.

- **Syslog and Callback writers exist in the JNI/Rust layer but do not have Java wrapper classes yet.** Syslog can be partially used via the `Logging(int level, String domain, int syslog)` constructor. The callback writer has no Java wrapper.
