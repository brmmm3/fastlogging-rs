/*
 * benchmark.go — Benchmark program comparing gofastlogging, slog and logrus.
 *
 * This benchmark mirrors the structure of pyfastlogging/benches/benchmark.py:
 *   - Short and long messages
 *   - No file, plain file, rotating file
 *   - Multiple log levels (DEBUG, INFO, WARNING, ERROR, CRITICAL)
 *
 * Build:
 *   go build -o bin/benchmark examples/benchmark/main.go
 * Run:
 *   ./bin/benchmark [count]
 *
 * Results are written to doc/benchmarks/ (JSON and HTML)
 */

package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"time"

	fl "gofastlogging/fastlogging"
	"gofastlogging/fastlogging/logging"
	"gofastlogging/fastlogging/writer"

	"github.com/sirupsen/logrus"
)

// ------------------------------------------------------------------ //
// Constants                                                          //
// ------------------------------------------------------------------ //

const (
	mb         = 1024 * 1024
	cntDefault = 5000
	numRounds  = 10
)

var tmpDir string

// ------------------------------------------------------------------ //
// Utility helpers                                                    //
// ------------------------------------------------------------------ //

func init() {
	tmpDir = filepath.Join(os.TempDir(), "gofastlogging_bench")
}

func nowSec() float64 {
	return float64(time.Now().UnixNano()) / 1e9
}

func ensureDir(path string) {
	_ = os.RemoveAll(path)
	_ = os.MkdirAll(path, 0755)
}

func getPath(title, filename string) string {
	dir := filepath.Join(tmpDir, title)
	ensureDir(dir)
	return filepath.Join(dir, filename)
}

func cleanupDir(title string) {
	dir := filepath.Join(tmpDir, title)
	_ = os.RemoveAll(dir)
}

// ------------------------------------------------------------------ //
// Level mapping                                                      //
// ------------------------------------------------------------------ //

/*
 * gofastlogging levels:
 *   DEBUG=10, INFO=20, WARNING=30, ERROR=40, CRITICAL=50
 *
 * slog levels:
 *   DEBUG=-4, INFO=0, WARN=4, ERROR=8
 *   slog has no CRITICAL; we map CRITICAL to ERROR.
 *
 * logrus levels:
 *   DEBUG=5, INFO=4, WARN=3, ERROR=2, FATAL=1, PANIC=0
 *   logrus has no CRITICAL; we map CRITICAL to FATAL (but FATAL calls os.Exit,
 *   so we use ERROR instead).
 */

var levelNames = []string{"DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"}
var cflLevels = []uint8{fl.DEBUG, fl.INFO, fl.WARNING, fl.ERROR, fl.CRITICAL}
var slogLevels = []slog.Level{slog.LevelDebug, slog.LevelInfo, slog.LevelWarn, slog.LevelError, slog.LevelError}
var logrusLevels = []logrus.Level{logrus.DebugLevel, logrus.InfoLevel, logrus.WarnLevel, logrus.ErrorLevel, logrus.ErrorLevel}

// ------------------------------------------------------------------ //
// Logging work functions                                             //
// Each iteration logs 20 messages (5 levels x 4 rounds)              //
// ------------------------------------------------------------------ //

func loggingWorkCFL(l *logging.Logging, cnt int, message string) float64 {
	t1 := nowSec()
	for i := 0; i < cnt; i++ {
		for round := 0; round < 4; round++ {
			l.Critical(fmt.Sprintf("Critical %d %s", i, message))
			l.Error(fmt.Sprintf("Error %d %s", i, message))
			l.Warning(fmt.Sprintf("Warning %s %d", message, i))
			l.Info(fmt.Sprintf("Info %s %d", message, i))
			l.Debug(fmt.Sprintf("Debug %s %d", message, i))
		}
	}
	return nowSec() - t1
}

func loggingWorkSlog(l *slog.Logger, cnt int, message string) float64 {
	t1 := nowSec()
	for i := 0; i < cnt; i++ {
		for round := 0; round < 4; round++ {
			l.Error(fmt.Sprintf("Critical %d %s", i, message))
			l.Error(fmt.Sprintf("Error %d %s", i, message))
			l.Warn(fmt.Sprintf("Warning %s %d", message, i))
			l.Info(fmt.Sprintf("Info %s %d", message, i))
			l.Debug(fmt.Sprintf("Debug %s %d", message, i))
		}
	}
	return nowSec() - t1
}

func loggingWorkLogrus(l *logrus.Logger, cnt int, message string) float64 {
	t1 := nowSec()
	for i := 0; i < cnt; i++ {
		for round := 0; round < 4; round++ {
			l.Error(fmt.Sprintf("Critical %d %s", i, message))
			l.Error(fmt.Sprintf("Error %d %s", i, message))
			l.Warn(fmt.Sprintf("Warning %s %d", message, i))
			l.Info(fmt.Sprintf("Info %s %d", message, i))
			l.Debug(fmt.Sprintf("Debug %s %d", message, i))
		}
	}
	return nowSec() - t1
}

// ------------------------------------------------------------------ //
// gofastlogging benchmark functions                                  //
// ------------------------------------------------------------------ //

func benchCFLNoFile(cnt int, level uint8, message string) float64 {
	l := logging.New(level, nil, nil, nil, nil)
	if l == nil {
		return -1.0
	}
	dt := loggingWorkCFL(l, cnt, message)
	l.Shutdown(false)
	return dt
}

func benchCFLFile(cnt int, level uint8, path, message string, rotate bool) float64 {
	var size, backlog uint32
	if rotate {
		size = mb
		backlog = 8
	}
	cfg := writer.FileWriterConfigNew(level, path, size, backlog, -1, -1, fl.Store)
	if cfg == nil {
		return -1.0
	}
	writers := []fl.WriterConfigEnum{*cfg}
	l := logging.New(level, nil, writers, nil, nil)
	if l == nil {
		return -1.0
	}
	dt := loggingWorkCFL(l, cnt, message)
	l.SyncAll(10.0)
	l.Shutdown(false)
	return dt
}

// ------------------------------------------------------------------ //
// slog benchmark functions                                           //
// ------------------------------------------------------------------ //

func benchSlogNoFile(cnt int, level slog.Level, message string) float64 {
	// Discard all output — measures pure logging overhead
	l := slog.New(slog.NewTextHandler(io.Discard, &slog.HandlerOptions{Level: slog.LevelDebug}))
	dt := loggingWorkSlog(l, cnt, message)
	return dt
}

func benchSlogFile(cnt int, level slog.Level, path, message string, rotate bool) float64 {
	f, err := os.OpenFile(path, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
	if err != nil {
		return -1.0
	}
	defer f.Close()
	l := slog.New(slog.NewTextHandler(f, &slog.HandlerOptions{Level: slog.LevelDebug}))
	dt := loggingWorkSlog(l, cnt, message)
	return dt
}

// ------------------------------------------------------------------ //
// logrus benchmark functions                                          //
// ------------------------------------------------------------------ //

func benchLogrusNoFile(cnt int, level logrus.Level, message string) float64 {
	l := logrus.New()
	l.SetOutput(io.Discard)
	l.SetLevel(logrus.DebugLevel)
	dt := loggingWorkLogrus(l, cnt, message)
	return dt
}

func benchLogrusFile(cnt int, level logrus.Level, path, message string, rotate bool) float64 {
	f, err := os.OpenFile(path, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
	if err != nil {
		return -1.0
	}
	defer f.Close()
	l := logrus.New()
	l.SetOutput(f)
	l.SetLevel(logrus.DebugLevel)
	dt := loggingWorkLogrus(l, cnt, message)
	return dt
}

// ------------------------------------------------------------------ //
// Measurement                                                        //
// ------------------------------------------------------------------ //

type levelResult struct {
	CFL    float64 `json:"gofastlogging"`
	Slog   float64 `json:"slog"`
	Logrus float64 `json:"logrus"`
}

type scenarioResult struct {
	Title  string        `json:"title"`
	Levels []levelResult `json:"levels"`
}

func measureNoFile(cnt int, lvIdx int, message string) (float64, float64, float64) {
	// gofastlogging
	{
		total := 0.0
		rounds := 0
		for i := 0; i < numRounds; i++ {
			t := benchCFLNoFile(cnt, cflLevels[lvIdx], message)
			if t < 0 {
				return -1.0, -1.0, -1.0
			}
			total += t
			rounds++
			if total > 2.0 {
				break
			}
		}
		cfl := total / float64(rounds)

		// slog
		total = 0.0
		rounds = 0
		for i := 0; i < numRounds; i++ {
			t := benchSlogNoFile(cnt, slogLevels[lvIdx], message)
			if t < 0 {
				return -1.0, -1.0, -1.0
			}
			total += t
			rounds++
			if total > 2.0 {
				break
			}
		}
		slogT := total / float64(rounds)

		// logrus
		total = 0.0
		rounds = 0
		for i := 0; i < numRounds; i++ {
			t := benchLogrusNoFile(cnt, logrusLevels[lvIdx], message)
			if t < 0 {
				return -1.0, -1.0, -1.0
			}
			total += t
			rounds++
			if total > 2.0 {
				break
			}
		}
		logrusT := total / float64(rounds)

		return cfl, slogT, logrusT
	}
}

func measureFile(cnt int, lvIdx int, message, title string, rotate bool) (float64, float64, float64) {
	// gofastlogging
	cfl := func() float64 {
		total := 0.0
		rounds := 0
		for i := 0; i < numRounds; i++ {
			path := getPath(title+"_cfl", "logging.log")
			t := benchCFLFile(cnt, cflLevels[lvIdx], path, message, rotate)
			if t < 0 {
				return -1.0
			}
			total += t
			rounds++
			if total > 2.0 {
				break
			}
		}
		if rounds == 0 {
			return -1.0
		}
		return total / float64(rounds)
	}()

	// slog
	slogT := func() float64 {
		total := 0.0
		rounds := 0
		for i := 0; i < numRounds; i++ {
			path := getPath(title+"_slog", "logging.log")
			t := benchSlogFile(cnt, slogLevels[lvIdx], path, message, rotate)
			if t < 0 {
				return -1.0
			}
			total += t
			rounds++
			if total > 2.0 {
				break
			}
		}
		if rounds == 0 {
			return -1.0
		}
		return total / float64(rounds)
	}()

	// logrus
	logrusT := func() float64 {
		total := 0.0
		rounds := 0
		for i := 0; i < numRounds; i++ {
			path := getPath(title+"_logrus", "logging.log")
			t := benchLogrusFile(cnt, logrusLevels[lvIdx], path, message, rotate)
			if t < 0 {
				return -1.0
			}
			total += t
			rounds++
			if total > 2.0 {
				break
			}
		}
		if rounds == 0 {
			return -1.0
		}
		return total / float64(rounds)
	}()

	cleanupDir(title + "_cfl")
	cleanupDir(title + "_slog")
	cleanupDir(title + "_logrus")

	return cfl, slogT, logrusT
}

// ------------------------------------------------------------------ //
// Main                                                               //
// ------------------------------------------------------------------ //

func main() {
	cnt := cntDefault
	if len(os.Args) > 1 {
		if v, err := strconv.Atoi(os.Args[1]); err == nil {
			cnt = v
		}
	}

	fmt.Printf("cnt: %d\n", cnt)
	ensureDir(tmpDir)

	msgKeys := []string{"short", "long"}
	messages := []string{
		"Message",
		"Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message Message",
	}

	scenarioNames := []string{"nolog", "file", "rotate"}
	scenarioTitles := []string{"No log file", "Log file", "Rotating log file"}
	rotateFlags := []bool{false, false, true}
	hasFile := []bool{false, true, true}

	// Results storage: [msg_type][scenario]
	results := make([][]scenarioResult, 2)
	for m := range results {
		results[m] = make([]scenarioResult, 3)
	}

	// Run all benchmarks
	for m := 0; m < 2; m++ {
		msgKey := msgKeys[m]
		message := messages[m]

		for s := 0; s < 3; s++ {
			results[m][s].Title = scenarioTitles[s]
			results[m][s].Levels = make([]levelResult, 5)

			for lv := 0; lv < 5; lv++ {
				lvName := levelNames[lv]
				fmt.Printf("\n### %s %s %s\n", msgKey, scenarioNames[s], lvName)

				var dtCFL, dtSlog, dtLogrus float64

				if hasFile[s] {
					title := fmt.Sprintf("%s_%s_%s", msgKey, scenarioNames[s], lvName)
					dtCFL, dtSlog, dtLogrus = measureFile(cnt, lv, message, title, rotateFlags[s])
				} else {
					dtCFL, dtSlog, dtLogrus = measureNoFile(cnt, lv, message)
				}

				fmt.Printf("  gofastlogging: %.4f s\n", dtCFL)
				fmt.Printf("  slog:           %.4f s\n", dtSlog)
				fmt.Printf("  logrus:         %.4f s\n", dtLogrus)

				results[m][s].Levels[lv] = levelResult{
					CFL:    dtCFL,
					Slog:   dtSlog,
					Logrus: dtLogrus,
				}
			}
		}
	}

	// Write JSON output
	jsonPath := filepath.Join("doc", "benchmarks", "go_benchmark.json")
	jf, err := os.Create(jsonPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Cannot open %s for writing: %v\n", jsonPath, err)
		os.Exit(1)
	}

	jsonData := make(map[string]map[string]scenarioResult)
	for m := 0; m < 2; m++ {
		jsonData[msgKeys[m]] = make(map[string]scenarioResult)
		for s := 0; s < 3; s++ {
			jsonData[msgKeys[m]][scenarioNames[s]] = results[m][s]
		}
	}

	enc := json.NewEncoder(jf)
	enc.SetIndent("", "  ")
	enc.Encode(jsonData)
	jf.Close()
	fmt.Printf("\nJSON results written to %s\n", jsonPath)

	// Generate HTML files
	tmplPath := filepath.Join("doc", "benchmarks", "template.html")
	tmpl, err := os.ReadFile(tmplPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Warning: Cannot open template %s, skipping HTML: %v\n", tmplPath, err)
		return
	}
	tmplStr := string(tmpl)

	// Generate one HTML per (msg_type, scenario)
	for m := 0; m < 2; m++ {
		for s := 0; s < 3; s++ {
			htmlPath := filepath.Join("doc", "benchmarks",
				fmt.Sprintf("%s_%s.html", scenarioNames[s], msgKeys[m]))

			content := tmplStr

			// Replace TITLE
			titleBuf := fmt.Sprintf("%s — %s", scenarioTitles[s], msgKeys[m])
			content = replacePlaceholder(content, "%(TITLE)s", titleBuf)

			// Replace each level placeholder
			for lv := 0; lv < 5; lv++ {
				placeholder := fmt.Sprintf("%%(%s)s", levelNames[lv])
				value := fmt.Sprintf("%.4f, %.4f, %.4f",
					results[m][s].Levels[lv].CFL,
					results[m][s].Levels[lv].Slog,
					results[m][s].Levels[lv].Logrus)
				content = replacePlaceholder(content, placeholder, value)
			}

			os.WriteFile(htmlPath, []byte(content), 0644)
		}
	}

	fmt.Printf("HTML files written to doc/benchmarks/\n")

	// Silence unused warnings
	_ = exec.Command
}

// replacePlaceholder replaces the first occurrence of placeholder in content with value.
func replacePlaceholder(content, placeholder, value string) string {
	idx := indexOf(content, placeholder)
	if idx < 0 {
		return content
	}
	return content[:idx] + value + content[idx+len(placeholder):]
}

func indexOf(s, sub string) int {
	for i := 0; i <= len(s)-len(sub); i++ {
		if s[i:i+len(sub)] == sub {
			return i
		}
	}
	return -1
}
