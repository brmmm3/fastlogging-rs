# Examples

## Logging to Console with Logging instance

```java
package org.logging;

import org.logging.FastLogging.ConsoleWriterConfig;
import org.logging.FastLogging.Logging;
import org.logging.FastLogging.WriterTypeEnum;

class ConsoleExample {
    static void doLogging() {
        ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
        Logging logging = new Logging(FastLogging.DEBUG, "root", console);
        logging.setLevel(WriterTypeEnum.Console, FastLogging.DEBUG);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warning("Warning Message");
        logging.error("Error Message");
        //logging.setLevel(WriterTypeEnum.Console, FastLogging.WARNING);
        logging.debug("Debug Message");
        logging.shutdown();
    }

    public static void main(String[] args) {
        doLogging();
        doLogging();
    }
}
```

## Logging to File using root logger

```java
package org.logging;

import org.logging.FastLogging.FileWriterConfig;
import org.logging.FastLogging.Logging;

class FileExample {
    static void doLogging(String pathName) {
        FileWriterConfig file = new FileWriterConfig(FastLogging.DEBUG, pathName);
        Logging logging = new Logging(FastLogging.DEBUG, "root", file);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warning("Warning Message");
        logging.error("Error Message");
        logging.debug("Debug Message");
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
        doLogging(pathName);
        doLogging(pathName);
    }
}
```

## Logging via network sockets

```python
import tempfile

from pyfastlogging import (
    TRACE,
    DEBUG,
    Logging,
    ConsoleWriterConfig,
    FileWriterConfig,
    ServerConfig,
    ClientWriterConfig,
)

def SomeThread(logger):
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")

if __name__ == "__main__":
    tmpDir = tempfile.mkdtemp(prefix="fastlogging")
    logging_server = Logging(
        TRACE,
        "LOGSRV",
        [
            ConsoleWriterConfig(TRACE, True),
            FileWriterConfig(TRACE, f"{tmpDir}/fastlogging.log"),
            ServerConfig(TRACE, "127.0.0.1"),
        ],
    )
    logging_server.sync_all(5.0)
    address = logging_server.get_server_address()
    print(address)
    key = logging_server.get_server_auth_key()
    print(key)
    logging_client = Logging(
        TRACE, "LOGCLIENT", [ClientWriterConfig(DEBUG, address, key)]
    )
    logging_client.trace("Trace Message")
    logging_client.debug("Debug Message")
    logging_client.info("Info Message")

    logging_server.trace("Trace Message")
    logging_server.debug("Debug Message")
    logging_server.info("Info Message")

    logging_client.sync_all(1.0)
    logging_server.sync_all(1.0)

    logging_client.shutdown()
    logging_server.shutdown()
```

## Logging using callback

```java
package org.logging;

import org.logging.FastLogging.CallbackWriterConfig;
import org.logging.FastLogging.CallbackWriterConfigLog;
import org.logging.FastLogging.Logging;
import org.logging.FastLogging.WriterTypeEnum;

class CallbackExample implements CallbackWriterConfigLog {
    public void log(int level, String domain, String message) {
        System.out.println(String.format("Java-CB: %d %s: %s", level,  domain, message));
    }

    static void doLogging() {
        CallbackWriterConfig callback = new CallbackWriterConfig(FastLogging.DEBUG, this);
        Logging logging = new Logging(FastLogging.DEBUG, "root", callback);
        logging.setLevel(WriterTypeEnum.Console, FastLogging.DEBUG);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warning("Warning Message");
        logging.error("Error Message");
        //logging.setLevel(WriterTypeEnum.Console, FastLogging.WARNING);
        logging.debug("Debug Message");
        logging.shutdown();
    }

    public static void main(String[] args) {
        doLogging();
        doLogging();
    }
}

```

## Logging to syslog

```python
from pyfastlogging import (
    TRACE,
    Logging,
)

if __name__ == "__main__":
    logger = Logging(
        TRACE,
        "main",
        syslog=TRACE,
    )
    logger.trace("Trace Message")
    logger.debug("Debug Message")
    logger.info("Info Message")
    logger.shutdown()
```
