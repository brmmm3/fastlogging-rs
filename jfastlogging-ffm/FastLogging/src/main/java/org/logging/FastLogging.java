package org.logging;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.nio.charset.StandardCharsets;

public class FastLogging {

	private static final Linker LINKER = Linker.nativeLinker();
	private static final SymbolLookup LOOKUP;

	static {
		System.loadLibrary("jfastlogging");
		LOOKUP = SymbolLookup.loaderLookup();
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

	public enum LevelSyms {
		Sym(0), Short(1), Str(2);

		private final int value;

		private LevelSyms(int value) {
			this.value = value;
		}

		public int getValue() {
			return value;
		}
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
		Root(0), Console(1), File(2), Files(3), Client(4), Clients(5), Server(6), Servers(7), Callback(8), Syslog(9);

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

	// ------------------------------------------------------------------
	// FFM helper methods
	// ------------------------------------------------------------------

	private static MethodHandle lookup(String name, FunctionDescriptor desc) {
		return LOOKUP.find(name).map(addr -> LINKER.downcallHandle(addr, desc))
				.orElseThrow(() -> new UnsatisfiedLinkError("Symbol not found: " + name));
	}

	private static MemorySegment allocStr(Arena arena, String s) {
		if (s == null) {
			return MemorySegment.NULL;
		}
		byte[] bytes = s.getBytes(StandardCharsets.UTF_8);
		MemorySegment seg = arena.allocate(bytes.length + 1);
		seg.copyFrom(MemorySegment.ofArray(bytes));
		seg.set(ValueLayout.JAVA_BYTE, bytes.length, (byte) 0);
		return seg;
	}

	private static long strLen(String s) {
		return s == null ? 0L : (long) s.getBytes(StandardCharsets.UTF_8).length;
	}

	// ------------------------------------------------------------------
	// Config constructors
	// ------------------------------------------------------------------

	public static class ExtConfig {
		long instance_ptr = 0;

		public ExtConfig(MessageStructEnum structured, boolean hostname, boolean pname, boolean pid, boolean tname,
				boolean tid) {
			try {
				MethodHandle mh = lookup("extConfigNew",
						FunctionDescriptor.of(ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_INT,
								ValueLayout.JAVA_BOOLEAN,
								ValueLayout.JAVA_BOOLEAN,
								ValueLayout.JAVA_BOOLEAN,
								ValueLayout.JAVA_BOOLEAN,
								ValueLayout.JAVA_BOOLEAN));
				instance_ptr = (long) mh.invoke(structured.getValue(), hostname, pname, pid, tname, tid);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}
	}

	public static class ConsoleWriterConfig {
		long instance_ptr = 0;

		public ConsoleWriterConfig(int level) {
			this(level, false);
		}

		public ConsoleWriterConfig(int level, boolean colors) {
			try {
				MethodHandle mh = lookup("consoleWriterConfigNew",
						FunctionDescriptor.of(ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_BYTE,
								ValueLayout.JAVA_BOOLEAN));
				instance_ptr = (long) mh.invoke((byte) level, colors);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}
	}

	public static class FileWriterConfig {
		long instance_ptr = 0;

		public FileWriterConfig(int level, String path) {
			this(level, path, 0, 0, 0, 0, CompressionMethodEnum.Store);
		}

		public FileWriterConfig(int level, String path, int size, int backlog, long timeout, long time,
				CompressionMethodEnum compression) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment pathSeg = allocStr(arena, path);
				MethodHandle mh = lookup("fileWriterConfigNew",
						FunctionDescriptor.of(ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_BYTE,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_BYTE));
				instance_ptr = (long) mh.invoke((byte) level, pathSeg, strLen(path),
						(long) size, (long) backlog, timeout, time, (byte) compression.getValue());
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}
	}

	public static class ClientWriterConfig {
		long instance_ptr = 0;

		public ClientWriterConfig(int level, String address, int port) {
			this(level, address, port, EncryptionMethod.NONE, null);
		}

		public ClientWriterConfig(int level, String address, int port, EncryptionMethod method, String key) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment addrSeg = allocStr(arena, address);
				MemorySegment keySeg = allocStr(arena, key);
				MethodHandle mh = lookup("clientWriterConfigNew",
						FunctionDescriptor.of(ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_BYTE,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_BYTE,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				instance_ptr = (long) mh.invoke((byte) level, addrSeg, strLen(address),
						(byte) method.getValue(), keySeg, strLen(key));
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}
	}

	public static class ServerConfig {
		long instance_ptr = 0;

		public ServerConfig(int level, String address, int port) {
			this(level, address, port, EncryptionMethod.NONE, null);
		}

		public ServerConfig(int level, String address, int port, EncryptionMethod method, String key) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment addrSeg = allocStr(arena, address);
				MemorySegment keySeg = allocStr(arena, key);
				MethodHandle mh = lookup("serverConfigNew",
						FunctionDescriptor.of(ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_BYTE,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_BYTE,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				instance_ptr = (long) mh.invoke((byte) level, addrSeg, strLen(address),
						(byte) method.getValue(), keySeg, strLen(key));
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}
	}

	@FunctionalInterface
	interface CallbackWriterConfigLog {
		void invoke(int level, MemorySegment domain, long domainLen, MemorySegment message, long messageLen);
	}

	public static class CallbackWriterConfig {
		long instance_ptr = 0;
		private Arena arena;

		public CallbackWriterConfig(int level, CallbackWriterConfigLog callback) {
			this.arena = Arena.ofShared();
			try {
				MethodHandle targetMh = MethodHandles.lookup().findVirtual(
						CallbackWriterConfigLog.class, "invoke",
						MethodType.methodType(void.class, int.class,
								MemorySegment.class, long.class, MemorySegment.class, long.class));
				MethodHandle boundMh = targetMh.bindTo(callback);
				MemorySegment cbStub = LINKER.upcallStub(boundMh,
						FunctionDescriptor.ofVoid(
								ValueLayout.JAVA_INT,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG),
						arena);
				MethodHandle mh = lookup("callbackWriterConfigNew",
						FunctionDescriptor.of(ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_BYTE));
				instance_ptr = (long) mh.invoke(cbStub, (byte) level);
			} catch (Throwable e) {
				arena.close();
				throw new RuntimeException(e);
			}
		}
	}

	// ------------------------------------------------------------------
	// Logging class
	// ------------------------------------------------------------------

	static public class Logging {

		Long instance_ptr = null;
		int instance_level = NOTSET;

		public Logging() {
			this(NOTSET, "root");
		}

		public Logging(int level) {
			this(level, "root");
		}

		public Logging(int level, String domain) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment domainSeg = allocStr(arena, domain);
				MethodHandle mh = lookup("loggingNew",
						FunctionDescriptor.of(ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_INT,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				instance_ptr = (long) mh.invoke(level, domainSeg, strLen(domain),
						MemorySegment.NULL, 0L, MemorySegment.NULL, MemorySegment.NULL, 0L);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
			instance_level = level;
		}

		public Logging(int level, String domain, ExtConfig extConfig) {
			this(level, domain);
			if (extConfig != null) {
				setExtConfig(extConfig);
			}
		}

		public Logging(int level, String domain, ConsoleWriterConfig console) {
			this(level, domain);
			if (console != null) {
				addWriter(console.instance_ptr);
			}
		}

		public Logging(int level, String domain, FileWriterConfig file) {
			this(level, domain);
			if (file != null) {
				addWriter(file.instance_ptr);
			}
		}

		public Logging(int level, String domain, CallbackWriterConfig callback) {
			this(level, domain);
			if (callback != null) {
				addWriter(callback.instance_ptr);
			}
		}

		public void shutdown() {
			shutdown(false);
		}

		public void shutdown(boolean now) {
			if (instance_ptr == null || instance_ptr == 0) {
				return;
			}
			try {
				MethodHandle mh = lookup("loggingShutdown",
						FunctionDescriptor.ofVoid(
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_INT));
				mh.invoke(instance_ptr, now ? 1 : 0);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
			instance_ptr = 0L;
		}

		public void setLevel(WriterTypeEnum writer, int level) {
			setLevel(writer, "", level);
		}

		public void setLevel(WriterTypeEnum writer, String key, int level) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment keySeg = allocStr(arena, key);
				MethodHandle mh = lookup("loggingSetLevel",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_BYTE));
				mh.invoke(instance_ptr, (long) writer.getValue(), keySeg, strLen(key), (byte) level);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
			instance_level = level;
		}

		public void setDomain(String domain) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment domainSeg = allocStr(arena, domain);
				MethodHandle mh = lookup("loggingSetDomain",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, domainSeg, strLen(domain));
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public void setLevel2Sym(LevelSyms level2sym) {
			try {
				MethodHandle mh = lookup("loggingSetLevel2Sym",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_INT));
				mh.invoke(instance_ptr, level2sym.getValue());
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public void setExtConfig(ExtConfig extConfig) {
			try {
				MethodHandle mh = lookup("loggingSetExtConfig",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, extConfig.instance_ptr);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public void addLogger(long logger_ptr) {
			try {
				MethodHandle mh = lookup("loggingAddLogger",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, logger_ptr);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public void removeLogger(long logger_ptr) {
			try {
				MethodHandle mh = lookup("loggingRemoveLogger",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, logger_ptr);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public void addWriter(long writer_ptr) {
			try {
				MethodHandle mh = lookup("loggingAddWriterConfig",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, writer_ptr);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public void removeWriter(WriterTypeEnum writer) {
			removeWriter(writer, "");
		}

		public void removeWriter(WriterTypeEnum writer, String key) {
			try {
				MethodHandle mh = lookup("loggingRemoveWriter",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, (long) writer.getValue());
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public void syncAll(double timeout) {
			try {
				MethodHandle mh = lookup("loggingSyncAll",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_DOUBLE));
				mh.invoke(instance_ptr, timeout);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public void rotate(String path) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment pathSeg = allocStr(arena, path);
				MethodHandle mh = lookup("loggingRotate",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, pathSeg, strLen(path));
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public String getConfigString() {
			try {
				MethodHandle mh = lookup("loggingGetConfigString",
						FunctionDescriptor.of(ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				MemorySegment ptr = (MemorySegment) mh.invoke(instance_ptr);
				if (ptr == null || ptr.address() == 0) {
					return null;
				}
				// Reinterpret with a large size so getUtf8String can read the C string
				MemorySegment strSeg = ptr.reinterpret(Integer.MAX_VALUE);
				return strSeg.getUtf8String(0);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public void getSaveConfig(String path) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment pathSeg = allocStr(arena, path);
				MethodHandle mh = lookup("loggingSaveConfig",
						FunctionDescriptor.of(ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, pathSeg, strLen(path));
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		// Logging methods

		public void trace(String message) {
			if (instance_level <= TRACE) {
				logMessage("loggingTrace", message);
			}
		}

		public void debug(String message) {
			if (instance_level <= DEBUG) {
				logMessage("loggingDebug", message);
			}
		}

		public void info(String message) {
			if (instance_level <= INFO) {
				logMessage("loggingInfo", message);
			}
		}

		public void success(String message) {
			if (instance_level <= SUCCESS) {
				logMessage("loggingSuccess", message);
			}
		}

		public void warning(String message) {
			if (instance_level <= WARN) {
				logMessage("loggingWarning", message);
			}
		}

		public void error(String message) {
			if (instance_level <= ERROR) {
				logMessage("loggingError", message);
			}
		}

		public void critical(String message) {
			if (instance_level <= CRITICAL) {
				logMessage("loggingCritical", message);
			}
		}

		public void fatal(String message) {
			if (instance_level <= FATAL) {
				logMessage("loggingFatal", message);
			}
		}

		public void exception(String message) {
			if (instance_level <= EXCEPTION) {
				logMessage("loggingException", message);
			}
		}

		private void logMessage(String funcName, String message) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment msgSeg = allocStr(arena, message);
				MethodHandle mh = lookup(funcName,
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, msgSeg, strLen(message));
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}
	}

	// ------------------------------------------------------------------
	// Logger class
	// ------------------------------------------------------------------

	public static class Logger {

		Long instance_ptr = null;
		int instance_level = NOTSET;

		public Logger() {
			this(NOTSET, null);
		}

		public Logger(int level) {
			this(level, null);
		}

		public Logger(String domain) {
			this(0, domain);
		}

		public Logger(int level, String domain) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment domainSeg = allocStr(arena, domain);
				MethodHandle mh = lookup("loggerNew",
						FunctionDescriptor.of(ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_INT,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				instance_ptr = (long) mh.invoke(level, domainSeg, strLen(domain));
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
			instance_level = level;
		}

		public void setLevel(int level) {
			try {
				MethodHandle mh = lookup("loggerSetLevel",
						FunctionDescriptor.ofVoid(
								ValueLayout.JAVA_LONG,
								ValueLayout.JAVA_INT));
				mh.invoke(instance_ptr, level);
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
			instance_level = level;
		}

		public void setDomain(String domain) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment domainSeg = allocStr(arena, domain);
				MethodHandle mh = lookup("loggerSetDomain",
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, domainSeg, strLen(domain));
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}

		public void trace(String message) {
			if (instance_level <= TRACE) {
				logMessage("loggerTrace", message);
			}
		}

		public void debug(String message) {
			if (instance_level <= DEBUG) {
				logMessage("loggerDebug", message);
			}
		}

		public void info(String message) {
			if (instance_level <= INFO) {
				logMessage("loggerInfo", message);
			}
		}

		public void success(String message) {
			if (instance_level <= SUCCESS) {
				logMessage("loggerSuccess", message);
			}
		}

		public void warning(String message) {
			if (instance_level <= WARN) {
				logMessage("loggerWarning", message);
			}
		}

		public void error(String message) {
			if (instance_level <= ERROR) {
				logMessage("loggerError", message);
			}
		}

		public void critical(String message) {
			if (instance_level <= CRITICAL) {
				logMessage("loggerCritical", message);
			}
		}

		public void fatal(String message) {
			if (instance_level <= FATAL) {
				logMessage("loggerFatal", message);
			}
		}

		public void exception(String message) {
			if (instance_level <= EXCEPTION) {
				logMessage("loggerException", message);
			}
		}

		private void logMessage(String funcName, String message) {
			try (Arena arena = Arena.ofConfined()) {
				MemorySegment msgSeg = allocStr(arena, message);
				MethodHandle mh = lookup(funcName,
						FunctionDescriptor.of(ValueLayout.JAVA_INT,
								ValueLayout.JAVA_LONG,
								ValueLayout.ADDRESS,
								ValueLayout.JAVA_LONG));
				mh.invoke(instance_ptr, msgSeg, strLen(message));
			} catch (Throwable e) {
				throw new RuntimeException(e);
			}
		}
	}
}
