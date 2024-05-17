package org.example;

import org.logging.FastLogging;
import org.logging.FastLogging.Logging;

class ConsoleExample {
    public static void main(String[] args) {
        Logging logging = new Logging();
        logging.setLevel(FastLogging.DEBUG);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warning("Warning Message");
        logging.error("Error Message");
        logging.setLevel(FastLogging.WARNING);
        logging.debug("Debug Message");
        logging.shutdown();
    }
}
