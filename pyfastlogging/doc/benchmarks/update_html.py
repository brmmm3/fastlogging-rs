#!/usr/bin/env python3
"""Replace the BENCH data in linux.html with values from python_linux/windows.json.

The JSON file has the hierarchy:  length -> exc -> scenario -> level -> lib
The HTML BENCH object has:        scenario -> length -> exc -> level (array of values)

Usage:
    python update_html.py
"""

import json
import re
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

# Ordered mapping from JSON keys to HTML column names.
# The order determines the column order in the generated BENCH arrays.
LIB_MAPPING = [
    ("logging",                "logging"),
    ("logging-optimized",      "logging OPTIMIZED"),
    ("loguru",                 "loguru"),
    ("fastlogging",            "fastlogging"),
    ("fastlogging-threads",    "fastlogging THREADS"),
    ("fastlogging-rs",         "fastlogging-rs Logging"),
    ("fastlogging-rs-root",    "fastlogging-rs root"),
]

SCENARIO_TITLES = {
    "nolog":  "No log file",
    "file":   "Log file",
    "rotate": "Rotating log file",
}

LEVELS = ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]

SCENARIOS = ["nolog", "file", "rotate"]
LENGTHS = ["short", "long"]
EXCS = ["noexc", "exc"]

# Colors for the 7-column layout
COLORS_JS = {
    "logging":                "#f87171",   # red-400
    "logging OPTIMIZED":      "#fb923c",   # orange-400
    "loguru":                 "#fbbf24",   # amber-400
    "fastlogging":            "#4f8cff",   # blue
    "fastlogging THREADS":    "#a855f7",   # purple-500
    "fastlogging-rs Logging": "#22d3ee",   # cyan-400
    "fastlogging-rs root":    "#34d399",   # emerald-400
}

BENCH_DIR = Path(__file__).resolve().parent


# ---------------------------------------------------------------------------
# Generation helpers
# ---------------------------------------------------------------------------

def fmt(v: float) -> str:
    """Format a float to 4 decimal places (matching the existing style)."""
    return f"{v:.4f}"


def generate_libs_js() -> str:
    """Generate the ``const LIBS = [...]`` line."""
    names = ", ".join(f"'{name}'" for _, name in LIB_MAPPING)
    return f"    const LIBS = [{names}];"


def generate_colors_js() -> str:
    """Generate the ``const COLORS = { ... }`` block."""
    lines = ["    const COLORS = {"]
    # Align values for readability.
    max_key_len = max(len(k) for k in COLORS_JS)
    for name, color in COLORS_JS.items():
        pad = max_key_len - len(name)
        lines.append(f"      '{name}':{' ' * (pad + 1)}'{color}',")
    lines.append("    };")
    return "\n".join(lines)


def generate_bench_js(data: dict) -> str:
    """Generate the ``const BENCH = { ... }`` block from the JSON data."""
    lines = ["    const BENCH = {"]

    for scenario in SCENARIOS:
        title = SCENARIO_TITLES[scenario]
        lines.append(f"      {scenario}: {{")
        lines.append(f"        title: '{title}',")

        for length in LENGTHS:
            lines.append(f"        {length}: {{")

            for exc in EXCS:
                lines.append(f"          {exc}: {{")
                lines.append("            libs: LIBS,")

                for level in LEVELS:
                    level_data = data[length][exc][scenario][level]
                    values = [fmt(level_data[json_key]) for json_key, _ in LIB_MAPPING]
                    val_str = ", ".join(values)
                    # CRITICAL uses no space before '[' to match existing style.
                    if level == "CRITICAL":
                        lines.append(f"            {level}:[{val_str}],")
                    else:
                        lines.append(f"            {level}:   [{val_str}],")

                lines.append("          },")
            lines.append("        },")
        lines.append("      },")

    lines.append("    };")
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# Regex replacement
# ---------------------------------------------------------------------------

def replace_block(html: str, pattern: str, replacement: str) -> str:
    """Replace the first block matched by *pattern* with *replacement*."""
    new_html, count = re.subn(pattern, replacement, html, count=1, flags=re.DOTALL)
    if count == 0:
        raise ValueError(f"Pattern not found in HTML:\n{pattern}")
    return new_html


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main(json_path: Path, hmtl_path: Path) -> int:
    # --- Load JSON --------------------------------------------------------
    if not json_path.exists():
        print(f"Error: JSON file not found: {json_path}", file=sys.stderr)
        return 1
    with open(json_path, "r") as f:
        data = json.load(f)

    # Validate that every expected key exists.
    missing = []
    for length in LENGTHS:
        for exc in EXCS:
            for scenario in SCENARIOS:
                for level in LEVELS:
                    for json_key, _ in LIB_MAPPING:
                        try:
                            _ = data[length][exc][scenario][level][json_key]
                        except (KeyError, TypeError):
                            missing.append(f"{length}.{exc}.{scenario}.{level}.{json_key}")
    if missing:
        print("Error: Missing keys in JSON:", file=sys.stderr)
        for m in missing:
            print(f"  - {m}", file=sys.stderr)
        return 1

    # --- Load HTML --------------------------------------------------------
    if not hmtl_path.exists():
        print(f"Error: HTML file not found: {hmtl_path}", file=sys.stderr)
        return 1
    with open(hmtl_path, "r") as f:
        html = f.read()

    # --- Replace COLORS ---------------------------------------------------
    colors_pattern = r"    const COLORS = \{.*?\};"
    html = replace_block(html, colors_pattern, generate_colors_js())

    # --- Replace LIBS -----------------------------------------------------
    # Remove the comment line(s) directly above ``const LIBS`` first, then
    # replace the LIBS constant itself.  We use ``[^\n]*`` for the comment
    # text so DOTALL does not make us swallow preceding blocks.
    html = re.sub(r"    // [^\n]*\n    const LIBS = \[[^\]]*\];",
                  generate_libs_js(), html, count=1)
    # Fallback: plain LIBS without a preceding comment.
    if "const LIBS =" not in html:
        html = replace_block(html, r"    const LIBS = \[[^\]]*\];", generate_libs_js())

    # Also remove any leftover LIBS_5 / LIBS_7 lines if present.
    html = re.sub(r"\n    // Standard 5-column.*?\n    const LIBS_5 = \[.*?\];", "", html, flags=re.DOTALL)
    html = re.sub(r"\n    // 7-column.*?\n    const LIBS_7 = \[.*?\];", "", html, flags=re.DOTALL)
    html = re.sub(r"\n    const LIBS_5 = \[.*?\];", "", html, flags=re.DOTALL)
    html = re.sub(r"\n    const LIBS_7 = \[.*?\];", "", html, flags=re.DOTALL)

    # --- Replace BENCH ----------------------------------------------------
    # The BENCH object is the largest block.  ``.*?`` with DOTALL matches up
    # to the first ``};`` (semicolon) — all inner blocks end with ``},``
    # (comma) so the first ``};`` is the closing brace of BENCH itself.
    bench_pattern = r"    const BENCH = \{.*?\};"
    html = replace_block(html, bench_pattern, generate_bench_js(data))

    # --- Write HTML -------------------------------------------------------
    with open(hmtl_path, "w") as f:
        f.write(html)

    print(f"Updated {hmtl_path.name} with data from {json_path.name}")
    print(f"  Scenarios: {', '.join(SCENARIOS)}")
    print(f"  Libraries: {', '.join(name for _, name in LIB_MAPPING)}")
    print(f"  Datasets:  {len(SCENARIOS) * len(LENGTHS) * len(EXCS)}")
    return 0


if __name__ == "__main__":
    for os_name in ("linux", "windows"):
        json_path = BENCH_DIR / f"python_{os_name}.json"
        hmtl_path = BENCH_DIR / f"{os_name}.html"
        raise SystemExit(main(json_path, hmtl_path))
