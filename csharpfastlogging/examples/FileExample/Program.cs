using FastLogging;

var path = Path.Combine(Path.GetTempPath(), "csharpfastlogging.log");
using var logging = new Logging(Levels.Debug, "root");
logging.AddWriterConfig(new FileWriterConfig(Levels.Debug, path, size: 1024, backlog: 3));
logging.Trace("Trace Message");
logging.Debug("Debug Message");
logging.Info("Info Message");
logging.Success("Success Message");
logging.Warn("Warning Message");
logging.Error("Error Message");
logging.Fatal("Fatal Message");
logging.SyncAll(2.0);
Console.WriteLine($"Wrote log to {path}");
