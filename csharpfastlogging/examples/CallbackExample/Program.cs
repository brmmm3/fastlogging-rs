using FastLogging;

var gate = new object();
var callback = new CallbackWriterConfig(Levels.Debug, (level, domain, message) =>
{
    lock (gate)
        Console.WriteLine($"CALLBACK {level} {domain}: {message}");
});
using var logging = new Logging(Levels.Debug, "root");
logging.AddWriterConfig(callback);
logging.Trace("Trace Message");
logging.Debug("Debug Message");
logging.Info("Info Message");
logging.Success("Success Message");
logging.Warn("Warning Message");
logging.Error("Error Message");
logging.Fatal("Fatal Message");
logging.SyncAll(2.0);
