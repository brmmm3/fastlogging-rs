package org.logging;

import java.lang.foreign.MemorySegment;
import java.nio.charset.StandardCharsets;

import org.logging.FastLogging.CallbackWriterConfig;
import org.logging.FastLogging.CallbackWriterConfigLog;
import org.logging.FastLogging.Logging;
import org.logging.FastLogging.WriterTypeEnum;

class CallbackExample implements CallbackWriterConfigLog {
  @Override
  public void invoke(int level, MemorySegment domain, long domainLen, MemorySegment message, long messageLen) {
    String domainStr = (domain == null || domainLen <= 0) ? "" : domain.getString(0, StandardCharsets.UTF_8);
    String messageStr = (message == null || messageLen <= 0) ? "" : message.getString(0, StandardCharsets.UTF_8);
    System.out.println(String.format("Java-CB: %d %s: %s", level, domainStr, messageStr));
  }

  void doLogging() {
    CallbackWriterConfig callback = new CallbackWriterConfig(FastLogging.DEBUG, this);
    Logging logging = new Logging(FastLogging.DEBUG, "root", callback);
    logging.setLevel(WriterTypeEnum.Console, FastLogging.DEBUG);
    logging.debug("Debug Message");
    logging.info("Info Message");
    logging.warning("Warning Message");
    logging.error("Error Message");
    // logging.setLevel(WriterTypeEnum.Console, FastLogging.WARNING);
    logging.debug("Debug Message");
    logging.shutdown();
  }

  public static void main(String[] args) {
    CallbackExample logging = new CallbackExample();
    logging.doLogging();
    logging.doLogging();
  }
}
