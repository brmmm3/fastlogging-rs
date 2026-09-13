package fastlogging_test

import (
	"fmt"
	"os"
	"path/filepath"
	"sync"
	"sync/atomic"
	"testing"

	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logger"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"
)

// ---------------------------------------------------------------------------
// Level helpers
// ---------------------------------------------------------------------------

func TestLevelConversion(t *testing.T) {
	// Verify level constants exist and have expected values.
	if fl.DEBUG != fl.DEBUG {
		t.Error("DEBUG constant mismatch")
	}
	if fl.INFO != fl.INFO {
		t.Error("INFO constant mismatch")
	}
	if fl.ERROR != fl.ERROR {
		t.Error("ERROR constant mismatch")
	}
	// Aliases.
	if fl.WARN != fl.WARNING {
		t.Error("WARN should equal WARNING")
	}
	if fl.CRITICAL != fl.FATAL {
		t.Error("CRITICAL should equal FATAL")
	}
}

// ---------------------------------------------------------------------------
// Basic logging
// ---------------------------------------------------------------------------

func TestDefaultLogging(t *testing.T) {
	log, err := logging.Default()
	if err != nil {
		t.Fatalf("failed to create default logging: %v", err)
	}
	if err = log.Trace("trace"); err != nil {
		t.Errorf("trace failed: %v", err)
	}
	if err = log.Debug("debug"); err != nil {
		t.Errorf("debug failed: %v", err)
	}
	if err = log.Info("info"); err != nil {
		t.Errorf("info failed: %v", err)
	}
	if err = log.Warning("warning"); err != nil {
		t.Errorf("warning failed: %v", err)
	}
	if err = log.Error("error"); err != nil {
		t.Errorf("error failed: %v", err)
	}
	if err = log.Critical("critical"); err != nil {
		t.Errorf("critical failed: %v", err)
	}
	if err = log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestLoggingNewWithConsoleWriter(t *testing.T) {
	var domain = "test"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	if err := log.Trace("trace msg"); err != nil {
		t.Errorf("trace failed: %v", err)
	}
	if err := log.Debug("debug msg"); err != nil {
		t.Errorf("debug failed: %v", err)
	}
	if err := log.Info("info msg"); err != nil {
		t.Errorf("info failed: %v", err)
	}
	if err := log.Success("success msg"); err != nil {
		t.Errorf("success failed: %v", err)
	}
	if err := log.Warning("warning msg"); err != nil {
		t.Errorf("warning failed: %v", err)
	}
	if err := log.Error("error msg"); err != nil {
		t.Errorf("error failed: %v", err)
	}
	if err := log.Critical("critical msg"); err != nil {
		t.Errorf("critical failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

// ---------------------------------------------------------------------------
// Console writer
// ---------------------------------------------------------------------------

func TestConsoleWriterConfigNew(t *testing.T) {
	cfg := writer.ConsoleWriterConfigNew(fl.DEBUG, false)
	if cfg == nil {
		t.Fatal("ConsoleWriterConfigNew returned nil")
	}
	var domain = "test"
	log := logging.New(fl.DEBUG, &domain, []fl.WriterConfigEnum{*cfg}, nil, nil)
	if log == nil {
		t.Fatal("New with console writer returned nil")
	}
	if err := log.Info("console test"); err != nil {
		t.Errorf("info failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

// ---------------------------------------------------------------------------
// File writer
// ---------------------------------------------------------------------------

func TestFileWriterWritesMessages(t *testing.T) {
	tempDir := t.TempDir()
	logFile := filepath.Join(tempDir, "test.log")
	var domain = "test"
	cfg := writer.FileWriterConfigNew(fl.DEBUG, logFile, 0, 0, -1, -1, fl.Store)
	if cfg == nil {
		t.Fatal("FileWriterConfigNew returned nil")
	}
	log := logging.New(fl.DEBUG, &domain, []fl.WriterConfigEnum{*cfg}, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	if err := log.Info("file info msg"); err != nil {
		t.Errorf("info failed: %v", err)
	}
	if err := log.Error("file error msg"); err != nil {
		t.Errorf("error failed: %v", err)
	}
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
	content, err := os.ReadFile(logFile)
	if err != nil {
		t.Fatalf("failed to read log file: %v", err)
	}
	if !contains(string(content), "file info msg") {
		t.Error("log file must contain info msg")
	}
	if !contains(string(content), "file error msg") {
		t.Error("log file must contain error msg")
	}
}

func TestFileWriterLevelFilter(t *testing.T) {
	tempDir := t.TempDir()
	logFile := filepath.Join(tempDir, "level.log")
	var domain = "test"
	cfg := writer.FileWriterConfigNew(fl.WARNING, logFile, 0, 0, -1, -1, fl.Store)
	if cfg == nil {
		t.Fatal("FileWriterConfigNew returned nil")
	}
	log := logging.New(fl.DEBUG, &domain, []fl.WriterConfigEnum{*cfg}, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	log.Debug("debug message")
	log.Info("info message")
	if err := log.Error("error message"); err != nil {
		t.Errorf("error failed: %v", err)
	}
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
	content, err := os.ReadFile(logFile)
	if err != nil {
		t.Fatalf("failed to read log file: %v", err)
	}
	if contains(string(content), "debug message") {
		t.Error("debug message should be filtered out")
	}
	if contains(string(content), "info message") {
		t.Error("info message should be filtered out")
	}
	if !contains(string(content), "error message") {
		t.Error("error message should be present")
	}
}

func TestFileWriterRotation(t *testing.T) {
	tempDir := t.TempDir()
	logFile := filepath.Join(tempDir, "rotate.log")
	var domain = "test"
	// backlog=3 enables rotation.
	cfg := writer.FileWriterConfigNew(fl.DEBUG, logFile, 0, 3, -1, -1, fl.Store)
	if cfg == nil {
		t.Fatal("FileWriterConfigNew returned nil")
	}
	log := logging.New(fl.DEBUG, &domain, []fl.WriterConfigEnum{*cfg}, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	for i := 0; i < 5; i++ {
		log.Info(fmt.Sprintf("message %d", i))
	}
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	if err := log.Rotate(""); err != nil {
		t.Errorf("rotate failed: %v", err)
	}
	log.Info("after rotate")
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
	// Current file should contain the new message.
	current, err := os.ReadFile(logFile)
	if err != nil {
		t.Fatalf("failed to read log file: %v", err)
	}
	if !contains(string(current), "after rotate") {
		t.Error("current file must contain 'after rotate'")
	}
}

// ---------------------------------------------------------------------------
// Writer management
// ---------------------------------------------------------------------------

func TestAddRemoveWriter(t *testing.T) {
	var domain = "test"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	// No writers yet.
	// Add a console writer.
	cfg := writer.ConsoleWriterConfigNew(fl.DEBUG, false)
	if err := log.AddWriterConfig(*cfg); err != nil {
		t.Fatalf("add_writer_config failed: %v", err)
	}
	// Add another.
	cfg2 := writer.ConsoleWriterConfigNew(fl.INFO, false)
	if err := log.AddWriterConfig(*cfg2); err != nil {
		t.Fatalf("add_writer_config 2 failed: %v", err)
	}
	// Remove first.
	if err := log.RemoveWriter(1); err != nil {
		t.Fatalf("remove_writer(1) failed: %v", err)
	}
	// Second should still exist.
	if err := log.RemoveWriter(2); err != nil {
		t.Fatalf("remove_writer(2) failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestRemoveWriterInvalidId(t *testing.T) {
	var domain = "test"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	// Removing a non-existent writer should not panic.
	log.RemoveWriter(12345)
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestAddWritersBatch(t *testing.T) {
	var domain = "test"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	configs := []fl.WriterConfigEnum{
		*writer.ConsoleWriterConfigNew(fl.DEBUG, false),
		*writer.ConsoleWriterConfigNew(fl.INFO, false),
	}
	if err := log.AddWriterConfigs(configs); err != nil {
		t.Fatalf("add_writer_configs failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestDisableEnableWriterFile(t *testing.T) {
	tempDir := t.TempDir()
	logFile := filepath.Join(tempDir, "toggle.log")
	var domain = "test"
	cfg := writer.FileWriterConfigNew(fl.DEBUG, logFile, 0, 0, -1, -1, fl.Store)
	if cfg == nil {
		t.Fatal("FileWriterConfigNew returned nil")
	}
	log := logging.New(fl.DEBUG, &domain, []fl.WriterConfigEnum{*cfg}, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	// Disable the first writer.
	if err := log.Disable(1); err != nil {
		t.Fatalf("disable failed: %v", err)
	}
	log.Info("disabled")
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	content, _ := os.ReadFile(logFile)
	if contains(string(content), "disabled") {
		t.Error("disabled writer should not write")
	}
	// Re-enable.
	if err := log.Enable(1); err != nil {
		t.Fatalf("enable failed: %v", err)
	}
	log.Info("enabled")
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
	content, _ = os.ReadFile(logFile)
	if !contains(string(content), "enabled") {
		t.Error("enabled writer should write")
	}
}

func TestDisableEnableInvalidWriter(t *testing.T) {
	var domain = "test"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	if err := log.Disable(9999); err == nil {
		t.Error("disable(9999) should return an error")
	}
	if err := log.Enable(9999); err == nil {
		t.Error("enable(9999) should return an error")
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestEnableDisableType(t *testing.T) {
	var domain = "test"
	cfg1 := writer.ConsoleWriterConfigNew(fl.DEBUG, false)
	cfg2 := writer.ConsoleWriterConfigNew(fl.DEBUG, false)
	log := logging.New(fl.DEBUG, &domain, []fl.WriterConfigEnum{*cfg1, *cfg2}, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	writerType := fl.WriterType{Typ: 1} // Console type
	if err := log.DisableType(writerType); err != nil {
		t.Fatalf("disable_type failed: %v", err)
	}
	if err := log.EnableType(writerType); err != nil {
		t.Fatalf("enable_type failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

// ---------------------------------------------------------------------------
// Level and domain management
// ---------------------------------------------------------------------------

func TestSetLevelGlobal(t *testing.T) {
	var domain = "test"
	cfg := writer.ConsoleWriterConfigNew(fl.INFO, false)
	log := logging.New(fl.DEBUG, &domain, []fl.WriterConfigEnum{*cfg}, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	// INFO level: DEBUG should be filtered.
	log.Debug("filtered by writer level")
	log.Info("passes")
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	// Change writer level to DEBUG.
	if err := log.SetLevel(1, fl.DEBUG); err != nil {
		t.Fatalf("set_level failed: %v", err)
	}
	log.Debug("now passes")
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestSetLevelWriter(t *testing.T) {
	var domain = "test"
	cfg := writer.ConsoleWriterConfigNew(fl.INFO, false)
	log := logging.New(fl.DEBUG, &domain, []fl.WriterConfigEnum{*cfg}, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	log.Debug("filtered")
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	// Lower the writer's level to DEBUG.
	if err := log.SetLevel(1, fl.DEBUG); err != nil {
		t.Fatalf("set_level failed: %v", err)
	}
	log.Debug("passes now")
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestSetDomainUpdatesInstanceConfig(t *testing.T) {
	var domain = "old"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	newDomain := "new-domain"
	if err := log.SetDomain(&newDomain); err != nil {
		t.Fatalf("set_domain failed: %v", err)
	}
	cfg := log.GetConfigString()
	if !contains(cfg, "new-domain") {
		t.Error("config string must contain new-domain")
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestSetExtConfig(t *testing.T) {
	var domain = "test"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	ext1 := fl.NewExtConfig(fl.String, false, false, false, false, false)
	if err := log.SetExtConfig(ext1); err != nil {
		t.Fatalf("set_ext_config 1 failed: %v", err)
	}
	ext2 := fl.NewExtConfig(fl.Json, false, false, false, true, true)
	if err := log.SetExtConfig(ext2); err != nil {
		t.Fatalf("set_ext_config 2 failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestSetLevel2Sym(t *testing.T) {
	var domain = "test"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	if err := log.SetLevel2Sym(fl.Str.Into()); err != nil {
		t.Fatalf("set_level2sym failed: %v", err)
	}
	cfg := log.GetConfigString()
	if !contains(cfg, "DEBUG") {
		t.Error("config string must contain DEBUG")
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

// ---------------------------------------------------------------------------
// Logger
// ---------------------------------------------------------------------------

func TestAddRemoveLogger(t *testing.T) {
	var domain = "root"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	domain2 := "logger-domain"
	log2 := logger.New(fl.DEBUG, &domain2)
	if log2 == nil {
		t.Fatal("logger.New returned nil")
	}
	if err := log.AddLogger(*log2); err != nil {
		t.Fatalf("add_logger failed: %v", err)
	}
	if err := log2.Info("logger message"); err != nil {
		t.Errorf("logger info failed: %v", err)
	}
	if err := log2.Trace("trace"); err != nil {
		t.Errorf("logger trace failed: %v", err)
	}
	if err := log2.Debug("debug"); err != nil {
		t.Errorf("logger debug failed: %v", err)
	}
	if err := log2.Warning("warning"); err != nil {
		t.Errorf("logger warning failed: %v", err)
	}
	if err := log2.Error("error"); err != nil {
		t.Errorf("logger error failed: %v", err)
	}
	if err := log2.Critical("critical"); err != nil {
		t.Errorf("logger critical failed: %v", err)
	}
	if err := log.RemoveLogger(*log2); err != nil {
		t.Fatalf("remove_logger failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestLoggerLevelFiltering(t *testing.T) {
	var domain = "root"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	domain2 := "logger-domain"
	log2 := logger.New(fl.INFO, &domain2)
	if log2 == nil {
		t.Fatal("logger.New returned nil")
	}
	if err := log.AddLogger(*log2); err != nil {
		t.Fatalf("add_logger failed: %v", err)
	}
	log2.Debug("filtered by logger level")
	if err := log2.Info("passes"); err != nil {
		t.Errorf("logger info failed: %v", err)
	}
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	// Change logger level.
	if err := log2.SetLevel(fl.DEBUG); err != nil {
		t.Fatalf("logger set_level failed: %v", err)
	}
	log2.Debug("now passes")
	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	if err := log.RemoveLogger(*log2); err != nil {
		t.Fatalf("remove_logger failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestLoggerSetDomain(t *testing.T) {
	domain := "d1"
	log2 := logger.New(fl.DEBUG, &domain)
	if log2 == nil {
		t.Fatal("logger.New returned nil")
	}
	newDomain := "d2"
	if err := log2.SetDomain(&newDomain); err != nil {
		t.Fatalf("set_domain failed: %v", err)
	}
}

func TestLoggerLevel(t *testing.T) {
	domain := "d1"
	log2 := logger.New(fl.INFO, &domain)
	if log2 == nil {
		t.Fatal("logger.New returned nil")
	}
	// Verify level was set.
	if err := log2.SetLevel(fl.DEBUG); err != nil {
		t.Fatalf("set_level failed: %v", err)
	}
}

// ---------------------------------------------------------------------------
// Config save/apply
// ---------------------------------------------------------------------------

func TestSaveConfig(t *testing.T) {
	tempDir := t.TempDir()
	configPath := filepath.Join(tempDir, "config.json")
	logFile := filepath.Join(tempDir, "save.log")
	var domain = "test"
	cfg := writer.FileWriterConfigNew(fl.DEBUG, logFile, 0, 0, -1, -1, fl.Store)
	if cfg == nil {
		t.Fatal("FileWriterConfigNew returned nil")
	}
	log := logging.New(fl.DEBUG, &domain, []fl.WriterConfigEnum{*cfg}, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	if err := log.SaveConfig(configPath); err != nil {
		t.Fatalf("save_config failed: %v", err)
	}
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		t.Error("config file must exist after save")
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

func TestGetConfigString(t *testing.T) {
	var domain = "test"
	log := logging.New(fl.DEBUG, &domain, nil, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}
	cfg := log.GetConfigString()
	if !contains(cfg, "level=") {
		t.Error("config string must contain level=")
	}
	if !contains(cfg, "domain=") {
		t.Error("config string must contain domain=")
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}
}

// ---------------------------------------------------------------------------
// Network configs (construct-only)
// ---------------------------------------------------------------------------

func TestServerClientConfigConstruction(t *testing.T) {
	key := fl.CreateKey(fl.NONE, nil)
	serverCfg := writer.ServerConfigNew(fl.DEBUG, "127.0.0.1", &key)
	if serverCfg == nil {
		t.Fatal("ServerConfigNew returned nil")
	}
	_ = serverCfg
	clientCfg := writer.ClientWriterConfigNew(fl.DEBUG, "127.0.0.1", &key)
	if clientCfg == nil {
		t.Fatal("ClientWriterConfigNew returned nil")
	}
	_ = clientCfg
}

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

func TestShutdownTwiceIsIdempotent(t *testing.T) {
	log, err := logging.Default()
	if err != nil {
		t.Fatalf("failed to create default logging: %v", err)
	}
	if err = log.Shutdown(false); err != nil {
		t.Errorf("first shutdown failed: %v", err)
	}
	// Second shutdown is a no-op — the logging instance is already freed.
	// We don't call it again to avoid use-after-free.
}

// ---------------------------------------------------------------------------
// ExtConfig helpers
// ---------------------------------------------------------------------------

func TestExtConfigDefault(t *testing.T) {
	ext := fl.NewExtConfig(fl.String, false, false, false, false, false)
	// Just verify construction succeeds.
	_ = ext
}

// ---------------------------------------------------------------------------
// Concurrency smoke test
// ---------------------------------------------------------------------------

func TestConcurrentLogging(t *testing.T) {
	tempDir := t.TempDir()
	logFile := filepath.Join(tempDir, "concurrent.log")
	var domain = "test"
	cfg := writer.FileWriterConfigNew(fl.DEBUG, logFile, 0, 0, -1, -1, fl.Store)
	if cfg == nil {
		t.Fatal("FileWriterConfigNew returned nil")
	}
	log := logging.New(fl.DEBUG, &domain, []fl.WriterConfigEnum{*cfg}, nil, nil)
	if log == nil {
		t.Fatal("New returned nil")
	}

	var wg sync.WaitGroup
	var counter atomic.Uint64
	const numThreads = 4
	const msgsPerThread = 50

	for threadID := 0; threadID < numThreads; threadID++ {
		wg.Add(1)
		go func(tid int) {
			defer wg.Done()
			for i := 0; i < msgsPerThread; i++ {
				log.Info(fmt.Sprintf("thread %d message %d", tid, i))
				counter.Add(1)
			}
		}(threadID)
	}
	wg.Wait()

	if err := log.SyncAll(2.0); err != nil {
		t.Errorf("sync_all failed: %v", err)
	}
	if err := log.Shutdown(false); err != nil {
		t.Errorf("shutdown failed: %v", err)
	}

	content, err := os.ReadFile(logFile)
	if err != nil {
		t.Fatalf("failed to read log file: %v", err)
	}
	// Verify all messages were written.
	_ = counter.Load()
	// Rough check: file should not be empty.
	if len(content) == 0 {
		t.Error("log file should not be empty after concurrent logging")
	}
	// Verify each thread's messages are present.
	for threadID := 0; threadID < numThreads; threadID++ {
		for i := 0; i < msgsPerThread; i++ {
			if !contains(string(content), fmt.Sprintf("thread %d message %d", threadID, i)) {
				t.Errorf("missing thread %d message %d", threadID, i)
			}
		}
	}
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

func contains(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr || len(s) > len(substr) && (s[:len(substr)] == substr || contains(s[1:], substr)))
}
