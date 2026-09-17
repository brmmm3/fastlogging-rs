# csharpfastlogging

C#/.NET wrapper for `cfastlogging`, with an API shaped like the C++ wrapper.
The wrapper uses P/Invoke over the stable C ABI and keeps native writer handles opaque.

## Build

Build the native library first from the repository root:

```powershell
cargo build -p cfastlogging
```

Then build the managed projects:

```powershell
dotnet build csharpfastlogging/FastLogging.csproj
dotnet test csharpfastlogging/tests/FastLogging.Tests.csproj
```

## Benchmark

The benchmark compares `csharpfastlogging` with Serilog and NLog across no-file,
plain-file, and rotating-file scenarios, using short and long messages at five
log levels:

```powershell
$env:Path = "$(Resolve-Path 'gofastlogging/lib');$env:Path"
dotnet run --project csharpfastlogging/benchmarks/CSharpBenchmark.csproj -- 5000
```

Pass a smaller count such as `1` for a quick smoke run. Results are written to
`csharpfastlogging/doc/benchmarks/csharp_benchmark.json` and the corresponding
HTML reports.

Set `PATH` (Windows) or `LD_LIBRARY_PATH` (Linux) so the runtime can find the
native `cfastlogging` library. The examples accept `FASTLOGGING_NATIVE_DIR` and
prepend it to the process library search path where supported.

## API

The main types are `Logging`, `Logger`, `ConsoleWriterConfig`,
`FileWriterConfig`, `CallbackWriterConfig`, and `ServerWriterConfig`. Levels are
available through `Levels`, and the `Logging` and `Logger` types implement
`IDisposable` for deterministic shutdown.

## Examples

```powershell
dotnet run --project csharpfastlogging/examples/ConsoleExample
dotnet run --project csharpfastlogging/examples/FileExample
dotnet run --project csharpfastlogging/examples/CallbackExample
dotnet run --project csharpfastlogging/examples/ThreadsExample
```

## OpenTelemetry

Configure `OpenTelemetryWriterConfig` to export logs over OTLP/HTTP. The
endpoint is written as `{endpoint}/v1/logs`; a local Collector uses
`http://localhost:4318`.

```csharp
using FastLogging;

using var logging = new Logging(Levels.Debug, "root");
logging.AddWriterConfig(new OpenTelemetryWriterConfig(
	Levels.Debug, "http://localhost:4318", "my-csharp-app"));
logging.Info("exported to OpenTelemetry");
```

The complete example is in `examples/OtelExample/Program.cs`.
