using FastLogging;

using var logging = new Logging(Levels.Debug, "root");
logging.AddWriterConfig(new ConsoleWriterConfig(Levels.Debug, colors: true));
using var threadLogger = new Logger(Levels.Debug, "LoggerThread", 1, 1);
logging.AddLogger(threadLogger);

var worker = Task.Run(() =>
{
    threadLogger.Trace("Trace Message");
    threadLogger.Debug("Debug Message");
    threadLogger.Info("Info Message");
    threadLogger.Success("Success Message");
    threadLogger.Warning("Warning Message");
    threadLogger.Error("Error Message");
    threadLogger.Fatal("Fatal Message");
});

logging.Trace("Trace Message");
logging.Debug("Debug Message");
logging.Info("Info Message");
logging.Success("Success Message");
logging.Warn("Warning Message");
logging.Error("Error Message");
logging.Fatal("Fatal Message");
await worker;
logging.SyncAll(2.0);
