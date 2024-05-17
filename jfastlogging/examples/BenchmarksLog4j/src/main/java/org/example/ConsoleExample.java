package org.example;

import org.apache.log4j.*;

class NetClientExample {
    @SuppressWarnings("deprecation")
    public static void main(String[] args) {
        Logger logging = Logger.getLogger("root");
        ConsoleAppender consoleAppender = new ConsoleAppender(new SimpleLayout());
        logging.addAppender(consoleAppender);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warn("Warning Message");
        logging.error("Error Message");
        logging.debug("Debug Message");
        Logger.shutdown();
    }
}
