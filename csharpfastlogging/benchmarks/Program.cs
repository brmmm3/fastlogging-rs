using System.Diagnostics;
using System.Text.Json;
using FastLogging;
using NLog;
using NLog.Config;
using NLog.Targets;
using Serilog;

using CflLogging = FastLogging.Logging;
using NLogLogger = NLog.Logger;
using SerilogLogger = Serilog.ILogger;

const int DefaultCount = 5000;
const int NumRounds = 10;
const int Megabyte = 1024 * 1024;
var count = args.Length > 0 && int.TryParse(args[0], out var parsed) ? parsed : DefaultCount;
var root = Directory.GetCurrentDirectory();
var outputDirectory = Directory.Exists(Path.Combine(root, "csharpfastlogging"))
    ? Path.Combine(root, "csharpfastlogging", "doc", "benchmarks")
    : Path.Combine(root, "..", "doc", "benchmarks");
Directory.CreateDirectory(outputDirectory);

var messages = new[]
{
    ("short", "Message"),
    ("long", "Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message")
};
var scenarios = new[]
{
    ("nolog", "No log file", false),
    ("file", "Log file", false),
    ("rotate", "Rotating log file", true)
};
var levelNames = new[] { "DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL" };
var levels = new byte[] { Levels.Debug, Levels.Info, Levels.Warning, Levels.Error, Levels.Critical };
var results = new Dictionary<string, Dictionary<string, ScenarioResult>>();

Console.WriteLine($"cnt: {count}");
foreach (var (messageName, message) in messages)
{
    results[messageName] = new Dictionary<string, ScenarioResult>();
    foreach (var (scenarioName, scenarioTitle, rotate) in scenarios)
    {
        var scenarioResult = new ScenarioResult { Title = scenarioTitle };
        results[messageName][scenarioName] = scenarioResult;
        for (var levelIndex = 0; levelIndex < levels.Length; levelIndex++)
        {
            Console.WriteLine($"\n### {messageName} {scenarioName} {levelNames[levelIndex]}");
            var title = $"{messageName}_{scenarioName}_{levelNames[levelIndex]}";
            var result = scenarioName == "nolog"
                ? MeasureNoFile(count, levels[levelIndex], message)
                : MeasureFile(count, levels[levelIndex], message, title, rotate);
            scenarioResult.Levels.Add(result);
            Console.WriteLine($"  csharpfastlogging: {result.CsharpFastLogging:F4} s");
            Console.WriteLine($"  serilog:            {result.Serilog:F4} s");
            Console.WriteLine($"  nlog:               {result.NLog:F4} s");
        }
    }
}

var jsonOptions = new JsonSerializerOptions { WriteIndented = true };
var jsonPath = Path.Combine(outputDirectory, "csharp_benchmark.json");
File.WriteAllText(jsonPath, JsonSerializer.Serialize(results, jsonOptions));
Console.WriteLine($"\nJSON results written to {jsonPath}");
WriteHtmlReports(outputDirectory, results, levelNames);
Console.WriteLine("HTML files written to doc/benchmarks/");

static LevelResult MeasureNoFile(int count, byte level, string message)
{
    return new LevelResult
    {
        CsharpFastLogging = Average(() => BenchCsharpNoFile(count, level, message)),
        Serilog = Average(() => BenchSerilogNoFile(count, message)),
        NLog = Average(() => BenchNLogNoFile(count, message))
    };
}

static LevelResult MeasureFile(int count, byte level, string message, string title, bool rotate)
{
    var csharpPath = BenchmarkPath(title, "csharpfastlogging");
    var serilogPath = BenchmarkPath(title, "serilog");
    var nlogPath = BenchmarkPath(title, "nlog");
    try
    {
        return new LevelResult
        {
            CsharpFastLogging = Average(() => BenchCsharpFile(count, level, message, csharpPath, rotate)),
            Serilog = Average(() => BenchSerilogFile(count, message, serilogPath, rotate)),
            NLog = Average(() => BenchNLogFile(count, message, nlogPath, rotate))
        };
    }
    finally
    {
        Cleanup(csharpPath);
        Cleanup(serilogPath);
        Cleanup(nlogPath);
    }
}

static double BenchCsharpNoFile(int count, byte level, string message)
{
    using var logging = new CflLogging(level, "bench");
    var stopwatch = Stopwatch.StartNew();
    LogMessages(logging, count, message);
    stopwatch.Stop();
    return stopwatch.Elapsed.TotalSeconds;
}

static double BenchCsharpFile(int count, byte level, string message, string path, bool rotate)
{
    Directory.CreateDirectory(Path.GetDirectoryName(path)!);
    using var logging = new CflLogging(level, "bench");
    logging.AddWriterConfig(new FileWriterConfig(level, path, rotate ? (uint)Megabyte : 0, rotate ? 8u : 0));
    var stopwatch = Stopwatch.StartNew();
    LogMessages(logging, count, message);
    stopwatch.Stop();
    logging.SyncAll(10.0);
    return stopwatch.Elapsed.TotalSeconds;
}

static void LogMessages(CflLogging logging, int count, string message)
{
    for (var i = 0; i < count; i++)
    {
        for (var round = 0; round < 4; round++)
        {
            logging.Critical($"Critical {i} {message}");
            logging.Error($"Error {i} {message}");
            logging.Warning($"Warning {message} {i}");
            logging.Info($"Info {message} {i}");
            logging.Debug($"Debug {message} {i}");
        }
    }
}

static double BenchSerilogNoFile(int count, string message)
{
    using var logger = new LoggerConfiguration().MinimumLevel.Verbose().CreateLogger();
    var stopwatch = Stopwatch.StartNew();
    LogMessagesSerilog(logger, count, message);
    stopwatch.Stop();
    return stopwatch.Elapsed.TotalSeconds;
}

static double BenchSerilogFile(int count, string message, string path, bool rotate)
{
    Directory.CreateDirectory(Path.GetDirectoryName(path)!);
    var loggerConfiguration = new LoggerConfiguration().MinimumLevel.Verbose();
    if (rotate)
        loggerConfiguration = loggerConfiguration.WriteTo.File(path, fileSizeLimitBytes: Megabyte, rollOnFileSizeLimit: true, retainedFileCountLimit: 8);
    else
        loggerConfiguration = loggerConfiguration.WriteTo.File(path);
    using var logger = loggerConfiguration.CreateLogger();
    var stopwatch = Stopwatch.StartNew();
    LogMessagesSerilog(logger, count, message);
    stopwatch.Stop();
    logger.Dispose();
    return stopwatch.Elapsed.TotalSeconds;
}

static void LogMessagesSerilog(SerilogLogger logger, int count, string message)
{
    for (var i = 0; i < count; i++)
    {
        for (var round = 0; round < 4; round++)
        {
            logger.Error("Critical {Index} {Message}", i, message);
            logger.Error("Error {Index} {Message}", i, message);
            logger.Warning("Warning {Message} {Index}", message, i);
            logger.Information("Info {Message} {Index}", message, i);
            logger.Debug("Debug {Message} {Index}", message, i);
        }
    }
}

static double BenchNLogNoFile(int count, string message)
{
    var logger = NLog.LogManager.GetLogger("bench");
    NLog.LogManager.Configuration = new LoggingConfiguration();
    var stopwatch = Stopwatch.StartNew();
    LogMessagesNLog(logger, count, message);
    stopwatch.Stop();
    NLog.LogManager.Shutdown();
    return stopwatch.Elapsed.TotalSeconds;
}

static double BenchNLogFile(int count, string message, string path, bool rotate)
{
    Directory.CreateDirectory(Path.GetDirectoryName(path)!);
    var configuration = new LoggingConfiguration();
    var target = new FileTarget("file") { FileName = path, KeepFileOpen = false, ConcurrentWrites = false };
    if (rotate)
    {
        target.ArchiveAboveSize = Megabyte;
        target.MaxArchiveFiles = 8;
        target.ArchiveNumbering = ArchiveNumberingMode.Rolling;
    }
    configuration.AddTarget(target);
    configuration.AddRule(LogLevel.Trace, LogLevel.Fatal, target);
    NLog.LogManager.Configuration = configuration;
    var logger = NLog.LogManager.GetLogger("bench");
    var stopwatch = Stopwatch.StartNew();
    LogMessagesNLog(logger, count, message);
    stopwatch.Stop();
    NLog.LogManager.Shutdown();
    return stopwatch.Elapsed.TotalSeconds;
}

static void LogMessagesNLog(NLogLogger logger, int count, string message)
{
    for (var i = 0; i < count; i++)
    {
        for (var round = 0; round < 4; round++)
        {
            logger.Error($"Critical {i} {message}");
            logger.Error($"Error {i} {message}");
            logger.Warn($"Warning {message} {i}");
            logger.Info($"Info {message} {i}");
            logger.Debug($"Debug {message} {i}");
        }
    }
}

static double Average(Func<double> benchmark)
{
    var total = 0.0;
    var rounds = 0;
    for (var round = 0; round < NumRounds; round++)
    {
        total += benchmark();
        rounds++;
        if (total > 2.0)
            break;
    }
    return rounds == 0 ? -1.0 : total / rounds;
}

static string BenchmarkPath(string title, string implementation)
{
    var directory = Path.Combine(Path.GetTempPath(), "csharpfastlogging_bench", $"{title}_{implementation}");
    Cleanup(directory);
    Directory.CreateDirectory(directory);
    return Path.Combine(directory, "logging.log");
}

static void Cleanup(string path)
{
    var directory = Directory.Exists(path) ? path : Path.GetDirectoryName(path);
    if (!string.IsNullOrEmpty(directory) && Directory.Exists(directory))
        Directory.Delete(directory, true);
}

static void WriteHtmlReports(string directory, Dictionary<string, Dictionary<string, ScenarioResult>> results, string[] levelNames)
{
    const string template = "<!DOCTYPE html>\n<html><head><meta charset=\"utf-8\"><title>C# Logging benchmark results</title><script src=\"https://www.gstatic.com/charts/loader.js\"></script><script>google.charts.load('current', {packages:['bar']}); google.charts.setOnLoadCallback(drawChart); function drawChart(){var data=google.visualization.arrayToDataTable([['Log level','csharpfastlogging','serilog','nlog'],%(ROWS)s]); var options={chart:{title:'%(TITLE)s'},bars:'horizontal'}; new google.charts.Bar(document.getElementById('barchart_material')).draw(data,google.charts.Bar.convertOptions(options));}</script><style>body{font-family:system-ui,sans-serif;margin:2rem}table{border-collapse:collapse;margin-top:1rem}th,td{border:1px solid #ccc;padding:.45rem .7rem;text-align:right}th:first-child,td:first-child{text-align:left}</style></head><body><h1>%(TITLE)s</h1><div id=\"barchart_material\" style=\"width:900px;height:500px\"></div><table><thead><tr><th>Log level</th><th>csharpfastlogging (s)</th><th>serilog (s)</th><th>nlog (s)</th></tr></thead><tbody>%(TABLE)s</tbody></table></body></html>";
    foreach (var (messageName, scenarios) in results)
    {
        foreach (var (scenarioName, scenario) in scenarios)
        {
            var rows = string.Join(",", scenario.Levels.Select((level, index) =>
                $"['{levelNames[index]}',{level.CsharpFastLogging.ToString("F6", System.Globalization.CultureInfo.InvariantCulture)},{level.Serilog.ToString("F6", System.Globalization.CultureInfo.InvariantCulture)},{level.NLog.ToString("F6", System.Globalization.CultureInfo.InvariantCulture)}]"));
            var table = string.Join("", scenario.Levels.Select((level, index) =>
                $"<tr><td>{levelNames[index]}</td><td>{level.CsharpFastLogging.ToString("F6", System.Globalization.CultureInfo.InvariantCulture)}</td><td>{level.Serilog.ToString("F6", System.Globalization.CultureInfo.InvariantCulture)}</td><td>{level.NLog.ToString("F6", System.Globalization.CultureInfo.InvariantCulture)}</td></tr>"));
            var html = template.Replace("%(ROWS)s", rows).Replace("%(TABLE)s", table).Replace("%(TITLE)s", $"{scenario.Title} - {messageName}");
            File.WriteAllText(Path.Combine(directory, $"{scenarioName}_{messageName}.html"), html);
        }
    }
}

sealed class ScenarioResult
{
    public string Title { get; set; } = string.Empty;
    public List<LevelResult> Levels { get; set; } = new();
}

sealed class LevelResult
{
    public double CsharpFastLogging { get; set; }
    public double Serilog { get; set; }
    public double NLog { get; set; }
}
