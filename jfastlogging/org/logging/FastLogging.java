package org.logging;

public class FastLogging {

    static {
        System.loadLibrary("jfastlogging");
    }

    // Log levels
    public static final int NOLOG = 100;
    public static final int EXCEPTION = 60;
    public static final int CRITICAL = 50;
    public static final int FATAL = CRITICAL;
    public static final int ERROR = 40;
    public static final int WARNING = 30;
    public static final int WARN = WARNING;
    public static final int SUCCESS = 25;
    public static final int INFO = 20;
    public static final int DEBUG = 10;
    public static final int TRACE = 5;
    public static final int NOTSET = 0;

    enum LevelSyms {
        Sym(0), Short(1), Str(2);

        private final int value;

        private LevelSyms(int value) {
            this.value = value;
        }

        public int getValue() {
            return value;
        }
    }

    static public String Level2Sym(int level) {
        switch (level) {
            case NOLOG:
                return "NOLOG";
            case EXCEPTION:
                return "EXCEPTION";
            case CRITICAL:
                return "CRITICAL";
            case ERROR:
                return "ERROR";
            case WARNING:
                return "WARNING";
            case SUCCESS:
                return "SUCCESS";
            case INFO:
                return "INFO";
            case DEBUG:
                return "DEBUG";
            case TRACE:
                return "TRACE";
            case NOTSET:
                return "NOTSET";
        }
        return "?";
    }

    public enum MessageStructEnum {
        String(0), Json(1), Xml(2);

        private final int value;

        private MessageStructEnum(int value) {
            this.value = value;
        }

        public int getValue() {
            return value;
        }
    }

    public enum WriterTypeEnum {
        Root(0), Console(1), File(2), Client(3), Server(4), Syslog(5);

        private final int value;

        private WriterTypeEnum(int value) {
            this.value = value;
        }

        public int getValue() {
            return value;
        }
    }

    public enum CompressionMethodEnum {
        Store(0), Deflate(1), Zstd(2), Lzma(3);

        private final int value;

        private CompressionMethodEnum(int value) {
            this.value = value;
        }

        public int getValue() {
            return value;
        }
    }

    public enum EncryptionMethod {
        NONE(0), AuthKey(1), AES(2);

        private final int value;

        private EncryptionMethod(int value) {
            this.value = value;
        }

        public int getValue() {
            return value;
        }
    }

    public static native long extConfigNew(int structured, boolean hostname, boolean pname, boolean pid, boolean tname,
            boolean tid);

    static public class ExtConfig {
        long instance_ptr = 0;

        public ExtConfig(MessageStructEnum structured, boolean hostname, boolean pname, boolean pid, boolean tname,
                boolean tid) {
            instance_ptr = extConfigNew(structured.getValue(), hostname, pname, pid, tname, tid);
        }
    }

    public static native long consoleWriterConfigNew(int level, boolean colors);

    static public class ConsoleWriterConfig {
        long instance_ptr = 0;

        public ConsoleWriterConfig(int level) {
            instance_ptr = consoleWriterConfigNew(level, false);
        }

        public ConsoleWriterConfig(int level, boolean colors) {
            instance_ptr = consoleWriterConfigNew(level, colors);
        }
    }

    public static native long fileWriterConfigNew(int level, String path, int size, int backlog, long timeout,
            long time, int compression);

    static public class FileWriterConfig {
        long instance_ptr = 0;

        public FileWriterConfig(int level, String path) {
            instance_ptr = fileWriterConfigNew(level, path, 0, 0, 0, 0, 0);
        }

        public FileWriterConfig(int level, String path, int size, int backlog, long timeout, long time,
                CompressionMethodEnum compression) {
            instance_ptr = fileWriterConfigNew(level, path, size, backlog, timeout, time, compression.getValue());
        }
    }

    public static native long clientWriterConfigNew(int level, String address, int port, int method, String key);

    static public class ClientWriterConfig {
        long instance_ptr = 0;

        public ClientWriterConfig(int level, String address, int port) {
            instance_ptr = clientWriterConfigNew(level, address, port, 0, null);
        }

        public ClientWriterConfig(int level, String address, int port, EncryptionMethod method, String key) {
            instance_ptr = clientWriterConfigNew(level, address, port, method.getValue(), key);
        }
    }

    public static native long serverConfigNew(int level, String address, int port, int method, String key);

    static public class ServerConfig {
        long instance_ptr = 0;

        public ServerConfig(int level, String address, int port) {
            instance_ptr = serverConfigNew(level, address, port, 0, null);
        }

        public ServerConfig(int level, String address, int port, EncryptionMethod method, String key) {
            instance_ptr = serverConfigNew(level, address, port, method.getValue(), key);
        }
    }

    // Logging class

    public static native long loggingNew(int level, String domain, long extConfig, long console, long file, long server,
            long client, int syslog, String config);

    private static native void loggingShutdown(long instance_ptr, boolean now);

    public static native void loggingSetLevel(long instance_ptr, int writer, String key, int level);

    public static native void loggingSetDomain(long instance_ptr, String domain);

    public static native void loggingSetLevel2Sym(long instance_ptr, int level2sym);

    public static native void loggingSetExtConfig(long instance_ptr, long extConfig);

    private static native void loggingAddLogger(long instance_ptr, long logger_ptr);

    private static native void loggingRemoveLogger(long instance_ptr, long logger_ptr);

    private static native void loggingAddWriter(long instance_ptr, long writer_ptr);

    private static native void loggingRemoveWriter(long instance_ptr, int writer, String key);

    public static native void loggingSync(long instance_ptr, boolean console, boolean file, boolean client,
            boolean syslog, double timeout);

    public static native void loggingSyncAll(long instance_ptr, double timeout);

    // File logger

    public static native void loggingRotate(long instance_ptr, String path);

    // Network

    public static native void loggingSetEncryption(long instance_ptr, String address, int method, String key);

    // Config

    private static native String loggingGetConfig(long instance_ptr, int writer, String key);

    private static native ServerConfig loggingGetServerConfig(long instance_ptr);

    private static native String loggingGetServerAddress(long instance_ptr);

    private static native String loggingGetServerAuthKey(long instance_ptr);

    private static native String loggingGetConfigString(long instance_ptr);

    private static native void loggingSaveConfig(long instance_ptr, String path);

    // Logging methods

    private static native void loggingTrace(long instance_ptr, String message);

    private static native void loggingDebug(long instance_ptr, String message);

    private static native void loggingInfo(long instance_ptr, String message);

    private static native void loggingSuccess(long instance_ptr, String message);

    private static native void loggingWarning(long instance_ptr, String message);

    private static native void loggingError(long instance_ptr, String message);

    private static native void loggingCritical(long instance_ptr, String message);

    private static native void loggingFatal(long instance_ptr, String message);

    private static native void loggingException(long instance_ptr, String message);

    static public class Logging {

        Long instance_ptr = null;
        int instance_level = NOTSET;

        public Logging() {
            instance_ptr = loggingNew(NOTSET, "root", 0, 0, 0, 0, 0, -1, null);
            instance_level = NOTSET;
        }

        public Logging(int level) {
            instance_ptr = loggingNew(level, "root", 0, 0, 0, 0, 0, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain) {
            instance_ptr = loggingNew(level, domain, 0, 0, 0, 0, 0, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain, ExtConfig extConfig) {
            long extConfig_ptr = 0;
            if (extConfig != null) {
                extConfig_ptr = extConfig.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, extConfig_ptr, 0, 0, 0, 0, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain, ConsoleWriterConfig console) {
            long console_ptr = 0;
            if (console != null) {
                console_ptr = console.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, 0, console_ptr, 0, 0, 0, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain, FileWriterConfig file) {
            long file_ptr = 0;
            if (file != null) {
                file_ptr = file.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, 0, 0, file_ptr, 0, 0, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain, ConsoleWriterConfig console, FileWriterConfig file) {
            long console_ptr = 0;
            if (console != null) {
                console_ptr = console.instance_ptr;
            }
            long file_ptr = 0;
            if (file != null) {
                file_ptr = file.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, 0, console_ptr, file_ptr, 0, 0, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain, FileWriterConfig file, ServerConfig server) {
            long file_ptr = 0;
            if (file != null) {
                file_ptr = file.instance_ptr;
            }
            long server_ptr = 0;
            if (server != null) {
                server_ptr = server.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, 0, 0, file_ptr, server_ptr, 0, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain, FileWriterConfig file, ClientWriterConfig client) {
            long file_ptr = 0;
            if (file != null) {
                file_ptr = file.instance_ptr;
            }
            long client_ptr = 0;
            if (client != null) {
                client_ptr = client.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, 0, 0, file_ptr, 0, client_ptr, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain, ConsoleWriterConfig console, ClientWriterConfig client) {
            long console_ptr = 0;
            if (console != null) {
                console_ptr = console.instance_ptr;
            }
            long client_ptr = 0;
            if (client != null) {
                client_ptr = client.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, 0, console_ptr, 0, 0, client_ptr, -1, null);
        }

        public Logging(int level, String domain, ConsoleWriterConfig console, FileWriterConfig file,
                ClientWriterConfig client) {
            long console_ptr = 0;
            if (console != null) {
                console_ptr = console.instance_ptr;
            }
            long file_ptr = 0;
            if (file != null) {
                file_ptr = file.instance_ptr;
            }
            long client_ptr = 0;
            if (client != null) {
                client_ptr = client.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, 0, console_ptr, file_ptr, 0, client_ptr, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain, ConsoleWriterConfig console, FileWriterConfig file,
                ServerConfig server) {
            long console_ptr = 0;
            if (console != null) {
                console_ptr = console.instance_ptr;
            }
            long file_ptr = 0;
            if (file != null) {
                file_ptr = file.instance_ptr;
            }
            long server_ptr = 0;
            if (server != null) {
                server_ptr = server.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, 0, console_ptr, file_ptr, server_ptr, 0, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain, ClientWriterConfig client) {
            long client_ptr = 0;
            if (client != null) {
                client_ptr = client.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, 0, 0, 0, 0, client_ptr, -1, null);
            instance_level = level;
        }

        public Logging(int level, String domain, int syslog) {
            instance_ptr = loggingNew(level, domain, 0, 0, 0, 0, 0, syslog, null);
            instance_level = level;
        }

        public Logging(int level, String domain, ExtConfig extConfig, ConsoleWriterConfig console,
                FileWriterConfig file, ClientWriterConfig client, int syslog) {
            long extConfig_ptr = 0;
            if (extConfig != null) {
                extConfig_ptr = extConfig.instance_ptr;
            }
            long console_ptr = 0;
            if (console != null) {
                console_ptr = console.instance_ptr;
            }
            long file_ptr = 0;
            if (file != null) {
                file_ptr = file.instance_ptr;
            }
            long client_ptr = 0;
            if (client != null) {
                client_ptr = client.instance_ptr;
            }
            instance_ptr = loggingNew(level, domain, extConfig_ptr, console_ptr, file_ptr, 0, client_ptr, syslog, null);
            instance_level = level;
        }

        public Logging(String path) {
            instance_ptr = loggingNew(NOTSET, null, 0, 0, 0, 0, 0, -1, path);
        }

        public void shutdown() {
            loggingShutdown(instance_ptr, false);
            instance_ptr = 0L;
        }

        public void shutdown(boolean now) {
            loggingShutdown(instance_ptr, now);
            instance_ptr = 0L;
        }

        public void setLevel(WriterTypeEnum writer, int level) {
            loggingSetLevel(instance_ptr, writer.getValue(), "", level);
            instance_level = level;
        }

        public void setLevel(WriterTypeEnum writer, String key, int level) {
            loggingSetLevel(instance_ptr, writer.getValue(), key, level);
            instance_level = level;
        }

        public void setDomain(String domain) {
            loggingSetDomain(instance_ptr, domain);
        }

        public void setLevel2Sym(LevelSyms level2sym) {
            loggingSetLevel2Sym(instance_ptr, level2sym.getValue());
        }

        public void setExtConfig(ExtConfig extConfig) {
            loggingSetExtConfig(instance_ptr, extConfig.instance_ptr);
        }

        public void addLogger(long logger_ptr) {
            loggingAddLogger(instance_ptr, logger_ptr);
        }

        public void removeLogger(long logger_ptr) {
            loggingRemoveLogger(instance_ptr, logger_ptr);
        }

        public void addWriter(long writer_ptr) {
            loggingAddWriter(instance_ptr, writer_ptr);
        }

        public void removeWriter(WriterTypeEnum writer) {
            loggingRemoveWriter(instance_ptr, writer.getValue(), "");
        }

        public void removeWriter(WriterTypeEnum writer, String key) {
            loggingRemoveWriter(instance_ptr, writer.getValue(), key);
        }

        public void sync(boolean console, boolean file, boolean client, boolean syslog, double timeout) {
            loggingSync(instance_ptr, console, file, client, syslog, timeout);
        }

        public void syncAll(double timeout) {
            loggingSyncAll(instance_ptr, timeout);
        }

        // File logger

        public void rotate(String path) {
            loggingRotate(instance_ptr, path);
        }

        // Network

        public void setEncryption(EncryptionMethod method, String key) {
            loggingSetEncryption(instance_ptr, null, method.getValue(), key);
        }

        public void setEncryption(String address, EncryptionMethod method, String key) {
            loggingSetEncryption(instance_ptr, address, method.getValue(), key);
        }

        // Config

        public String getConfig(WriterTypeEnum writer, String key) {
            return loggingGetConfig(instance_ptr, writer.getValue(), key);
        }

        public ServerConfig getServerConfig() {
            return loggingGetServerConfig(instance_ptr);
        }

        public String getServerAddress() {
            return loggingGetServerAddress(instance_ptr);
        }

        public String getServerAuthKey() {
            return loggingGetServerAuthKey(instance_ptr);
        }

        public String getConfigString() {
            return loggingGetConfigString(instance_ptr);
        }

        public void getSaveConfig(String path) {
            loggingSaveConfig(instance_ptr, path);
        }

        // Logging methods

        public void trace(String message) {
            if (instance_level <= TRACE) {
                loggingTrace(instance_ptr, message);
            }
        }

        public void debug(String message) {
            if (instance_level <= DEBUG) {
                loggingDebug(instance_ptr, message);
            }
        }

        public void info(String message) {
            if (instance_level <= INFO) {
                loggingInfo(instance_ptr, message);
            }
        }

        public void success(String message) {
            if (instance_level <= SUCCESS) {
                loggingSuccess(instance_ptr, message);
            }
        }

        public void warning(String message) {
            if (instance_level <= WARN) {
                loggingWarning(instance_ptr, message);
            }
        }

        public void error(String message) {
            if (instance_level <= ERROR) {
                loggingError(instance_ptr, message);
            }
        }

        public void critical(String message) {
            if (instance_level <= CRITICAL) {
                loggingCritical(instance_ptr, message);
            }
        }

        public void fatal(String message) {
            if (instance_level <= FATAL) {
                loggingFatal(instance_ptr, message);
            }
        }

        public void exception(String message) {
            if (instance_level <= EXCEPTION) {
                loggingException(instance_ptr, message);
            }
        }
    }

    // Logger class

    private static native long loggerNew(int level, String domain);

    private static native long loggerNewExt(int level, String domain, boolean tname, boolean tid);

    public static native void loggerSetLevel(long instance_ptr, int level);

    public static native void loggerSetDomain(long instance_ptr, String domain);

    private static native void loggerTrace(long instance_ptr, String message);

    private static native void loggerDebug(long instance_ptr, String message);

    private static native void loggerInfo(long instance_ptr, String message);

    private static native void loggerSuccess(long instance_ptr, String message);

    private static native void loggerWarning(long instance_ptr, String message);

    private static native void loggerError(long instance_ptr, String message);

    private static native void loggerCritical(long instance_ptr, String message);

    private static native void loggerFatal(long instance_ptr, String message);

    private static native void loggerException(long instance_ptr, String message);

    public class Logger {

        Long instance_ptr = null;
        int instance_level = NOTSET;

        public Logger() {
            instance_ptr = loggerNew(NOTSET, null);
        }

        public Logger(int level) {
            instance_ptr = loggerNew(level, null);
            instance_level = level;
        }

        public Logger(String domain) {
            instance_ptr = loggerNew(0, domain);
        }

        public Logger(int level, String domain) {
            instance_ptr = loggerNew(level, domain);
            instance_level = level;
        }

        public Logger(int level, String domain, boolean tname, boolean tid) {
            instance_ptr = loggerNewExt(level, domain, tname, tid);
            instance_level = level;
        }

        public void setLevel(int level) {
            loggerSetLevel(instance_ptr, level);
            instance_level = level;
        }

        public void setDomain(String domain) {
            loggerSetDomain(instance_ptr, domain);
        }

        public void trace(String message) {
            if (instance_level <= TRACE) {
                loggerTrace(instance_ptr, message);
            }
        }

        public void debug(String message) {
            if (instance_level <= DEBUG) {
                loggerDebug(instance_ptr, message);
            }
        }

        public void info(String message) {
            if (instance_level <= INFO) {
                loggerInfo(instance_ptr, message);
            }
        }

        public void success(String message) {
            if (instance_level <= SUCCESS) {
                loggerSuccess(instance_ptr, message);
            }
        }

        public void warning(String message) {
            if (instance_level <= WARN) {
                loggerWarning(instance_ptr, message);
            }
        }

        public void error(String message) {
            if (instance_level <= ERROR) {
                loggerError(instance_ptr, message);
            }
        }

        public void critical(String message) {
            if (instance_level <= CRITICAL) {
                loggerCritical(instance_ptr, message);
            }
        }

        public void fatal(String message) {
            if (instance_level <= FATAL) {
                loggerFatal(instance_ptr, message);
            }
        }

        public void exception(String message) {
            if (instance_level <= EXCEPTION) {
                loggerException(instance_ptr, message);
            }
        }
    }
}
