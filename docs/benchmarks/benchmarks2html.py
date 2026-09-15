"""Generate a self-contained benchmark presentation page (index.html).

Loads all *.json benchmark files in this directory, embeds the data into a
single HTML file and renders interactive charts with Chart.js (CDN) plus
readable result tables.

Usage:
    python3 benchmarks2html.py

Output:
    index.html
"""

import glob
import json
import os

OUTPUT = "index.html"
ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))

# Canonical order for log levels on the chart x-axes
LEVEL_ORDER = ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL", "EXCEPTION"]

# Chart colors per framework / library
COLORS = {
    # Python benchmarks
    "logging": "#e4572e",
    "logging-optimized": "#f4a261",
    "loguru": "#e9c46a",
    "fastlogging": "#2e9e46",
    "fastlogging-threads": "#2e86c1",
    "fastlogging-rs": "#8e44ad",
    "fastlogging-rs-root": "#bb6bd9",
    "fastlogging-rs-default": "#bb6bd9",
    # Java benchmarks
    "jfastlogging": "#2e9e46",
    "log4j": "#e67e22",
    "log4j2": "#e74c3c",
    # Native and Go package benchmarks
    "cfastlogging": "#16a085",
    "zlog": "#95a5a6",
    "cppfastlogging": "#2980b9",
    "cxxfastlogging": "#8e44ad",
    "gofastlogging": "#00a6a6",
    "slog": "#f39c12",
    "logrus": "#d35400",
}


def collect():
    """Read every *.json file and return {stem: data}."""
    result = {}
    for f in sorted(glob.glob("*.json")):
        stem = os.path.splitext(os.path.basename(f))[0]
        with open(f, "rb") as fh:
            result[stem] = json.load(fh)
    return result


def build_js(data):
    """Serialize the benchmark data into a JS object literal."""
    return "const BENCH_DATA = " + json.dumps(data, indent=2) + ";\n"


def load_json(path):
    with open(os.path.join(ROOT, path), "rb") as fh:
        return json.load(fh)


def normalize_compact(raw):
    """Normalize C/C++/Go benchmark JSON to the shared level-keyed shape."""
    result = {}
    for size, writers in raw.items():
        result[size] = {}
        for writer, value in writers.items():
            if "levels" in value:
                levels = ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]
                values = {level: row for level, row in zip(levels, value["levels"])}
            else:
                values = {
                    level: row for level, row in value.items() if level != "title"
                }
            result[size][writer] = {
                "title": value.get("title", writer),
                "values": values,
            }
    return result


def collect_package_data():
    packages = []
    compact = [
        ("C", "cfastlogging/doc/benchmarks/c_benchmark.json"),
        ("C++", "cppfastlogging/doc/benchmarks/cpp_benchmark.json"),
        ("Go", "gofastlogging/doc/benchmarks/go_benchmark.json"),
    ]
    for name, path in compact:
        packages.append(
            {
                "name": name,
                "kind": "compact",
                "data": normalize_compact(load_json(path)),
            }
        )

    for platform, path in [
        ("Linux", "pyfastlogging/doc/benchmarks/python_linux.json"),
        ("Windows", "pyfastlogging/doc/benchmarks/python_windows.json"),
    ]:
        packages.append(
            {
                "name": f"Python — {platform}",
                "kind": "python",
                "platform": platform,
                "data": load_json(path),
            }
        )
    return packages


def build_package_js():
    return (
        "const PACKAGE_DATA = " + json.dumps(collect_package_data(), indent=2) + ";\n"
    )


TEMPLATE = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>fastlogging-rs — Benchmark Results</title>
<script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.1/dist/chart.umd.min.js"></script>
<style>
  :root {
    --bg: #0f1420;
    --panel: #1a2233;
    --panel2: #212c42;
    --text: #e6ebf5;
    --muted: #93a1b8;
    --accent: #2e9e46;
    --border: #2c3a55;
  }
  * { box-sizing: border-box; }
  body {
    margin: 0;
    font-family: "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    background: var(--bg);
    color: var(--text);
    line-height: 1.55;
  }
  header {
    padding: 40px 24px 24px;
    text-align: center;
    background: linear-gradient(135deg, #141d31 0%, #1d2a44 55%, #17351f 100%);
    border-bottom: 1px solid var(--border);
  }
  header h1 { margin: 0 0 6px; font-size: 2rem; }
  header p { margin: 0 auto; max-width: 760px; color: var(--muted); }
  .badges { margin-top: 14px; }
  .badge {
    display: inline-block; margin: 4px; padding: 4px 12px; border-radius: 999px;
    background: var(--panel2); border: 1px solid var(--border); font-size: .85rem;
  }
  main { max-width: 1180px; margin: 0 auto; padding: 24px; }
  h2 { margin-top: 48px; border-bottom: 1px solid var(--border); padding-bottom: 8px; }
  h3 { margin: 28px 0 8px; }
  .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(460px, 1fr)); gap: 18px; }
  .card {
    background: var(--panel); border: 1px solid var(--border); border-radius: 10px;
    padding: 16px; overflow: hidden;
  }
  .chart-card { height: 360px; display: flex; flex-direction: column; }
  .chart-card h4 { min-height: 24px; margin: 0 0 10px; font-size: 1rem; }
  .card .sub { margin: 0 0 10px; color: var(--muted); font-size: .85rem; }
  .chart-card canvas { width: 100% !important; height: 300px !important; min-height: 0; }
  table {
    width: 100%; border-collapse: collapse; margin: 10px 0; font-size: .9rem;
    background: var(--panel); border-radius: 8px; overflow: hidden;
  }
  th, td { padding: 7px 10px; border-bottom: 1px solid var(--border); text-align: right; }
  th { background: var(--panel2); text-align: left; }
  td:first-child, th:first-child { text-align: left; }
  tr:last-child td { border-bottom: none; }
  .speedup { color: var(--accent); font-weight: 600; }
  .slow { color: #e74c3c; }
  .note { color: var(--muted); font-size: .88rem; margin: 6px 0 18px; }
  details { background: var(--panel); border: 1px solid var(--border); border-radius: 10px; padding: 12px 16px; margin: 10px 0; }
  summary { cursor: pointer; font-weight: 600; }
  footer { text-align: center; color: var(--muted); padding: 30px; font-size: .85rem; }
  .legend { display: flex; flex-wrap: wrap; gap: 14px; margin: 10px 0 4px; font-size: .85rem; }
  .legend span { display: inline-flex; align-items: center; gap: 6px; }
  .dot { width: 12px; height: 12px; border-radius: 3px; display: inline-block; }
</style>
</head>
<body>
<header>
  <h1>⚡ fastlogging-rs — Benchmark Results</h1>
  <p>Head-to-head comparison of the <strong>fastlogging</strong> logging framework against
    Python, Go, C and C++ logging backends, Apache Log4j and Log4j2, plus the raw Rust core.
     Lower is better — all values are wall-clock seconds for the full log run.</p>
  <div class="badges">
    <span class="badge">Linux</span><span class="badge">Windows&nbsp;10</span>
    <span class="badge">short / long messages</span><span class="badge">with / without exception</span>
    <span class="badge">console&nbsp;(none) / file / rotating file</span>
  </div>
</header>
<main>
  <div id="content"></div>
  <footer>Generated from the raw JSON files in <code>doc/benchmarks/</code> by <code>benchmarks2html.py</code>.</footer>
</main>
<script>
__DATA__
__PACKAGE_DATA__
/* ------------------------------------------------------------------ *
 *  Rendering helpers
 * ------------------------------------------------------------------ */

const LEVELS = ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL", "EXCEPTION"];
const SIZE_LABEL = { short: "Short message", long: "Long message" };
const EXC_LABEL  = { exc: "With exception", noexc: "Without exception" };
const WRITER_LABEL = { nolog: "No output (console, filtered)", file: "File writer", rotate: "Rotating file writer" };

function orderLevels(obj) {
  const out = {};
  for (const lv of LEVELS) if (lv in obj) out[lv] = obj[lv];
  // append any level we did not know about
  for (const k of Object.keys(obj)) if (!(k in out)) out[k] = obj[k];
  return out;
}

function fmt(v) { return v === undefined ? "—" : Number(v).toFixed(3); }

function colorFor(name) { return COLORS[name] || COLORS["fastlogging"]; }

/* Speedup vs. the slowest competing implementation for a single value. */
function ratio(base, other) {
  if (base === undefined || other === undefined || other === 0) return null;
  return base / other;
}

function esc(s) { return String(s).replace(/&/g, "&amp;").replace(/</g, "&lt;"); }

/* ------------------------------------------------------------------ *
 *  Java comparison charts (jfastlogging vs log4j vs log4j2)
 * ------------------------------------------------------------------ */

function javaCharts(osName) {
  const files = {
    jfastlogging: (osName === "Linux" ? "linux_jfastlogging" : "windows_jfastlogging"),
    log4j:        (osName === "Linux" ? "linux_log4j"        : "windows_log4j"),
    log4j2:       (osName === "Linux" ? null                  : "windows_log4j2"),
  };
  const sets = {};
  for (const [name, file] of Object.entries(files)) {
    if (!file || !BENCH_DATA[file]) continue;
    sets[name] = BENCH_DATA[file][osName];
  }
  if (Object.keys(sets).length === 0) return "";

  let html = `<h2>Java benchmarks — ${esc(osName)}</h2><p class="note">jfastlogging (JNI) vs Apache Log4j${sets.log4j2 ? " and Log4j2" : ""}. Values in seconds, lower is better.</p>`;

  for (const size of ["short", "long"]) {
    for (const exc of ["exc", "noexc"]) {
      html += `<h3>${SIZE_LABEL[size]} — ${EXC_LABEL[exc]}</h3><div class="grid">`;
      for (const writer of ["nolog", "file", "rotate"]) {
        const labels = new Set();
        for (const s of Object.values(sets)) {
          const w = s?.[size]?.[exc]?.[writer];
          if (w) Object.keys(orderLevels(w)).forEach((l) => labels.add(l));
        }
        const levelList = LEVELS.filter((l) => labels.has(l));
        const datasets = Object.entries(sets).map(([name, s]) => {
          const w = s?.[size]?.[exc]?.[writer] || {};
          const o = orderLevels(w);
          return {
            label: name,
            data: levelList.map((l) => o[l]),
            backgroundColor: colorFor(name),
          };
        });
        const cardId = "java-" + osName + "-" + size + "-" + exc + "-" + writer;
        html += `<div class="card chart-card"><h4>${WRITER_LABEL[writer]}</h4><canvas id="${cardId}"></canvas></div>`;
        chartsToCreate.push({
          id: cardId,
          type: "bar",
          labels: levelList,
          datasets,
        });
      }
      html += `</div>`;
    }
  }
  return html;
}

/* ------------------------------------------------------------------ *
 *  Python benchmark charts (logging vs fastlogging vs Rust)
 * ------------------------------------------------------------------ */

function pythonCharts() {
  const d = BENCH_DATA["linux_pybenchmarks"];
  if (!d) return "";
  let html = `<h2>Python / Rust benchmarks — Linux</h2><p class="note">Python stdlib <code>logging</code> vs <code>fastlogging</code> Python bindings (single- and multi-threaded) vs the raw <code>fastlogging-rs</code> Rust core. Values in seconds, lower is better.</p>`;

  for (const size of ["short", "long"]) {
    for (const exc of ["noexc", "exc"]) {
      html += `<h3>${SIZE_LABEL[size]} — ${EXC_LABEL[exc]}</h3><div class="grid">`;
      for (const writer of ["nolog", "file", "rotate"]) {
        const w = d?.[size]?.[exc]?.[writer] || {};
        const levelList = LEVELS.filter((l) => w[l]);
        const frameworks = new Set();
        for (const l of levelList) Object.keys(w[l]).forEach((f) => frameworks.add(f));
        const datasets = [...frameworks].map((f) => ({
          label: f,
          data: levelList.map((l) => w[l]?.[f]),
          backgroundColor: colorFor(f),
        }));
        const cardId = "py-" + size + "-" + exc + "-" + writer;
        html += `<div class="card chart-card"><h4>${WRITER_LABEL[writer]}</h4><canvas id="${cardId}"></canvas></div>`;
        chartsToCreate.push({
          id: cardId,
          type: "bar",
          labels: levelList,
          datasets,
        });
      }
      html += `</div>`;
    }
  }
  return html;
}

/* ------------------------------------------------------------------ *
 *  Tables — Java
 * ------------------------------------------------------------------ */

function javaTable(osName) {
  const files = {
    jfastlogging: (osName === "Linux" ? "linux_jfastlogging" : "windows_jfastlogging"),
    log4j:        (osName === "Linux" ? "linux_log4j"        : "windows_log4j"),
    log4j2:       (osName === "Linux" ? null                  : "windows_log4j2"),
  };
  const sets = {};
  for (const [name, file] of Object.entries(files)) {
    if (!file || !BENCH_DATA[file]) continue;
    sets[name] = BENCH_DATA[file][osName];
  }
  if (Object.keys(sets).length === 0) return "";
  const names = Object.keys(sets);
  let html = `<h2>Java benchmark tables — ${esc(osName)}</h2><p class="note">Values in seconds (lower is better). The <span class="speedup">green</span> column is the speedup of jfastlogging over the best competing framework (≥ 1 means faster).</p>`;
  for (const size of ["short", "long"]) {
    for (const exc of ["exc", "noexc"]) {
      for (const writer of ["nolog", "file", "rotate"]) {
        const w = {};
        for (const n of names) w[n] = sets[n]?.[size]?.[exc]?.[writer] || {};
        const levelList = LEVELS.filter((l) => names.some((n) => w[n][l] !== undefined));
        html += `<details open><summary>${SIZE_LABEL[size]} — ${EXC_LABEL[exc]} — ${WRITER_LABEL[writer]}</summary>`;
        html += `<table><thead><tr><th>Level</th>`;
        for (const n of names) html += `<th>${esc(n)} (s)</th>`;
        html += `<th>⏱ speedup</th></tr></thead><tbody>`;
        for (const l of levelList) {
          const jf = w.jfastlogging?.[l];
          const best = Math.min(...names.filter((n) => n !== "jfastlogging").map((n) => w[n][l]).filter((v) => v !== undefined));
          const sp = ratio(jf, best);
          html += `<tr><td>${esc(l)}</td>`;
          for (const n of names) html += `<td>${fmt(w[n][l])}</td>`;
          html += `<td class="${sp !== null && sp >= 1 ? "speedup" : "slow"}">${sp !== null ? "×" + sp.toFixed(1) : "—"}</td></tr>`;
        }
        html += `</tbody></table></details>`;
      }
    }
  }
  return html;
}

/* ------------------------------------------------------------------ *
 *  Tables — Python
 * ------------------------------------------------------------------ */

function pythonTable() {
  const d = BENCH_DATA["linux_pybenchmarks"];
  if (!d) return "";
  let html = `<h2>Python / Rust benchmark tables — Linux</h2><p class="note">Values in seconds (lower is better). The <span class="speedup">green</span> column is the speedup of <code>fastlogging-rs</code> over Python stdlib <code>logging</code>.</p>`;
  for (const size of ["short", "long"]) {
    for (const exc of ["noexc", "exc"]) {
      for (const writer of ["nolog", "file", "rotate"]) {
        const w = d?.[size]?.[exc]?.[writer] || {};
        const levelList = LEVELS.filter((l) => w[l]);
        html += `<details open><summary>${SIZE_LABEL[size]} — ${EXC_LABEL[exc]} — ${WRITER_LABEL[writer]}</summary>`;
        html += `<table><thead><tr><th>Level</th><th>logging (s)</th><th>fastlogging (s)</th><th>fastlogging-threads (s)</th><th>fastlogging-rs (s)</th><th>fastlogging-rs-default (s)</th><th>⏱ speedup rs vs logging</th></tr></thead><tbody>`;
        for (const l of levelList) {
          const cell = w[l] || {};
          const sp = ratio(cell["fastlogging-rs"], cell["logging"]);
          html += `<tr><td>${esc(l)}</td>`;
          for (const k of ["logging", "fastlogging", "fastlogging-threads", "fastlogging-rs", "fastlogging-rs-default"]) {
            html += `<td>${fmt(cell[k])}</td>`;
          }
          html += `<td class="${sp !== null && sp >= 1 ? "speedup" : "slow"}">${sp !== null ? "×" + sp.toFixed(1) : "—"}</td></tr>`;
        }
        html += `</tbody></table></details>`;
      }
    }
  }
  return html;
}

/* ------------------------------------------------------------------ *
 *  C, C++, Go and standalone Python package results
 * ------------------------------------------------------------------ */

function packageChartsAndTables() {
  let html = "";
  for (const pkg of PACKAGE_DATA) {
    html += `<h2>${esc(pkg.name)} benchmarks</h2>`;
    html += `<p class="note">Values in seconds, lower is better. Results use the same chart and table layout for every package.</p>`;
    const sizes = ["short", "long"];
    const excs = pkg.kind === "python" ? ["noexc", "exc"] : [null];
    for (const size of sizes) {
      for (const exc of excs) {
        const source = pkg.kind === "python" ? pkg.data?.[size]?.[exc] : pkg.data?.[size];
        if (!source) continue;
        html += `<h3>${SIZE_LABEL[size]}${exc ? " — " + EXC_LABEL[exc] : ""}</h3><div class="grid">`;
        const writers = ["nolog", "file", "rotate"];
        for (const writer of writers) {
          const entry = source?.[writer];
          if (!entry) continue;
          const values = pkg.kind === "python" ? entry : entry.values;
          const labels = Object.keys(values || {}).filter((level) => LEVELS.includes(level));
          const frameworks = new Set();
          for (const level of labels) Object.keys(values[level] || {}).forEach((name) => frameworks.add(name));
          const datasets = [...frameworks].map((name) => ({
            label: name,
            data: labels.map((level) => values[level]?.[name]),
            backgroundColor: colorFor(name),
          }));
          const cardId = `pkg-${pkg.name.replace(/\\W/g, "-")}-${size}-${exc || "all"}-${writer}`;
          html += `<div class="card chart-card"><h4>${esc(entry.title || WRITER_LABEL[writer])}</h4><canvas id="${cardId}"></canvas></div>`;
          chartsToCreate.push({ id: cardId, type: "bar", labels, datasets });
        }
        html += `</div>`;
        for (const writer of writers) {
          const entry = source?.[writer];
          if (!entry) continue;
          const values = pkg.kind === "python" ? entry : entry.values;
          const labels = Object.keys(values || {}).filter((level) => LEVELS.includes(level));
          const frameworks = new Set();
          for (const level of labels) Object.keys(values[level] || {}).forEach((name) => frameworks.add(name));
          html += `<details open><summary>${SIZE_LABEL[size]}${exc ? " — " + EXC_LABEL[exc] : ""} — ${esc(entry.title || WRITER_LABEL[writer])}</summary><table><thead><tr><th>Level</th>`;
          for (const name of frameworks) html += `<th>${esc(name)} (s)</th>`;
          html += `</tr></thead><tbody>`;
          for (const level of labels) {
            html += `<tr><td>${esc(level)}</td>`;
            for (const name of frameworks) html += `<td>${fmt(values[level]?.[name])}</td>`;
            html += `</tr>`;
          }
          html += `</tbody></table></details>`;
        }
      }
    }
  }
  return html;
}

/* ------------------------------------------------------------------ *
 *  Boot
 * ------------------------------------------------------------------ */

const COLORS = __COLORS__;
const chartsToCreate = [];
const content = document.getElementById("content");

let html = `<h2>Overview</h2><p class="note">The raw data was measured by the benchmark suites in this repository and stored as JSON in <code>doc/benchmarks/</code>. Charts below are grouped by platform, message size, exception usage and writer type.</p>`;

html += pythonCharts();
html += javaCharts("Linux");
html += javaCharts("Windows 10");
html += pythonTable();
html += javaTable("Linux");
html += javaTable("Windows 10");
html += packageChartsAndTables();

content.innerHTML = html;

for (const c of chartsToCreate) {
  const ctx = document.getElementById(c.id);
  if (!ctx) continue;
  new Chart(ctx, {
    type: c.type,
    data: { labels: c.labels, datasets: c.datasets },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: { labels: { color: "#e6ebf5" } },
        tooltip: {
          callbacks: {
            label: (ctx) => ctx.dataset.label + ": " + Number(ctx.raw).toFixed(3) + " s",
          },
        },
      },
      scales: {
        x: { ticks: { color: "#93a1b8", maxRotation: 0, minRotation: 0 }, grid: { color: "#2c3a55" } },
        y: { beginAtZero: true, ticks: { color: "#93a1b8" }, grid: { color: "#2c3a55" }, title: { display: true, text: "seconds (lower is better)", color: "#93a1b8" } },
      },
    },
  });
}
</script>
</body>
</html>
"""


def main():
    data = collect()
    if not data:
        raise SystemExit("No *.json benchmark files found in the current directory.")
    js_data = build_js(data)
    colors_js = json.dumps(COLORS)
    page = (
        TEMPLATE.replace("__DATA__", js_data)
        .replace("__PACKAGE_DATA__", build_package_js())
        .replace("__COLORS__", colors_js)
    )
    outputs = [OUTPUT, os.path.join("..", "index.html")]
    for output in outputs:
        with open(output, "w", encoding="utf-8") as fh:
            fh.write(page)
    print(
        f"Wrote {len(outputs)} unified pages with {len(data)} root and {len(collect_package_data())} package datasets embedded."
    )


if __name__ == "__main__":
    main()
