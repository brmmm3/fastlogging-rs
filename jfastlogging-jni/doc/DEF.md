# Definitions

## Log level names and their values

`NOLOG` (100) &ensp;&ensp;&ensp;&ensp;&ensp; No logging.  
`EXCEPTION` (60) &nbsp; Log exception messages. In addition to the message text the exception info (output of traceback.format_exc) is logged (SLOW!).  
`CRITICAL` (50) &nbsp;&ensp; Log fatal/critical messages. Default color is bright red.  
`FATAL` (50) &ensp;&ensp;&ensp;&ensp;&ensp; Same as CRITICAL.  
`ERROR` (40) &ensp;&ensp;&ensp;&ensp;&ensp; Log also error messages. Default color is red.  
`WARNING` (30) &ensp;&ensp;&ensp; Log also warning messages. Default color is bright yellow.  
`SUCCESS` (25) &ensp;&ensp;&ensp; Success messages.  
`INFO` (20) &ensp;&ensp;&ensp;&ensp;&ensp;&ensp;&nbsp; Log also info messages. Default color is bright green.  
`DEBUG` (10) &ensp;&ensp;&ensp;&ensp;&ensp;  Log also debug messages. Default color is white.  
`TRACE` (5) &ensp;&ensp;&ensp;&ensp;&ensp;  Trace messages.  
`NOTSET` (0) &ensp;&ensp;&ensp;&ensp; All messages are logged.

## Enum `LevelSyms`

The enum has following values:

```java
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
```

## Enum `MessageStructEnum`

The enum has following values:

```java
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
```

## Enum `WriterTypeEnum`

The enum has following values:

```java
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
```

## Enum `CompressionMethodEnum`

The enum has following values:

```java
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
```

## Enum `EncryptionMethod`

The enum has following values:

```java
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
```

## Class `ExtConfig`

This class is for configuring extended formatting setting. It has following members:

```java
static public class ExtConfig {
    long instance_ptr = 0;

    public ExtConfig(MessageStructEnum structured, boolean hostname, boolean pname, boolean pid, boolean tname,
            boolean tid) {
        instance_ptr = extConfigNew(structured.getValue(), hostname, pname, pid, tname, tid);
    }
}
```

## Class `ConsoleWriterConfig`

```rust
static public class ConsoleWriterConfig {
    long instance_ptr = 0;

    public ConsoleWriterConfig(int level) {
        instance_ptr = consoleWriterConfigNew(level, false);
    }

    public ConsoleWriterConfig(int level, boolean colors) {
        instance_ptr = consoleWriterConfigNew(level, colors);
    }
}
```

## Class `FileWriterConfig`

```rust
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
```

## Class `ClientWriterConfig`

```rust
static public class ClientWriterConfig {
    long instance_ptr = 0;

    public ClientWriterConfig(int level, String address, int port) {
        instance_ptr = clientWriterConfigNew(level, address, port, 0, null);
    }

    public ClientWriterConfig(int level, String address, int port, EncryptionMethod method, String key) {
        instance_ptr = clientWriterConfigNew(level, address, port, method.getValue(), key);
    }
}
```

## Class `ServerConfig`

```rust
static public class ServerConfig {
    long instance_ptr = 0;

    public ServerConfig(int level, String address, int port) {
        instance_ptr = serverConfigNew(level, address, port, 0, null);
    }

    public ServerConfig(int level, String address, int port, EncryptionMethod method, String key) {
        instance_ptr = serverConfigNew(level, address, port, method.getValue(), key);
    }
}
```
