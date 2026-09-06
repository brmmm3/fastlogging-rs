package org.logging;

import junit.framework.TestCase;

/**
 * Unit tests for the pure-Java portions of the {@link FastLogging} class.
 */
public class FastLoggingTest extends TestCase {

    public FastLoggingTest(String testName) {
        super(testName);
    }

    public void testLevel2SymAllLevels() {
        assertEquals("NOLOG", FastLogging.Level2Sym(FastLogging.NOLOG));
        assertEquals("EXCEPTION", FastLogging.Level2Sym(FastLogging.EXCEPTION));
        assertEquals("CRITICAL", FastLogging.Level2Sym(FastLogging.CRITICAL));
        assertEquals("ERROR", FastLogging.Level2Sym(FastLogging.ERROR));
        assertEquals("WARNING", FastLogging.Level2Sym(FastLogging.WARNING));
        assertEquals("SUCCESS", FastLogging.Level2Sym(FastLogging.SUCCESS));
        assertEquals("INFO", FastLogging.Level2Sym(FastLogging.INFO));
        assertEquals("DEBUG", FastLogging.Level2Sym(FastLogging.DEBUG));
        assertEquals("TRACE", FastLogging.Level2Sym(FastLogging.TRACE));
        assertEquals("NOTSET", FastLogging.Level2Sym(FastLogging.NOTSET));
    }

    public void testLevel2SymAliases() {
        // FATAL is an alias for CRITICAL, WARN is an alias for WARNING
        assertEquals("CRITICAL", FastLogging.Level2Sym(FastLogging.FATAL));
        assertEquals("WARNING", FastLogging.Level2Sym(FastLogging.WARN));
    }

    public void testLevel2SymUnknownLevel() {
        // Levels not covered by the switch return "?"
        assertEquals("?", FastLogging.Level2Sym(55));
        assertEquals("?", FastLogging.Level2Sym(15));
        assertEquals("?", FastLogging.Level2Sym(1));
        assertEquals("?", FastLogging.Level2Sym(-1));
    }

    public void testLevelConstants() {
        assertEquals(100, FastLogging.NOLOG);
        assertEquals(60, FastLogging.EXCEPTION);
        assertEquals(50, FastLogging.CRITICAL);
        assertEquals(40, FastLogging.ERROR);
        assertEquals(30, FastLogging.WARNING);
        assertEquals(25, FastLogging.SUCCESS);
        assertEquals(20, FastLogging.INFO);
        assertEquals(10, FastLogging.DEBUG);
        assertEquals(5, FastLogging.TRACE);
        assertEquals(0, FastLogging.NOTSET);
    }

    public void testMessageStructEnum() {
        assertEquals(0, FastLogging.MessageStructEnum.String.getValue());
        assertEquals(1, FastLogging.MessageStructEnum.Json.getValue());
        assertEquals(2, FastLogging.MessageStructEnum.Xml.getValue());
    }

    public void testWriterTypeEnum() {
        assertEquals(0, FastLogging.WriterTypeEnum.Root.getValue());
        assertEquals(1, FastLogging.WriterTypeEnum.Console.getValue());
        assertEquals(2, FastLogging.WriterTypeEnum.File.getValue());
        assertEquals(3, FastLogging.WriterTypeEnum.Client.getValue());
        assertEquals(4, FastLogging.WriterTypeEnum.Server.getValue());
        assertEquals(5, FastLogging.WriterTypeEnum.Syslog.getValue());
    }

    public void testCompressionMethodEnum() {
        assertEquals(0, FastLogging.CompressionMethodEnum.Store.getValue());
        assertEquals(1, FastLogging.CompressionMethodEnum.Deflate.getValue());
        assertEquals(2, FastLogging.CompressionMethodEnum.Zstd.getValue());
        assertEquals(3, FastLogging.CompressionMethodEnum.Lzma.getValue());
    }

    public void testEncryptionMethodEnum() {
        assertEquals(0, FastLogging.EncryptionMethod.NONE.getValue());
        assertEquals(1, FastLogging.EncryptionMethod.AuthKey.getValue());
        assertEquals(2, FastLogging.EncryptionMethod.AES.getValue());
    }
}
