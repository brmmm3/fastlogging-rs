using System.Runtime.InteropServices;
using System.Text;

namespace FastLogging;

public static class Levels
{
    public const byte NoLog = 100;
    public const byte Exception = 60;
    public const byte Critical = 50;
    public const byte Fatal = Critical;
    public const byte Error = 40;
    public const byte Warning = 30;
    public const byte Warn = Warning;
    public const byte Success = 25;
    public const byte Info = 20;
    public const byte Debug = 10;
    public const byte Trace = 5;
    public const byte NotSet = 0;
}

public enum CompressionMethod : byte { Store, Deflate, Zstd, Lzma }
public enum MessageStruct : byte { String, Json, Xml }
public enum EncryptionMethod : byte { None, AuthKey, Aes }
public enum WriterType : byte { Root, Console, File, Files, Client, Clients, Server, Servers, Syslog }

public sealed class ExtConfig
{
    internal IntPtr Handle { get; }

    public ExtConfig(MessageStruct structured, sbyte hostname, sbyte pname, sbyte pid, sbyte tname, sbyte tid)
        => Handle = Native.ext_config_new(structured, hostname, pname, pid, tname, tid);
}

public abstract class WriterConfig
{
    internal IntPtr Handle { get; }
    internal WriterConfig(IntPtr handle) => Handle = handle;
}

public sealed class ConsoleWriterConfig : WriterConfig
{
    public ConsoleWriterConfig(byte level, bool colors = false)
        : base(Native.console_writer_config_new(level, colors ? (sbyte)1 : (sbyte)0)) { }
}

public sealed class FileWriterConfig : WriterConfig
{
    public FileWriterConfig(byte level, string path, uint size = 0, uint backlog = 0,
        int timeout = -1, long time = -1, CompressionMethod compression = CompressionMethod.Store)
        : base(Create(level, path, size, backlog, timeout, time, compression)) { }

    private static IntPtr Create(byte level, string path, uint size, uint backlog,
        int timeout, long time, CompressionMethod compression)
    {
        return Native.file_writer_config_new(level, path, size, backlog, timeout, time, ref compression);
    }
}

public sealed class ClientWriterConfig : WriterConfig
{
    public ClientWriterConfig(byte level, string address)
        : base(Native.client_writer_config_new(level, address, IntPtr.Zero)) { }
}

public sealed class ServerWriterConfig : WriterConfig
{
    public ServerWriterConfig(byte level, string address)
        : base(Native.server_config_new(level, address, IntPtr.Zero)) { }
}

public delegate void LogCallback(byte level, string domain, string message);

public sealed class CallbackWriterConfig : WriterConfig
{
    internal Native.Callback NativeCallback { get; }

    public CallbackWriterConfig(byte level, LogCallback callback)
        : this(level, callback, new Native.Callback((nativeLevel, domain, message) =>
            callback(nativeLevel, Utf8(domain), Utf8(message))))
    { }

    private CallbackWriterConfig(byte level, LogCallback callback, Native.Callback nativeCallback)
        : base(Native.callback_writer_config_new(level, nativeCallback))
    {
        NativeCallback = nativeCallback;
    }

    private static string Utf8(IntPtr value) => Marshal.PtrToStringUTF8(value) ?? string.Empty;
}

public sealed class Logging : IDisposable
{
    private IntPtr _handle;
    private readonly List<Delegate> _callbacks = new();

    public Logging(byte level = Levels.NotSet, string domain = "root")
    {
        _handle = Native.logging_new(level, domain, null, UIntPtr.Zero, IntPtr.Zero, null);
        EnsureCreated();
    }

    public Logging(byte level, string domain, ExtConfig extConfig)
    {
        ArgumentNullException.ThrowIfNull(extConfig);
        _handle = Native.logging_new(level, domain, null, UIntPtr.Zero, extConfig.Handle, null);
        EnsureCreated();
    }

    public static Logging Default()
    {
        var logging = new Logging(skipCreate: true) { _handle = Native.logging_new_default() };
        logging.EnsureCreated();
        return logging;
    }

    private Logging(bool skipCreate) { }

    public IntPtr Handle => _handle;

    public int AddWriterConfig(WriterConfig config)
    {
        ArgumentNullException.ThrowIfNull(config);
        var result = Native.logging_add_writer_config(_handle, config.Handle);
        if (config is CallbackWriterConfig callback)
            _callbacks.Add(callback.NativeCallback);
        return result;
    }

    public void AddLogger(Logger logger)
    {
        ArgumentNullException.ThrowIfNull(logger);
        Native.logging_add_logger(_handle, logger.Handle);
    }

    public int SetLevel(uint writerId, byte level) => Native.logging_set_level(_handle, writerId, level);
    public void SetDomain(string domain) => Native.logging_set_domain(_handle, domain);
    public int Enable(uint writerId) => Native.logging_enable(_handle, writerId);
    public int Disable(uint writerId) => Native.logging_disable(_handle, writerId);
    public void RemoveWriter(uint writerId) => Native.logging_remove_writer(_handle, writerId);
    public int SyncAll(double timeout) => Native.logging_sync_all(_handle, timeout).ToInt32();
    public int Rotate(string? path = null) => Native.logging_rotate(_handle, path).ToInt32();
    public int SaveConfig(string path) => Native.logging_save_config(_handle, path);
    public int ApplyConfig(string path) => Native.logging_apply_config(_handle, path);
    public string? ConfigString => Utf8(Native.logging_get_config_string(_handle));

    public int Trace(string message) => Native.logging_trace(_handle, message).ToInt32();
    public int Debug(string message) => Native.logging_debug(_handle, message).ToInt32();
    public int Info(string message) => Native.logging_info(_handle, message).ToInt32();
    public int Success(string message) => Native.logging_success(_handle, message).ToInt32();
    public int Warning(string message) => Native.logging_warning(_handle, message).ToInt32();
    public int Warn(string message) => Warning(message);
    public int Error(string message) => Native.logging_error(_handle, message).ToInt32();
    public int Critical(string message) => Native.logging_critical(_handle, message).ToInt32();
    public int Fatal(string message) => Native.logging_fatal(_handle, message).ToInt32();
    public int Exception(string message) => Native.logging_exception(_handle, message).ToInt32();

    public void Dispose()
    {
        if (_handle == IntPtr.Zero)
            return;
        Native.logging_shutdown(_handle, 0);
        _handle = IntPtr.Zero;
        GC.SuppressFinalize(this);
    }

    ~Logging() => Dispose();

    private void EnsureCreated()
    {
        if (_handle == IntPtr.Zero)
            throw new InvalidOperationException("The native fastlogging instance could not be created.");
    }

    private static string? Utf8(IntPtr value) => value == IntPtr.Zero ? null : Marshal.PtrToStringUTF8(value);
}

public sealed class Logger : IDisposable
{
    private IntPtr _handle;

    public Logger(byte level, string domain) => _handle = Native.logger_new(level, domain);
    public Logger(byte level, string domain, sbyte threadName, sbyte threadId)
        => _handle = Native.logger_new_ext(level, domain, threadName, threadId);

    internal IntPtr Handle => _handle;

    public void SetLevel(byte level) => Native.logger_set_level(_handle, level);
    public void SetDomain(string domain) => Native.logger_set_domain(_handle, domain);
    public int Trace(string message) => Native.logger_trace(_handle, message);
    public int Debug(string message) => Native.logger_debug(_handle, message);
    public int Info(string message) => Native.logger_info(_handle, message);
    public int Success(string message) => Native.logger_success(_handle, message);
    public int Warning(string message) => Native.logger_warning(_handle, message);
    public int Warn(string message) => Warning(message);
    public int Error(string message) => Native.logger_error(_handle, message);
    public int Critical(string message) => Native.logger_critical(_handle, message);
    public int Fatal(string message) => Native.logger_fatal(_handle, message);
    public int Exception(string message) => Native.logger_exception(_handle, message);
    public void Dispose() => GC.SuppressFinalize(this);
}

internal static partial class Native
{
    private const string Library = "libcfastlogging";

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    internal delegate void Callback(byte level, IntPtr domain, IntPtr message);

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl, EntryPoint = "console_writer_config_new")]
    internal static extern IntPtr console_writer_config_new(byte level, sbyte colors);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl, EntryPoint = "file_writer_config_new")]
    internal static extern IntPtr file_writer_config_new(byte level, [MarshalAs(UnmanagedType.LPUTF8Str)] string path,
        uint size, uint backlog, int timeout, long time, ref CompressionMethod compression);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl, EntryPoint = "client_writer_config_new")]
    internal static extern IntPtr client_writer_config_new(byte level, [MarshalAs(UnmanagedType.LPUTF8Str)] string address, IntPtr key);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl, EntryPoint = "server_config_new")]
    internal static extern IntPtr server_config_new(byte level, [MarshalAs(UnmanagedType.LPUTF8Str)] string address, IntPtr key);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl, EntryPoint = "callback_writer_config_new")]
    internal static extern IntPtr callback_writer_config_new(byte level, Callback callback);

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl, EntryPoint = "logging_new")]
    internal static extern IntPtr logging_new(byte level, [MarshalAs(UnmanagedType.LPUTF8Str)] string domain,
        [In] IntPtr[]? configs, UIntPtr configCount, IntPtr extConfig,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string? configPath);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr ext_config_new(MessageStruct structured, sbyte hostname, sbyte pname, sbyte pid, sbyte tname, sbyte tid);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_new_default();
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logging_shutdown(IntPtr logging, sbyte now);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logging_add_writer_config(IntPtr logging, IntPtr config);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void logging_add_logger(IntPtr logging, IntPtr logger);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logging_set_level(IntPtr logging, uint writer, byte level);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void logging_set_domain(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string domain);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logging_enable(IntPtr logging, uint writer);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logging_disable(IntPtr logging, uint writer);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void logging_remove_writer(IntPtr logging, uint writer);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_sync_all(IntPtr logging, double timeout);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_rotate(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string? path);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logging_save_config(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string path);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logging_apply_config(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string path);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_get_config_string(IntPtr logging);

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_trace(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_debug(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_info(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_success(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_warning(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_error(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_critical(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_fatal(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logging_exception(IntPtr logging, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logger_new(byte level, [MarshalAs(UnmanagedType.LPUTF8Str)] string domain);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern IntPtr logger_new_ext(byte level, [MarshalAs(UnmanagedType.LPUTF8Str)] string domain, sbyte tname, sbyte tid);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void logger_set_level(IntPtr logger, byte level);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern void logger_set_domain(IntPtr logger, [MarshalAs(UnmanagedType.LPUTF8Str)] string domain);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logger_trace(IntPtr logger, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logger_debug(IntPtr logger, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logger_info(IntPtr logger, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logger_success(IntPtr logger, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logger_warning(IntPtr logger, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logger_error(IntPtr logger, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logger_critical(IntPtr logger, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logger_fatal(IntPtr logger, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)] internal static extern int logger_exception(IntPtr logger, [MarshalAs(UnmanagedType.LPUTF8Str)] string message);
}