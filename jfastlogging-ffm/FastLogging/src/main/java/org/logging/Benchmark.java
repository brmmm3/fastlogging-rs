package org.logging;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.logging.Level;
import java.util.logging.Logger;

import org.logging.FastLogging.CompressionMethodEnum;
import org.logging.FastLogging.FileWriterConfig;
import org.logging.FastLogging.Logging;

import ch.qos.logback.classic.LoggerContext;
import ch.qos.logback.classic.encoder.PatternLayoutEncoder;
import ch.qos.logback.classic.spi.ILoggingEvent;
import ch.qos.logback.core.FileAppender;
import ch.qos.logback.core.rolling.RollingFileAppender;
import ch.qos.logback.core.rolling.SizeBasedTriggeringPolicy;
import ch.qos.logback.core.util.FileSize;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.core.config.Configuration;
import org.apache.logging.log4j.core.config.builder.api.*;
import org.apache.logging.log4j.core.config.builder.impl.BuiltConfiguration;

import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * Benchmark program comparing jfastlogging-ffm, logback, log4j2 and jul.
 *
 * Mirrors the structure of the Go benchmark:
 * - Short and long messages
 * - No file, plain file, rotating file
 * - Multiple log levels (DEBUG, INFO, WARNING, ERROR, CRITICAL)
 *
 * Build:
 * mvn compile
 * Run:
 * mvn exec:java -Dexec.mainClass="org.logging.Benchmark" [count]
 *
 * Results are written to doc/benchmarks/ (JSON and HTML)
 */
public class Benchmark {

    // ------------------------------------------------------------------
    // Constants
    // ------------------------------------------------------------------

    private static final int MB = 1024 * 1024;
    private static final int CNT_DEFAULT = 5000;
    private static final int NUM_ROUNDS = 10;

    // Log levels (fastlogging values)
    private static final int DEBUG = 10;
    private static final int INFO = 20;
    private static final int WARNING = 30;
    private static final int ERROR = 40;
    private static final int CRITICAL = 50;

    private static final String[] LEVEL_NAMES = { "DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL" };
    private static final int[] CFL_LEVELS = { DEBUG, INFO, WARNING, ERROR, CRITICAL };

    private static String tmpDir;

    static {
        String osName = System.getProperty("os.name");
        if (osName.startsWith("Windows")) {
            tmpDir = "C:\\temp\\jfastlogging_bench";
        } else {
            tmpDir = "/tmp/jfastlogging_bench";
        }
    }

    // ------------------------------------------------------------------
    // Utility helpers
    // ------------------------------------------------------------------

    private static double nowSec() {
        return System.nanoTime() / 1_000_000_000.0;
    }

    private static void ensureDir(String path) {
        File f = new File(path);
        if (f.exists()) {
            f.delete();
        }
        f.mkdirs();
    }

    private static String getPath(String title, String filename) {
        String dir = tmpDir + File.separator + title;
        ensureDir(dir);
        return dir + File.separator + filename;
    }

    private static void cleanupDir(String title) {
        File f = new File(tmpDir + File.separator + title);
        if (f.exists()) {
            for (File child : f.listFiles()) {
                child.delete();
            }
        }
    }

    // ------------------------------------------------------------------
    // Logging work functions
    // Each iteration logs 20 messages (5 levels x 4 rounds)
    // ------------------------------------------------------------------

    private static double loggingWorkCFL(Logging logging, int cnt, String message) {
        double t1 = nowSec();
        for (int i = 0; i < cnt; i++) {
            for (int round = 0; round < 4; round++) {
                logging.critical("Critical " + i + " " + message);
                logging.error("Error " + i + " " + message);
                logging.warning("Warning " + message + " " + i);
                logging.info("Info " + message + " " + i);
                logging.debug("Debug " + message + " " + i);
            }
        }
        return nowSec() - t1;
    }

    private static double loggingWorkLogback(ch.qos.logback.classic.Logger logger, int cnt, String message) {
        double t1 = nowSec();
        for (int i = 0; i < cnt; i++) {
            for (int round = 0; round < 4; round++) {
                logger.error(String.format("Critical %d %s", i, message));
                logger.error(String.format("Error %d %s", i, message));
                logger.warn(String.format("Warning %s %d", message, i));
                logger.info(String.format("Info %s %d", message, i));
                logger.debug(String.format("Debug %s %d", message, i));
            }
        }
        return nowSec() - t1;
    }

    private static double loggingWorkLog4j2(org.apache.logging.log4j.Logger logger, int cnt, String message) {
        double t1 = nowSec();
        for (int i = 0; i < cnt; i++) {
            for (int round = 0; round < 4; round++) {
                logger.error(String.format("Critical %d %s", i, message));
                logger.error(String.format("Error %d %s", i, message));
                logger.warn(String.format("Warning %s %d", message, i));
                logger.info(String.format("Info %s %d", message, i));
                logger.debug(String.format("Debug %s %d", message, i));
            }
        }
        return nowSec() - t1;
    }

    private static double loggingWorkJul(Logger logger, int cnt, String message) {
        double t1 = nowSec();
        for (int i = 0; i < cnt; i++) {
            for (int round = 0; round < 4; round++) {
                logger.log(Level.SEVERE, String.format("Critical %d %s", i, message));
                logger.log(Level.SEVERE, String.format("Error %d %s", i, message));
                logger.log(Level.WARNING, String.format("Warning %s %d", message, i));
                logger.log(Level.INFO, String.format("Info %s %d", message, i));
                logger.log(Level.FINE, String.format("Debug %s %d", message, i));
            }
        }
        return nowSec() - t1;
    }

    private static double benchJni(int cnt, int level, String message, String path, boolean rotate) {
        try {
            String jniClasses = System.getProperty("jni.classes",
                    new File("../../jfastlogging-jni/FastLogging/target/classes").getCanonicalPath());
            File jniLib = new File(System.getProperty("jni.lib",
                    "../../jfastlogging-jni/FastLogging/lib")).getCanonicalFile();
            String java = new File(System.getProperty("java.home"), "bin" + File.separator + "java").getPath();
            String classPath = jniClasses + File.pathSeparator + System.getProperty("java.class.path");
            ProcessBuilder command = new ProcessBuilder(java,
                    "-Djava.library.path=" + jniLib,
                    "-cp", classPath,
                    "org.logging.JniPerf",
                    Integer.toString(cnt), Integer.toString(level), message,
                    path == null ? "" : path, Boolean.toString(rotate));
            command.redirectErrorStream(true);
            Process process = command.start();
            String output;
            try (InputStream stream = process.getInputStream()) {
                output = new String(stream.readAllBytes(), StandardCharsets.UTF_8).trim();
            }
            if (process.waitFor() != 0) {
                return -1.0;
            }
            return Double.parseDouble(output.substring(output.lastIndexOf('\n') + 1));
        } catch (Exception e) {
            return -1.0;
        }
    }

    // ------------------------------------------------------------------
    // jfastlogging-ffm benchmark functions
    // ------------------------------------------------------------------

    private static double benchCFLNoFile(int cnt, int level, String message) {
        Logging logging = new Logging(level, "root");
        double dt = loggingWorkCFL(logging, cnt, message);
        logging.shutdown(false);
        return dt;
    }

    private static double benchCFLFile(int cnt, int level, String path, String message, boolean rotate) {
        int size = 0;
        int backlog = 0;
        if (rotate) {
            size = MB;
            backlog = 8;
        }
        CompressionMethodEnum compression = CompressionMethodEnum.Store;
        FileWriterConfig file = new FileWriterConfig(level, path, size, backlog, 0, 0, compression);
        Logging logging = new Logging(level, "root", file);
        double dt = loggingWorkCFL(logging, cnt, message);
        logging.syncAll(10.0);
        logging.shutdown(false);
        return dt;
    }

    // ------------------------------------------------------------------
    // logback benchmark functions
    // ------------------------------------------------------------------

    private static double benchLogbackNoFile(int cnt, int level, String message) {
        LoggerContext lc = new LoggerContext();
        ch.qos.logback.classic.Logger logger = lc.getLogger("bench");
        logger.setLevel(ch.qos.logback.classic.Level.DEBUG);
        // No appenders = no output
        logger.detachAndStopAllAppenders();
        double dt = loggingWorkLogback(logger, cnt, message);
        lc.stop();
        return dt;
    }

    private static double benchLogbackFile(int cnt, int level, String path, String message, boolean rotate) {
        LoggerContext lc = new LoggerContext();
        ch.qos.logback.classic.Logger logger = lc.getLogger("bench");
        logger.setLevel(ch.qos.logback.classic.Level.DEBUG);

        PatternLayoutEncoder encoder = new PatternLayoutEncoder();
        encoder.setContext(lc);
        encoder.setPattern("%d{yyyy-MM-dd HH:mm:ss} %-5level %msg%n");
        encoder.start();

        if (rotate) {
            RollingFileAppender<ILoggingEvent> appender = new RollingFileAppender<>();
            appender.setContext(lc);
            appender.setFile(path);
            SizeBasedTriggeringPolicy<ILoggingEvent> triggerPolicy = new SizeBasedTriggeringPolicy<>();
            triggerPolicy.setContext(lc);
            triggerPolicy.setMaxFileSize(new FileSize(MB));
            triggerPolicy.start();
            appender.setTriggeringPolicy(triggerPolicy);
            appender.setEncoder(encoder);
            appender.start();
            logger.addAppender(appender);
        } else {
            FileAppender<ILoggingEvent> appender = new FileAppender<>();
            appender.setContext(lc);
            appender.setFile(path);
            appender.setEncoder(encoder);
            appender.start();
            logger.addAppender(appender);
        }

        double dt = loggingWorkLogback(logger, cnt, message);
        lc.stop();
        return dt;
    }

    // ------------------------------------------------------------------
    // log4j2 benchmark functions
    // ------------------------------------------------------------------

    private static double benchLog4j2NoFile(int cnt, int level, String message) {
        // Build a config with no appenders = no output
        ConfigurationBuilder<BuiltConfiguration> builder = ConfigurationBuilderFactory.newConfigurationBuilder();
        builder.add(builder.newLogger("bench", org.apache.logging.log4j.Level.ALL)
                .addAttribute("additivity", false));
        builder.add(builder.newRootLogger(org.apache.logging.log4j.Level.OFF));
        org.apache.logging.log4j.core.LoggerContext ctx = (org.apache.logging.log4j.core.LoggerContext) LogManager
                .getContext(false);
        Configuration oldConfig = ctx.getConfiguration();
        ctx.reconfigure(builder.build());
        org.apache.logging.log4j.Logger logger = LogManager.getLogger("bench");
        double dt = loggingWorkLog4j2(logger, cnt, message);
        // Restore old config
        ctx.reconfigure(oldConfig);
        return dt;
    }

    private static double benchLog4j2File(int cnt, int level, String path, String message, boolean rotate) {
        org.apache.logging.log4j.core.LoggerContext ctx = (org.apache.logging.log4j.core.LoggerContext) LogManager
                .getContext(false);
        Configuration oldConfig = ctx.getConfiguration();
        ConfigurationBuilder<BuiltConfiguration> builder = ConfigurationBuilderFactory.newConfigurationBuilder();
        LayoutComponentBuilder layoutBuilder = builder.newLayout("PatternLayout")
                .addAttribute("pattern", "%d{yyyy-MM-dd HH:mm:ss} %-5level: %msg%n");

        org.apache.logging.log4j.Level logLevel = org.apache.logging.log4j.Level.ALL;

        if (rotate) {
            ComponentBuilder<?> triggeringPolicy = builder.newComponent("Policies")
                    .addComponent(builder.newComponent("SizeBasedTriggeringPolicy").addAttribute("size", "1M"));
            AppenderComponentBuilder appenderBuilder = builder.newAppender("FileLogger", "RollingFile")
                    .addAttribute("fileName", path)
                    .addAttribute("filePattern", path + "-%d{MM-dd-yy}.log.gz")
                    .add(layoutBuilder)
                    .addComponent(triggeringPolicy);
            builder.add(appenderBuilder);
        } else {
            AppenderComponentBuilder appenderBuilder = builder.newAppender("FileLogger", "File")
                    .addAttribute("fileName", path)
                    .add(layoutBuilder);
            builder.add(appenderBuilder);
        }
        builder.add(builder.newLogger("FileLogger", logLevel)
                .add(builder.newAppenderRef("FileLogger"))
                .addAttribute("additivity", false));
        builder.add(builder.newRootLogger(org.apache.logging.log4j.Level.OFF));

        ctx.reconfigure(builder.build());

        org.apache.logging.log4j.Logger logger = LogManager.getLogger("FileLogger");
        double dt = loggingWorkLog4j2(logger, cnt, message);
        // Restore old config
        ctx.reconfigure(oldConfig);
        return dt;
    }

    // ------------------------------------------------------------------
    // jul benchmark functions
    // ------------------------------------------------------------------

    private static double benchJulNoFile(int cnt, int level, String message) {
        // Use a logger with a null handler (no output)
        Logger logger = Logger.getLogger("bench_nolog");
        logger.setUseParentHandlers(false);
        logger.setLevel(Level.ALL);
        // Remove all handlers
        for (java.util.logging.Handler h : logger.getHandlers()) {
            logger.removeHandler(h);
        }
        double dt = loggingWorkJul(logger, cnt, message);
        return dt;
    }

    private static double benchJulFile(int cnt, int level, String path, String message, boolean rotate) {
        Logger logger = Logger.getLogger("bench_file");
        logger.setUseParentHandlers(false);
        logger.setLevel(Level.ALL);
        for (java.util.logging.Handler h : logger.getHandlers()) {
            logger.removeHandler(h);
        }
        try {
            java.util.logging.FileHandler handler;
            if (rotate) {
                handler = new java.util.logging.FileHandler(path, MB, 8, false);
            } else {
                handler = new java.util.logging.FileHandler(path, false);
            }
            handler.setLevel(Level.ALL);
            handler.setFormatter(new java.util.logging.SimpleFormatter());
            logger.addHandler(handler);
        } catch (IOException e) {
            return -1.0;
        }
        double dt = loggingWorkJul(logger, cnt, message);
        for (java.util.logging.Handler h : logger.getHandlers()) {
            h.close();
            logger.removeHandler(h);
        }
        return dt;
    }

    // ------------------------------------------------------------------
    // Measurement
    // ------------------------------------------------------------------

    static class LevelResult {
        double cfl;
        double jni;
        double logback;
        double log4j2;
        double jul;

        LevelResult(double cfl, double jni, double logback, double log4j2, double jul) {
            this.cfl = cfl;
            this.jni = jni;
            this.logback = logback;
            this.log4j2 = log4j2;
            this.jul = jul;
        }
    }

    static class ScenarioResult {
        String title;
        List<LevelResult> levels = new ArrayList<>();
    }

    private static LevelResult measureNoFile(int cnt, int lvIdx, String message) {
        // jfastlogging-ffm
        double cfl = -1.0;
        {
            double total = 0.0;
            int rounds = 0;
            for (int i = 0; i < NUM_ROUNDS; i++) {
                double t = benchCFLNoFile(cnt, CFL_LEVELS[lvIdx], message);
                if (t < 0) {
                    cfl = -1.0;
                    break;
                }
                total += t;
                rounds++;
                if (total > 2.0)
                    break;
            }
            if (rounds > 0)
                cfl = total / rounds;
        }

        // logback
        double logbackT = -1.0;
        {
            double total = 0.0;
            int rounds = 0;
            for (int i = 0; i < NUM_ROUNDS; i++) {
                double t = benchLogbackNoFile(cnt, CFL_LEVELS[lvIdx], message);
                if (t < 0) {
                    logbackT = -1.0;
                    break;
                }
                total += t;
                rounds++;
                if (total > 2.0)
                    break;
            }
            if (rounds > 0)
                logbackT = total / rounds;
        }

        // log4j2
        double log4j2T = -1.0;
        {
            double total = 0.0;
            int rounds = 0;
            for (int i = 0; i < NUM_ROUNDS; i++) {
                double t = benchLog4j2NoFile(cnt, CFL_LEVELS[lvIdx], message);
                if (t < 0) {
                    log4j2T = -1.0;
                    break;
                }
                total += t;
                rounds++;
                if (total > 2.0)
                    break;
            }
            if (rounds > 0)
                log4j2T = total / rounds;
        }

        // jul
        double julT = -1.0;
        {
            double total = 0.0;
            int rounds = 0;
            for (int i = 0; i < NUM_ROUNDS; i++) {
                double t = benchJulNoFile(cnt, CFL_LEVELS[lvIdx], message);
                if (t < 0) {
                    julT = -1.0;
                    break;
                }
                total += t;
                rounds++;
                if (total > 2.0)
                    break;
            }
            if (rounds > 0)
                julT = total / rounds;
        }

        double jni = -1.0;
        double totalJni = 0.0;
        int roundsJni = 0;
        for (int i = 0; i < NUM_ROUNDS; i++) {
            double t = benchJni(cnt, CFL_LEVELS[lvIdx], message, null, false);
            if (t < 0) {
                jni = -1.0;
                break;
            }
            totalJni += t;
            roundsJni++;
            if (totalJni > 2.0)
                break;
        }
        if (roundsJni > 0)
            jni = totalJni / roundsJni;

        return new LevelResult(cfl, jni, logbackT, log4j2T, julT);
    }

    private static LevelResult measureFile(int cnt, int lvIdx, String message, String title, boolean rotate) {
        // jfastlogging-ffm
        double cfl = -1.0;
        {
            double total = 0.0;
            int rounds = 0;
            for (int i = 0; i < NUM_ROUNDS; i++) {
                String path = getPath(title + "_cfl", "logging.log");
                double t = benchCFLFile(cnt, CFL_LEVELS[lvIdx], path, message, rotate);
                if (t < 0) {
                    cfl = -1.0;
                    break;
                }
                total += t;
                rounds++;
                if (total > 2.0)
                    break;
            }
            if (rounds > 0)
                cfl = total / rounds;
            cleanupDir(title + "_cfl");
        }

        // logback
        double logbackT = -1.0;
        {
            double total = 0.0;
            int rounds = 0;
            for (int i = 0; i < NUM_ROUNDS; i++) {
                String path = getPath(title + "_logback", "logging.log");
                double t = benchLogbackFile(cnt, CFL_LEVELS[lvIdx], path, message, rotate);
                if (t < 0) {
                    logbackT = -1.0;
                    break;
                }
                total += t;
                rounds++;
                if (total > 2.0)
                    break;
            }
            if (rounds > 0)
                logbackT = total / rounds;
            cleanupDir(title + "_logback");
        }

        // log4j2
        double log4j2T = -1.0;
        {
            double total = 0.0;
            int rounds = 0;
            for (int i = 0; i < NUM_ROUNDS; i++) {
                String path = getPath(title + "_log4j2", "logging.log");
                double t = benchLog4j2File(cnt, CFL_LEVELS[lvIdx], path, message, rotate);
                if (t < 0) {
                    log4j2T = -1.0;
                    break;
                }
                total += t;
                rounds++;
                if (total > 2.0)
                    break;
            }
            if (rounds > 0)
                log4j2T = total / rounds;
            cleanupDir(title + "_log4j2");
        }

        // jul
        double julT = -1.0;
        {
            double total = 0.0;
            int rounds = 0;
            for (int i = 0; i < NUM_ROUNDS; i++) {
                String path = getPath(title + "_jul", "logging.log");
                double t = benchJulFile(cnt, CFL_LEVELS[lvIdx], path, message, rotate);
                if (t < 0) {
                    julT = -1.0;
                    break;
                }
                total += t;
                rounds++;
                if (total > 2.0)
                    break;
            }
            if (rounds > 0)
                julT = total / rounds;
            cleanupDir(title + "_jul");
        }

        double jni = -1.0;
        double totalJni = 0.0;
        int roundsJni = 0;
        for (int i = 0; i < NUM_ROUNDS; i++) {
            String path = getPath(title + "_jni", "logging.log");
            double t = benchJni(cnt, CFL_LEVELS[lvIdx], message, path, rotate);
            if (t < 0) {
                jni = -1.0;
                break;
            }
            totalJni += t;
            roundsJni++;
            if (totalJni > 2.0)
                break;
        }
        if (roundsJni > 0)
            jni = totalJni / roundsJni;
        cleanupDir(title + "_jni");

        return new LevelResult(cfl, jni, logbackT, log4j2T, julT);
    }

    // ------------------------------------------------------------------
    // Main
    // ------------------------------------------------------------------

    public static void main(String[] args) throws IOException {
        int cnt = CNT_DEFAULT;
        if (args.length > 0) {
            try {
                cnt = Integer.parseInt(args[0]);
            } catch (NumberFormatException e) {
                // use default
            }
        }

        System.out.println("cnt: " + cnt);
        ensureDir(tmpDir);

        String[] msgKeys = { "short", "long" };
        String[] messages = {
                "Message",
                "Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message"
        };

        String[] scenarioNames = { "nolog", "file", "rotate" };
        String[] scenarioTitles = { "No log file", "Log file", "Rotating log file" };
        boolean[] rotateFlags = { false, false, true };
        boolean[] hasFile = { false, true, true };

        // Results storage: [msg_type][scenario]
        ScenarioResult[][] results = new ScenarioResult[2][3];
        for (int m = 0; m < 2; m++) {
            for (int s = 0; s < 3; s++) {
                results[m][s] = new ScenarioResult();
                results[m][s].title = scenarioTitles[s];
            }
        }

        // Run all benchmarks
        for (int m = 0; m < 2; m++) {
            String msgKey = msgKeys[m];
            String message = messages[m];

            for (int s = 0; s < 3; s++) {
                for (int lv = 0; lv < 5; lv++) {
                    String lvName = LEVEL_NAMES[lv];
                    System.out.println("\n### " + msgKey + " " + scenarioNames[s] + " " + lvName);

                    LevelResult lr;
                    if (hasFile[s]) {
                        String title = msgKey + "_" + scenarioNames[s] + "_" + lvName;
                        lr = measureFile(cnt, lv, message, title, rotateFlags[s]);
                    } else {
                        lr = measureNoFile(cnt, lv, message);
                    }

                    System.out.printf("  jfastlogging-ffm: %.4f s%n", lr.cfl);
                    System.out.printf("  jfastlogging-jni: %.4f s%n", lr.jni);
                    System.out.printf("  logback:          %.4f s%n", lr.logback);
                    System.out.printf("  log4j2:           %.4f s%n", lr.log4j2);
                    System.out.printf("  jul:              %.4f s%n", lr.jul);

                    results[m][s].levels.add(lr);
                }
            }
        }

        // Write JSON output
        String benchDir = ".." + File.separator + "doc" + File.separator + "benchmarks";
        new File(benchDir).mkdirs();
        String jsonPath = benchDir + File.separator + "java_benchmark.json";
        Map<String, Object> jsonData = new HashMap<>();
        for (int m = 0; m < 2; m++) {
            Map<String, Object> msgMap = new HashMap<>();
            for (int s = 0; s < 3; s++) {
                Map<String, Object> scenarioMap = new HashMap<>();
                scenarioMap.put("title", results[m][s].title);
                List<Map<String, Double>> levelList = new ArrayList<>();
                for (int lv = 0; lv < 5; lv++) {
                    LevelResult lr = results[m][s].levels.get(lv);
                    Map<String, Double> levelMap = new HashMap<>();
                    levelMap.put("jfastlogging-ffm", lr.cfl);
                    levelMap.put("jfastlogging-jni", lr.jni);
                    levelMap.put("logback", lr.logback);
                    levelMap.put("log4j2", lr.log4j2);
                    levelMap.put("jul", lr.jul);
                    levelList.add(levelMap);
                }
                scenarioMap.put("levels", levelList);
                msgMap.put(scenarioNames[s], scenarioMap);
            }
            jsonData.put(msgKeys[m], msgMap);
        }

        ObjectMapper objectMapper = new ObjectMapper();
        String jsonStr = objectMapper.writerWithDefaultPrettyPrinter().writeValueAsString(jsonData);
        Files.write(Paths.get(jsonPath), jsonStr.getBytes(StandardCharsets.UTF_8));
        System.out.println("\nJSON results written to " + jsonPath);

        // Generate HTML files
        String tmplPath = benchDir + File.separator + "template.html";
        String tmplStr = new String(Files.readAllBytes(Paths.get(tmplPath)), StandardCharsets.UTF_8);

        for (int m = 0; m < 2; m++) {
            for (int s = 0; s < 3; s++) {
                String htmlPath = benchDir + File.separator
                        + scenarioNames[s] + "_" + msgKeys[m] + ".html";

                String content = tmplStr;

                // Replace TITLE
                String titleStr = scenarioTitles[s] + " — " + msgKeys[m];
                content = replacePlaceholder(content, "%(TITLE)s", titleStr);

                // Replace each level placeholder
                for (int lv = 0; lv < 5; lv++) {
                    String placeholder = "%(" + LEVEL_NAMES[lv] + ")s";
                    LevelResult lr = results[m][s].levels.get(lv);
                    String value = String.format(java.util.Locale.US, "%.4f, %.4f, %.4f, %.4f, %.4f",
                            lr.cfl, lr.jni, lr.logback, lr.log4j2, lr.jul);
                    content = replacePlaceholder(content, placeholder, value);
                }

                Files.write(Paths.get(htmlPath), content.getBytes(StandardCharsets.UTF_8));
            }
        }
        System.out.println("HTML files written to doc/benchmarks/");
    }

    private static String replacePlaceholder(String content, String placeholder, String value) {
        int idx = content.indexOf(placeholder);
        if (idx < 0)
            return content;
        return content.substring(0, idx) + value + content.substring(idx + placeholder.length());
    }
}
