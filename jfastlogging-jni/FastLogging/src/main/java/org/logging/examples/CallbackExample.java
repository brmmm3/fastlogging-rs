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
        CallbackWriterConfig callback = new FastLogging.CallbackWriterConfig(FastLogging.DEBUG, this);
        Logging logging = new Logging(FastLogging.DEBUG, "root", callback);
        logging.setLevel(0, FastLogging.DEBUG);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warning("Warning Message");
        logging.error("Error Message");
        // logging.setLevel(WriterTypeEnum.Console, FastLogging.WARNING);
        logging.debug("Debug Message");
        logging.shutdown();
    }

    public static void main(String[] args) {
        CallbackExample example = new CallbackExample();
        example.doLogging();
        example.doLogging();
    }
}
