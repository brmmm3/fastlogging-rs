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
import org.apache.logging.log4j.Level;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.apache.logging.log4j.core.LoggerContext;
import org.apache.logging.log4j.core.config.*;
import org.apache.logging.log4j.core.config.builder.api.*;
import org.apache.logging.log4j.core.config.builder.impl.BuiltConfiguration;

class BenchmarksLog4j2 {
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

    @SuppressWarnings("DuplicatedCode")
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

    private static String getLogPathName(String tmpDirName, String fileName, String title) throws IOException {
        String pathName = null;
        if (fileName != null) {
            String dirName = tmpDirName + "/LOG4J_" + title;
            File file = new File(dirName);
            if (!file.exists()) {
                file.mkdirs();
            }
            pathName = dirName + "/" + fileName;
            if (Files.exists(Paths.get(pathName))) {
                Files.delete(Paths.get(pathName));
            }
        }
        return pathName;
    }

    private static long DoLog4j2(int cnt, int level, String pathName, boolean bRotate,
            boolean bWithException, String message) throws IOException {
        int backlog = 0;
        if (bRotate) {
            backlog = 10;
        }
        // Add Appender
        LoggerContext ctx = (LoggerContext) LogManager.getContext(false);
        Configuration config = ctx.getConfiguration();
        ConfigurationBuilder<BuiltConfiguration> builder = ConfigurationBuilderFactory.newConfigurationBuilder();
        LayoutComponentBuilder layoutBuilder = builder.newLayout("PatternLayout")
                .addAttribute("pattern", "%d{yyyy-MM-dd HH:mm:ss} %-5level: %msg%n%throwable");
        Level logLevel = getLevel(level);
        if (pathName != null) {
            if (bRotate) {
                ComponentBuilder triggeringPolicy = builder.newComponent("Policies")
                        .addComponent(builder.newComponent("SizeBasedTriggeringPolicy").addAttribute("size", "1M"));
                AppenderComponentBuilder appenderBuilder = builder.newAppender("FileLogger", "RollingFile")
                        .addAttribute("fileName", pathName)
                        .addAttribute("filePattern", pathName + "-%d{MM-dd-yy}.log.gz")
                        .add(layoutBuilder)
                        .addComponent(triggeringPolicy);
                builder.add(appenderBuilder);
                builder.add(builder.newLogger("FileLogger", logLevel)
                        .add(builder.newAppenderRef("FileLogger"))
                        .addAttribute("additivity", false));
            } else {
                AppenderComponentBuilder appenderBuilder = builder.newAppender("FileLogger", "File")
                        .addAttribute("fileName", pathName)
                        .add(layoutBuilder);
                builder.add(appenderBuilder);
                builder.add(builder.newLogger("FileLogger", logLevel)
                        .add(builder.newAppenderRef("FileLogger"))
                        .addAttribute("additivity", false));
            }
        }
        ctx.setConfiguration(builder.build());
        ctx.initialize();
        for (String key : config.getAppenders().keySet()) {
            if (key.startsWith("DefaultConsole")) {
                ((DefaultConfiguration) config).removeAppender(key);
            }
        }
        Logger logger = ctx.getLogger("FileLogger");
        if (pathName == null) {
            Configurator.setLevel("FileLogger", Level.OFF);
        }
        Instant start = Instant.now();
        long dt0 = LoggingWork(logger, cnt, bWithException, message);
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
                            String pathName = getLogPathName(tmpDirName, fileName, title);
                            System.out.println("log4j2: " + title);
                            int dtCnt = 0;
                            while (dtCnt++ < 10) {
                                dtTotal += DoLog4j2(cnt, level, pathName, bRotate, bWithException, message);
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
        Files.write(Paths.get("../../log4j2.json"), jacksonData.getBytes());
        System.out.println("Finished.");
    }
}
