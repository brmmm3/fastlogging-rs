using FastLogging;

using var logging = new Logging(Levels.Debug, "root");
logging.AddWriterConfig(new OpenTelemetryWriterConfig(Levels.Debug, "http://localhost:4318", "my-csharp-app"));
logging.Trace("Trace Message");
logging.Debug("Debug Message");
logging.Info("Info Message");
logging.Success("Success Message");
logging.Warn("Warning Message");
logging.Error("Error Message");
logging.Fatal("Fatal Message");
