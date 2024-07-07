package org.logging;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.time.Duration;
import java.time.Instant;
import java.util.HashMap;
import java.util.Map;
import java.io.File;
import java.io.IOException;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.log4j.*;

class BenchmarksLog4j {
    // Log levels
    public static final int NOLOG = 70;
    public static final int EXCEPTION = 60;
    public static final int CRITICAL = 50;
    public static final int FATAL = CRITICAL;
    public static final int ERROR = 40;
    public static final int WARNING = 30;
    public static final int WARN = WARNING;
    public static final int INFO = 20;
    public static final int DEBUG = 10;
    public static final int NOTSET = 0;

    static public String Level2Sym(int level) {
        switch (level) {
            case NOLOG:
                return "NOLOG";
            case EXCEPTION:
                return "EXCEPTION";
            case CRITICAL:
                return "CRITICAL";
            case ERROR:
                return "ERROR";
            case WARNING:
                return "WARNING";
            case INFO:
                return "INFO";
            case DEBUG:
                return "DEBUG";
            case NOTSET:
                return "NOTSET";
        }
        return "?";
    }

    private static Level getLevel(int level) {
        Level logLevel = Level.ALL;
        switch (level) {
            case NOLOG:
                return Level.OFF;
            case EXCEPTION:
                return Level.FATAL;
            case CRITICAL:
                return Level.FATAL;
            case ERROR:
                return Level.ERROR;
            case WARNING:
                return Level.WARN;
            case INFO:
                return Level.INFO;
            case DEBUG:
                return Level.DEBUG;
            case NOTSET:
                return Level.ALL;
        }
        return logLevel;
    }

    private static long LoggingWork(Logger logging, int cnt, boolean bWithException, String message) {
        Instant start = Instant.now();
        for (int i = 0; i < cnt; i++) {
            logging.fatal(String.format("Fatal %d %s", i, message));
            logging.error(String.format("Error %d %s", i, message));
            logging.warn(String.format("Warning %d %s", i, message));
            logging.info(String.format("Info %d %s", i, message));
            logging.debug(String.format("Debug %d %s", i, message));
            logging.fatal(String.format("Fatal %d %s", i, message));
            logging.error(String.format("Error %d %s", i, message));
            logging.warn(String.format("Warning %d %s", i, message));
            logging.info(String.format("Info %d %s", i, message));
            logging.debug(String.format("Debug %d %s", i, message));
            logging.fatal(String.format("Fatal %d %s", i, message));
            logging.error(String.format("Error %d %s", i, message));
            logging.warn(String.format("Warning %d %s", i, message));
            logging.info(String.format("Info %d %s", i, message));
            logging.debug(String.format("Debug %d %s", i, message));
            logging.fatal(String.format("Fatal %d %s", i, message));
            logging.error(String.format("Error %d %s", i, message));
            logging.warn(String.format("Warning %d %s", i, message));
            logging.info(String.format("Info %d %s", i, message));
            logging.debug(String.format("Debug %d %s", i, message));
            if (bWithException) {
                try {
                    @SuppressWarnings("unused")
                    int x = 1 / 0;
                } catch (ArithmeticException e) {
                    logging.fatal(e.getMessage());
                }
            }
        }
        Instant end = Instant.now();
        Duration dt = Duration.between(start, end);
        return dt.toMillis();
    }

    private static String getTitle(String msg, String fileName, boolean bRotate, boolean bWithException, int level) {
        String title = msg.toUpperCase();
        if (fileName == null) {
            title += "_NO_FILE";
        } else {
            title += "_FILE";
        }
        if (bRotate) {
            title += "_ROTATE";
        }
        if (bWithException) {
            title += "_EXC";
        }
        title += "_" + Level2Sym(level);
        return title;
    }

    private static String getLogPathName(String tmpDirName, String loggerName, String fileName, String title) throws IOException {
        String pathName = null;
        if (fileName != null) {
            String dirName = tmpDirName + "/" + loggerName + "/" + title;
            File file = new File(dirName);
            if (!file.exists()) {
                file.mkdirs();
            }
            pathName = dirName + "/" + fileName;
        }
        return pathName;
    }

    private static long DoLog4j(int cnt, int level, String pathName, boolean bRotate,
            boolean bWithException, String message) throws IOException {
        int backlog = 0;
        if (bRotate) {
            backlog = 10;
        }
        // Initialize Logger log2j
        Logger logging = Logger.getLogger("root");
        logging.removeAllAppenders();
        Level logLevel = getLevel(level);
        logging.setLevel(logLevel);
        PatternLayout layout = new PatternLayout("%d{yyyy-MM-dd HH:mm:ss} %-5p %c{1}:%L - %m%n");
        if (pathName != null) {
            if (bRotate) {
                RollingFileAppender appender = new RollingFileAppender(layout, pathName);
                appender.setMaxFileSize("1MB");
                appender.setMaxBackupIndex(backlog);
                logging.addAppender(appender);
            } else {
                FileAppender appender = new FileAppender(layout, pathName);
                logging.addAppender(appender);
            }
        }
        Instant start = Instant.now();
        long dt0 = LoggingWork(logging, cnt, bWithException, message);
        Instant end = Instant.now();
        Duration dt = Duration.between(start, end);
        System.out.printf("  total: %f %f%n", (float) dt0 / 1000.0, (float) dt.toMillis() / 1000.0);
        return dt.toMillis();
    }

    public static void main(String[] args) throws IOException {
        int cnt = 5000;
        System.out.println("cnt: " + cnt);
        @SuppressWarnings("rawtypes")
        Map<String, Map> dtAllJson = new HashMap<>();
        Map<String, String> msgMessage = new HashMap<String, String>() {
            {
                put("short", "Message");
                put("long",
                        "Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message");
            }
        };
        String osName = System.getProperty("os.name");
        String tmpDirName;
        if (osName.startsWith("Windows")) {
            tmpDirName = "C:\\temp\\log4j";
        } else {
            tmpDirName = "/tmp/log4j";
        }
        msgMessage.forEach((msg, message) -> {
            @SuppressWarnings("rawtypes")
            Map<String, Map> dtAllJsonMsg = new HashMap<>();
            dtAllJson.put(msg, dtAllJsonMsg);
            boolean[] falseTrue = { false, true };
            for (boolean bWithException : falseTrue) {
                @SuppressWarnings("rawtypes")
                Map<String, Map> dtAllJsonMsgExc = new HashMap<>();
                if (bWithException) {
                    dtAllJsonMsg.put("exc", dtAllJsonMsgExc);
                } else {
                    dtAllJsonMsg.put("noexc", dtAllJsonMsgExc);
                }
                String[][] titleNameFileNamebRotate = {
                        { "No log file", "nolog", null, null },
                        { "Log file", "file", "logging.log", null },
                        { "Rotating log file", "rotate", "logging.log", "rotate" }
                };
                for (var tnfr : titleNameFileNamebRotate) {
                    Map<String, Double> dtAllJsonMsgExcName = new HashMap<>();
                    dtAllJsonMsgExc.put(tnfr[1], dtAllJsonMsgExcName);
                    int[] levels = { DEBUG, INFO, WARNING, ERROR, CRITICAL, EXCEPTION };
                    for (int level : levels) {
                        boolean bRotate = tnfr[3] != null;
                        long dtTotal = 0;
                        try {
                            String fileName = tnfr[2];
                            String title = getTitle(msg, fileName, bRotate, bWithException, level);
                            String pathName = getLogPathName(tmpDirName, "log4j", fileName, title);
                            System.out.println("log4j: " + title);
                            int dtCnt = 0;
                            while (dtCnt++ < 10) {
                                dtTotal += DoLog4j(cnt, level, pathName, bRotate, bWithException, message);
                                if (dtTotal > 2000) {
                                    break;
                                }
                            }
                            dtTotal /= dtCnt;
                        } catch (IOException e) {
                            throw new RuntimeException(e);
                        }
                        dtAllJsonMsgExcName.put(Level2Sym(level), (float) dtTotal / 1000.0);
                    }
                }
            }
        });
        @SuppressWarnings("rawtypes")
        Map<String, Map> dtAllOs = new HashMap<>();
        dtAllOs.put(osName, dtAllJson);
        ObjectMapper objectMapper = new ObjectMapper();
        String jacksonData = objectMapper.writerWithDefaultPrettyPrinter().writeValueAsString(dtAllOs);
        Files.write(Paths.get("../../log4j.json"), jacksonData.getBytes());
        System.out.println("Finished.");
    }
}
