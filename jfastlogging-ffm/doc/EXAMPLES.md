# Examples

The following examples are available in `jfastlogging-jni/FastLogging/src/main/java/org/logging/examples/`. Build with `make lib && make java_build`.

## 1. Console Example

Based on `ConsoleExample.java`:

```java
package org.logging.examples;

import org.logging.FastLogging;
import org.logging.FastLogging.ConsoleWriterConfig;
import org.logging.FastLogging.Logging;

class ConsoleExample {
    void doLogging() {
        ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
        Logging logging = new Logging(FastLogging.INFO, "root", console);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warning("Warning Message");
        logging.error("Error Message");
        logging.shutdown();
    }

    public static void main(String[] args) {
        ConsoleExample example = new ConsoleExample();
        example.doLogging();
        example.doLogging();
    }
}
```

## 2. File Example

Based on `FileExample.java`:

```java
package org.logging.examples;

import org.logging.FastLogging;
import org.logging.FastLogging.FileWriterConfig;
import org.logging.FastLogging.Logging;

class FileExample {
    void doLogging(String pathName) {
        FileWriterConfig file = new FileWriterConfig(FastLogging.DEBUG, pathName);
        Logging logging = new Logging(FastLogging.DEBUG, "root", file);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warning("Warning Message");
        logging.error("Error Message");
        logging.shutdown();
    }

    public static void main(String[] args) {
        String osName = System.getProperty("os.name");
        String pathName;
        if (osName.startsWith("Windows")) {
            pathName = "C:\\temp\\jfastlogging\\FileExample.log";
        } else {
            pathName = "/tmp/jfastlogging/FileExample.log";
        }
        FileExample example = new FileExample();
        example.doLogging(pathName);
        example.doLogging(pathName);
    }
}
```

## 3. Callback Example

Based on `CallbackExample.java`. Note: This references `CallbackWriterConfig` and `CallbackWriterConfigLog` which may not be in the current Java wrapper release.

```java
package org.logging.examples;

import org.logging.FastLogging;
import org.logging.FastLogging.CallbackWriterConfig;
import org.logging.FastLogging.CallbackWriterConfigLog;
import org.logging.FastLogging.Logging;

class CallbackExample implements CallbackWriterConfigLog {
    public void onLog(int level, String domain, String message) {
        System.out.println(String.format("Java-CB: %d %s: %s", level, domain, message));
    }

    void doLogging() {
        CallbackWriterConfig callback = new CallbackWriterConfig(FastLogging.DEBUG, this);
        Logging logging = new Logging(FastLogging.DEBUG, "root", callback);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warning("Warning Message");
        logging.error("Error Message");
        logging.shutdown();
    }

    public static void main(String[] args) {
        CallbackExample example = new CallbackExample();
        example.doLogging();
        example.doLogging();
    }
}
```

## 4. Network Example (server + client)

```java
package org.logging.examples;

import org.logging.FastLogging;
import org.logging.FastLogging.ConsoleWriterConfig;
import org.logging.FastLogging.FileWriterConfig;
import org.logging.FastLogging.ServerConfig;
import org.logging.FastLogging.ClientWriterConfig;
import org.logging.FastLogging.Logging;

class NetworkExample {
    public static void main(String[] args) {
        // Server
        ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
        FileWriterConfig file = new FileWriterConfig(FastLogging.DEBUG, "/tmp/server.log");
        ServerConfig server = new ServerConfig(FastLogging.DEBUG, "127.0.0.1", 12345);
        Logging loggingServer = new Logging(FastLogging.DEBUG, "LOGSRV", console, file, server);
        loggingServer.syncAll(5.0);

        String address = loggingServer.getServerAddress();
        System.out.println("Server address: " + address);

        // Client
        ClientWriterConfig client = new ClientWriterConfig(FastLogging.DEBUG, "127.0.0.1", 12345);
        Logging loggingClient = new Logging(FastLogging.DEBUG, "LOGCLIENT", client);

        loggingClient.trace("Trace message");
        loggingClient.debug("Debug message");
        loggingClient.info("Info Message");

        loggingServer.trace("Trace message");
        loggingServer.debug("Debug message");
        loggingServer.info("Info Message");

        loggingClient.syncAll(1.0);
        loggingServer.syncAll(1.0);
        loggingClient.shutdown();
        loggingServer.shutdown();
    }
}
```

## 5. Network Example (encrypted)

```java
package org.logging.examples;

import org.logging.FastLogging;
import org.logging.FastLogging.*;
import org.logging.FastLogging.EncryptionMethod;

class NetworkEncryptedExample {
    public static void main(String[] args) {
        String key = "my-secret-aes-key";

        // Server with AES encryption
        ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
        FileWriterConfig file = new FileWriterConfig(FastLogging.DEBUG, "/tmp/server.log");
        ServerConfig server = new ServerConfig(FastLogging.DEBUG, "127.0.0.1", 12345,
                EncryptionMethod.AES, key);
        Logging loggingServer = new Logging(FastLogging.DEBUG, "LOGSRV", console, file, server);
        loggingServer.syncAll(5.0);

        String address = loggingServer.getServerAddress();
        String authKey = loggingServer.getServerAuthKey();

        // Client with AES encryption
        ClientWriterConfig client = new ClientWriterConfig(FastLogging.DEBUG, "127.0.0.1", 12345,
                EncryptionMethod.AES, authKey);
        Logging loggingClient = new Logging(FastLogging.DEBUG, "LOGCLIENT", client);

        loggingClient.info("Encrypted hello");
        loggingClient.syncAll(1.0);
        loggingServer.syncAll(1.0);
        loggingClient.shutdown();
        loggingServer.shutdown();
    }
}
```

## 6. Extended Config Example

```java
package org.logging.examples;

import org.logging.FastLogging;
import org.logging.FastLogging.*;

class ExtConfigExample {
    public static void main(String[] args) {
        ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
        ExtConfig extConfig = new ExtConfig(MessageStructEnum.Xml, true, false, true, false, true);
        Logging logging = new Logging(FastLogging.DEBUG, "root", extConfig, console, null, null, -1);
        logging.info("Info with extended config");
        logging.shutdown();
    }
}
```

## 7. Logger (threads) Example

```java
package org.logging.examples;

import org.logging.FastLogging;
import org.logging.FastLogging.*;

class ThreadsExample {
    public static void main(String[] args) throws InterruptedException {
        ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
        Logging logging = new Logging(FastLogging.DEBUG, "root", console);

        FastLogging fastLogging = new FastLogging();
        FastLogging.Logger logger = fastLogging.new Logger(FastLogging.DEBUG, "WorkerThread", true, true);
        logging.addLogger(logger.instance_ptr);

        Thread thread = new Thread(() -> {
            logger.trace("Trace from worker");
            logger.debug("Debug from worker");
            logger.info("Info from worker");
        });
        thread.start();

        logging.trace("Trace from main");
        logging.debug("Debug from main");
        logging.info("Info from main");

        thread.join();
        logging.shutdown();
    }
}
```

## 8. Config File Example

```java
package org.logging.examples;

import org.logging.FastLogging;
import org.logging.FastLogging.Logging;

class ConfigFileExample {
    public static void main(String[] args) {
        // Load configuration from file
        Logging logging = new Logging("/path/to/fastlogging.json");
        logging.info("Logger configured from file");
        logging.shutdown();
    }
}
```

## Available examples

| Example | Description |
|---|---|
| `ConsoleExample` | Console writer |
| `FileExample` | File writer |
| `CallbackExample` | Callback writer (requires CallbackWriterConfig class) |
| `NetworkExample` | Network logging (unencrypted) |
| `NetworkEncryptedExample` | Network logging with AES |
| `ExtConfigExample` | Extended formatting config |
| `ThreadsExample` | Multi-threaded logging with Logger |
| `ConfigFileExample` | Load config from file |
