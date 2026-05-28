package org.logging.examples;

import org.logging.FastLogging;
import org.logging.FastLogging.ConsoleWriterConfig;
import org.logging.FastLogging.Logging;

class ConsoleExample {
  void doLogging() {
    ConsoleWriterConfig console = new ConsoleWriterConfig(FastLogging.DEBUG, true);
    Logging logging = new Logging(FastLogging.INFO, "root", console);
    /*
     * 
     * 
     * logging.setLevel(0, FastLogging.DEBUG);
     * logging.debug("Debug Message");
     * logging.info("Info Message");
     * logging.warning("Warning Message");
     * logging.error("Error Message");
     * // logging.setLevel(WriterTypeEnum.Console, FastLogging.WARNING);
     * logging.debug("Debug Message");
     * 
     * logging.shutdown();
     */
  }

  public static void main(String[] args) {
    ConsoleExample logging = new ConsoleExample();
    logging.doLogging();
    logging.doLogging();
  }
}
