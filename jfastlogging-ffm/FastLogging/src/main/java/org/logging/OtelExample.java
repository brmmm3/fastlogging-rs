package org.logging;

import org.logging.FastLogging.Logging;
import org.logging.FastLogging.OpenTelemetryWriterConfig;

class OtelExample {
    static void doLogging() {
        OpenTelemetryWriterConfig otel = new OpenTelemetryWriterConfig(
                FastLogging.DEBUG,
                "http://localhost:4318",
                "my-java-ffm-app");
        Logging logging = new Logging(FastLogging.DEBUG, "root", otel);
        logging.debug("Debug Message");
        logging.info("Info Message");
        logging.warning("Warning Message");
        logging.error("Error Message");
        logging.shutdown();
    }

    public static void main(String[] args) {
        doLogging();
    }
}
