using FastLogging;

static class FastLoggingTests
{
    private static int _passed;

    private static void Check(bool condition, string message)
    {
        if (!condition)
            throw new InvalidOperationException(message);
    }

    private static void TestLevelConstants()
    {
        Check(Levels.NotSet == 0 && Levels.Trace == 5 && Levels.Debug == 10, "basic levels");
        Check(Levels.Info == 20 && Levels.Success == 25 && Levels.Warning == 30, "mid levels");
        Check(Levels.Error == 40 && Levels.Critical == 50 && Levels.Exception == 60, "high levels");
        Check(Levels.Fatal == Levels.Critical && Levels.Warn == Levels.Warning, "level aliases");
    }

    private static void TestDefaultLogging()
    {
        using var logging = Logging.Default();
        Check(logging.Handle != IntPtr.Zero, "default logging handle");
        logging.Trace("trace");
        logging.Debug("debug");
        logging.Info("info");
        logging.Warning("warning");
        logging.Error("error");
        logging.Critical("critical");
        Check(logging.SyncAll(2.0) == 0, "default sync");
    }

    private static void TestFileWriterFiltering()
    {
        var path = Path.Combine(Path.GetTempPath(), $"fastlogging-{Guid.NewGuid():N}.log");
        try
        {
            using (var logging = new Logging(Levels.Debug, "test"))
            {
                Check(logging.AddWriterConfig(new FileWriterConfig(Levels.Warning, path)) == 0, "add file writer");
                logging.Debug("filtered debug");
                logging.Info("filtered info");
                logging.Error("visible error");
                Check(logging.SyncAll(2.0) == 0, "file sync");
            }
            var content = File.ReadAllText(path);
            Check(!content.Contains("filtered debug") && !content.Contains("filtered info"), "filtered messages");
            Check(content.Contains("visible error"), "visible message");
        }
        finally { File.Delete(path); }
    }

    private static void TestRotationAndLogger()
    {
        var path = Path.Combine(Path.GetTempPath(), $"fastlogging-{Guid.NewGuid():N}.log");
        try
        {
            using var logging = new Logging(Levels.Debug, "test");
            Check(logging.AddWriterConfig(new FileWriterConfig(Levels.Debug, path, backlog: 3)) == 0, "rotation writer");
            using var logger = new Logger(Levels.Debug, "worker", 1, 1);
            logging.AddLogger(logger);
            logger.Info("from logger");
            Check(logging.SyncAll(2.0) == 0, "logger sync");
            Check(logging.Rotate() == 0, "rotate");
            logging.Info("after rotate");
            Check(logging.SyncAll(2.0) == 0, "post-rotate sync");
        }
        finally { File.Delete(path); }
    }

    private static void TestCallbackWriter()
    {
        using var received = new ManualResetEventSlim();
        string? message = null;
        using var logging = new Logging(Levels.Debug, "test");
        Check(logging.AddWriterConfig(new CallbackWriterConfig(Levels.Debug, (_, _, value) =>
        {
            message = value;
            received.Set();
        })) == 0, "callback writer");
        logging.Info("callback message");
        Check(received.Wait(TimeSpan.FromSeconds(2)), "callback delivery");
        Check(message?.Contains("callback message", StringComparison.Ordinal) == true, "callback content");
    }

    public static int Main()
    {
        var tests = new (string Name, Action Run)[]
        {
            (nameof(TestLevelConstants), TestLevelConstants),
            (nameof(TestDefaultLogging), TestDefaultLogging),
            (nameof(TestFileWriterFiltering), TestFileWriterFiltering),
            (nameof(TestRotationAndLogger), TestRotationAndLogger),
            (nameof(TestCallbackWriter), TestCallbackWriter),
        };

        foreach (var test in tests)
        {
            Console.WriteLine($"=== RUN   {test.Name}");
            try
            {
                test.Run();
                _passed++;
                Console.WriteLine($"--- PASS: {test.Name}");
            }
            catch (Exception error)
            {
                Console.Error.WriteLine($"--- FAIL: {test.Name}: {error.Message}");
                return 1;
            }
        }

        Console.WriteLine($"PASS: {_passed} tests");
        return 0;
    }
}
