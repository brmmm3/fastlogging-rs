package org.logging;

public class FastLogging {

    static {
        System.loadLibrary("jfastlogging");
    }

    // Log levels
    public static final int NOLOG = 70;
    public static final int EXCEPTION = 60;
    public static final int CRITICAL = 50;
    public static final int FATAL = CRITICAL;
    public static final int ERROR = 40;
    public static final int WARNING = 30;
    public static final int WARN = WARNING;
    public static final int INFO = 20;
    public static final int DEBUG = 10;
    public static final int NOTSET = 0;

    enum LevelSyms {
        Sym,
        Short,
        Str,
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
            case INFO:
                return "INFO";
            case DEBUG:
                return "DEBUG";
            case NOTSET:
                return "NOTSET";
        }
        return "?";
    }

    // Logging class

    public static native long loggingNew(int level, String domain, boolean console, String file, String server,
            String connect, int max_size, int backlog);

    private static native void loggingShutdown(long instance_ptr, boolean now);

    private static native void loggingAddLogger(long instance_ptr, long logger_ptr);

    private static native void loggingRemoveLogger(long instance_ptr, long logger_ptr);

    public static native void loggingSetLevel(long instance_ptr, int level);

    public static native void loggingSetDomain(long instance_ptr, String domain);

    // TODO
    public static native void loggingSetLevel2Sym(long instance_ptr, String domain);

    public static native void loggingSetConsoleWriter(long instance_ptr, int level);

    public static native void loggingSetConsoleColors(long instance_ptr, boolean colors);

    public static native void loggingSetFileWriter(long instance_ptr, int level, String path, int maxSize, int backlog);

    public static native void loggingRotate(long instance_ptr);

    public static native void loggingSync(long instance_ptr, double timeout);

    public static native void loggingConnect(long instance_ptr, String address, int level, String key);

    public static native void loggingDisconnect(long instance_ptr, String address);

    public static native void loggingSetClientLevel(long instance_ptr, String address, int level);

    public static native void loggingSetClientEncryption(long instance_ptr, String address, String key);

    public static native void loggingServerStart(long instance_ptr, String address, int level, String key);

    public static native void loggingServerShutdown(long instance_ptr);

    public static native void loggingSetServerLevel(long instance_ptr, int level);

    public static native void loggingSetServerEncryption(long instance_ptr, String key);

    private static native void loggingDebug(long instance_ptr, String message);

    private static native void loggingInfo(long instance_ptr, String message);

    private static native void loggingWarning(long instance_ptr, String message);

    private static native void loggingError(long instance_ptr, String message);

    private static native void loggingCritical(long instance_ptr, String message);

    private static native void loggingFatal(long instance_ptr, String message);

    private static native void loggingException(long instance_ptr, String message);

    static public class Logging {

        Long instance_ptr = null;
        int level = NOTSET;

        public Logging() {
            instance_ptr = loggingNew(0, "root", true, null, null, null, 0, 0);
        }

        public void shutdown() {
            loggingShutdown(instance_ptr, false);
            instance_ptr = 0L;
        }

        public void shutdown(boolean now) {
            loggingShutdown(instance_ptr, now);
            instance_ptr = 0L;
        }

        public void addLogger(long logger_ptr) {
            loggingAddLogger(instance_ptr, logger_ptr);
        }

        public void removeLogger(long logger_ptr) {
            loggingRemoveLogger(instance_ptr, logger_ptr);
        }

        public void setLevel(int level) {
            loggingSetLevel(instance_ptr, level);
            this.level = level;
        }

        public void setDomain(String domain) {
            loggingSetDomain(instance_ptr, domain);
        }

        public void setConsoleLogger(int level) {
            loggingSetConsoleWriter(instance_ptr, level);
        }

        public void setConsoleColors(boolean colors) {
            loggingSetConsoleColors(instance_ptr, colors);
        }

        public void setFileLogger(int level, String path, int maxSize, int backlog) {
            loggingSetFileWriter(instance_ptr, level, path, maxSize, backlog);
        }

        public void rotate() {
            loggingRotate(instance_ptr);
        }

        public void sync(double timeout) {
            loggingSync(instance_ptr, timeout);
        }

        public void connect(String address, int level, String key) {
            loggingConnect(instance_ptr, address, level, key);
        }

        public void disconnect(String address) {
            loggingDisconnect(instance_ptr, address);
        }

        public void setClientLevel(String address, int level) {
            loggingSetClientLevel(instance_ptr, address, level);
        }

        public void setClientEncryption(String address, String key) {
            loggingSetClientEncryption(instance_ptr, address, key);
        }

        public void setServerStart(String address, int level, String key) {
            loggingServerStart(instance_ptr, address, level, key);
        }

        public void setServerShutdown() {
            loggingServerShutdown(instance_ptr);
        }

        public void setServerLevel(int level) {
            loggingSetServerLevel(instance_ptr, level);
        }

        public void setServerEncryption(String key) {
            loggingSetServerEncryption(instance_ptr, key);
        }

        public void debug(String message) {
            if (level <= DEBUG) {
                loggingDebug(instance_ptr, message);
            }
        }

        public void info(String message) {
            if (level <= INFO) {
                loggingInfo(instance_ptr, message);
            }
        }

        public void warning(String message) {
            if (level <= WARN) {
                loggingWarning(instance_ptr, message);
            }
        }

        public void error(String message) {
            if (level <= ERROR) {
                loggingError(instance_ptr, message);
            }
        }

        public void critical(String message) {
            if (level <= CRITICAL) {
                loggingCritical(instance_ptr, message);
            }
        }

        public void fatal(String message) {
            if (level <= FATAL) {
                loggingFatal(instance_ptr, message);
            }
        }

        public void exception(String message) {
            if (level <= EXCEPTION) {
                loggingException(instance_ptr, message);
            }
        }
    }

    // Logger class

    private static native long loggerNew(int level, String domain);

    public static native void loggerSetLevel(long instance_ptr, int level);

    public static native void loggerSetDomain(long instance_ptr, String domain);

    private static native void loggerDebug(long instance_ptr, String message);

    private static native void loggerInfo(long instance_ptr, String message);

    private static native void loggerWarning(long instance_ptr, String message);

    private static native void loggerError(long instance_ptr, String message);

    private static native void loggerCritical(long instance_ptr, String message);

    private static native void loggerFatal(long instance_ptr, String message);

    private static native void loggerException(long instance_ptr, String message);

    public class Logger {

        Long instance_ptr = null;
        int level = NOTSET;

        public Logger() {
            instance_ptr = loggerNew(0, null);
        }

        public Logger(int level) {
            instance_ptr = loggerNew(level, null);
        }

        public Logger(String domain) {
            instance_ptr = loggerNew(0, domain);
        }

        public Logger(int level, String domain) {
            instance_ptr = loggerNew(level, domain);
        }

        public void setLevel(int level) {
            loggerSetLevel(instance_ptr, level);
            this.level = level;
        }

        public void setDomain(String domain) {
            loggerSetDomain(instance_ptr, domain);
        }

        public void debug(String message) {
            if (level <= DEBUG) {
                loggerDebug(instance_ptr, message);
            }
        }

        public void info(String message) {
            if (level <= INFO) {
                loggerInfo(instance_ptr, message);
            }
        }

        public void warning(String message) {
            if (level <= WARN) {
                loggerWarning(instance_ptr, message);
            }
        }

        public void error(String message) {
            if (level <= ERROR) {
                loggerError(instance_ptr, message);
            }
        }

        public void critical(String message) {
            if (level <= CRITICAL) {
                loggerCritical(instance_ptr, message);
            }
        }

        public void fatal(String message) {
            if (level <= FATAL) {
                loggerFatal(instance_ptr, message);
            }
        }

        public void exception(String message) {
            if (level <= EXCEPTION) {
                loggerException(instance_ptr, message);
            }
        }
    }
}
