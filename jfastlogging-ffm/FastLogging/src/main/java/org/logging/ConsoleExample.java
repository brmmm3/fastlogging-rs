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
