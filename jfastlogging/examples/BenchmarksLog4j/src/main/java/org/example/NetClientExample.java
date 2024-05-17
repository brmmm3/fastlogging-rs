package org.example;

import org.apache.log4j.*;
import org.apache.log4j.net.SocketAppender;

class NetClientExample {
    @SuppressWarnings("deprecation")
    public static void main(String[] args) {
        Logger logging = Logger.getLogger("root");
        ConsoleAppender consoleAppender = new ConsoleAppender(new SimpleLayout());
        logging.addAppender(consoleAppender);
        SocketAppender appender = new SocketAppender("127.0.0.1", 4712);
        logging.addAppender(appender);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warn("Warning Message");
        logging.error("Error Message");
        logging.debug("Debug Message");
        Logger.shutdown();
    }
}
