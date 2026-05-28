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
