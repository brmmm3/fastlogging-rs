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
