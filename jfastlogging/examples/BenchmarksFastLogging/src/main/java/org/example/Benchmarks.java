package org.example;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Duration;
import java.time.Instant;
import java.util.HashMap;
import java.util.Map;

import org.logging.FastLogging;
import org.logging.FastLogging.CompressionMethodEnum;
import org.logging.FastLogging.ConsoleWriterConfig;
import org.logging.FastLogging.FileWriterConfig;
import org.logging.FastLogging.Logging;

import com.fasterxml.jackson.databind.ObjectMapper;

class Benchmarks {
    private static long LoggingWork(Logging logging, int cnt, boolean bWithException, String message) {
        Instant start = Instant.now();
        for (int i = 0; i < cnt; i++) {
            logging.fatal(String.format("Fatal %d %s", i, message));
            logging.error(String.format("Error %d %s", i, message));
            logging.warning(String.format("Warning %d %s", i, message));
            logging.success(String.format("Success %d %s", i, message));
            logging.info(String.format("Info %d %s", i, message));
            logging.debug(String.format("Debug %d %s", i, message));
            logging.trace(String.format("Trace %d %s", i, message));
            logging.fatal(String.format("Fatal %d %s", i, message));
            logging.error(String.format("Error %d %s", i, message));
            logging.warning(String.format("Warning %d %s", i, message));
            logging.success(String.format("Success %d %s", i, message));
            logging.info(String.format("Info %d %s", i, message));
            logging.debug(String.format("Debug %d %s", i, message));
            logging.trace(String.format("Trace %d %s", i, message));
            logging.fatal(String.format("Fatal %d %s", i, message));
            logging.error(String.format("Error %d %s", i, message));
            logging.warning(String.format("Warning %d %s", i, message));
            logging.success(String.format("Success %d %s", i, message));
            logging.info(String.format("Info %d %s", i, message));
            logging.debug(String.format("Debug %d %s", i, message));
            logging.trace(String.format("Trace %d %s", i, message));
            logging.fatal(String.format("Fatal %d %s", i, message));
            logging.error(String.format("Error %d %s", i, message));
            logging.warning(String.format("Warning %d %s", i, message));
            logging.success(String.format("Success %d %s", i, message));
            logging.info(String.format("Info %d %s", i, message));
            logging.debug(String.format("Debug %d %s", i, message));
            logging.trace(String.format("Trace %d %s", i, message));
            if (bWithException) {
                try {
                    @SuppressWarnings("unused")
                    int x = 1 / 0;
                } catch (ArithmeticException e) {
                    logging.exception(e.getMessage());
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
        title += "_" + FastLogging.Level2Sym(level);
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
        }
        return pathName;
    }

    private static long DoFastLoggingRsDefault(int cnt, int level, String pathName, boolean bRotate,
            boolean bWithException, String message) throws IOException {
        int maxSize = 0;
        int backlog = 0;
        int timeout = 0;
        int time = 0;
        CompressionMethodEnum compression = CompressionMethodEnum.Store;
        if (bRotate) {
            maxSize = 1024 * 1024;
            backlog = 10;
        }
        // Initialize Logger jfastlogging
        ConsoleWriterConfig console = new ConsoleWriterConfig(level, true);
        FileWriterConfig file = null;
        if (pathName != null) {
            Path path = Paths.get(pathName);
            if (Files.exists(path)) {
                Files.delete(path);
            }
            file = new FileWriterConfig(level, pathName, maxSize, backlog, timeout, time, compression);
        }
        Logging logging = new Logging(level, "root", console, file);
        Instant start = Instant.now();
        long dt0 = LoggingWork(logging, cnt, bWithException, message);
        logging.shutdown();
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
            tmpDirName = "C:\\temp\\jfastlogging";
        } else {
            tmpDirName = "/tmp/jfastlogging";
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
                    int[] levels = { FastLogging.DEBUG, FastLogging.INFO, FastLogging.WARNING, FastLogging.ERROR,
                            FastLogging.CRITICAL, FastLogging.EXCEPTION };
                    for (int level : levels) {
                        boolean bRotate = tnfr[3] != null;
                        long dtTotal = 0;
                        try {
                            String fileName = tnfr[2];
                            String title = getTitle(msg, fileName, bRotate, bWithException, level);
                            String pathName = getLogPathName(tmpDirName, fileName, title);
                            System.out.println("fastlogging-rs: " + title);
                            int dtCnt = 0;
                            while (dtCnt++ < 10) {
                                dtTotal += DoFastLoggingRsDefault(cnt, level, pathName, bRotate, bWithException,
                                        message);
                                if (dtTotal > 2000) {
                                    break;
                                }
                            }
                            dtTotal /= dtCnt;
                        } catch (IOException e) {
                            throw new RuntimeException(e);
                        }
                        dtAllJsonMsgExcName.put(FastLogging.Level2Sym(level), (float) dtTotal / 1000.0);
                    }
                }
            }
        });
        Map<String, Map> dtAllOs = new HashMap<>();
        dtAllOs.put(osName, dtAllJson);
        ObjectMapper objectMapper = new ObjectMapper();
        String jacksonData = objectMapper.writerWithDefaultPrettyPrinter().writeValueAsString(dtAllOs);
        Files.write(Paths.get("../../jfastlogging.json"), jacksonData.getBytes());
    }
}
