package org.logging;

public final class JniPerf {
    public static void main(String[] args) {
        int count = args.length > 0 ? Integer.parseInt(args[0]) : 5000;
        int level = args.length > 1 ? Integer.parseInt(args[1]) : FastLogging.DEBUG;
        String message = args.length > 2 ? args[2] : "Message";
        String path = args.length > 3 ? args[3] : "";
        boolean rotate = args.length > 4 && Boolean.parseBoolean(args[4]);

        FastLogging.Logging logging;
        if (path.isEmpty()) {
            logging = new FastLogging.Logging(level, "bench");
        } else {
            FastLogging.FileWriterConfig file = new FastLogging.FileWriterConfig(
                    level, path, rotate ? 1024 * 1024 : 0, rotate ? 8 : 0, 0, 0,
                    FastLogging.CompressionMethodEnum.Store);
            logging = new FastLogging.Logging(level, "bench", file);
        }

        long start = System.nanoTime();
        for (int i = 0; i < count; i++) {
            for (int round = 0; round < 4; round++) {
                logging.critical("Critical " + i + " " + message);
                logging.error("Error " + i + " " + message);
                logging.warning("Warning " + message + " " + i);
                logging.info("Info " + message + " " + i);
                logging.debug("Debug " + message + " " + i);
            }
        }
        double elapsed = (System.nanoTime() - start) / 1_000_000_000.0;
        if (!path.isEmpty()) {
            logging.syncAll(10.0);
        }
        logging.shutdown(false);
        System.out.printf(java.util.Locale.US, "%.6f%n", elapsed);
    }
}