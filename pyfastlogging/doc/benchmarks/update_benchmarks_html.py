#!/usr/bin/env python3
"""Update const DATA in windows.html from python_windows.json.

The source JSON layout is:
    size -> exc -> sink -> level -> framework

The windows.html DATA layout is:
    sink -> size -> exc -> level -> framework
"""

from __future__ import annotations

import json
import re
from pathlib import Path

SINKS = ["nolog", "file", "rotate"]
SIZES = ["short", "long"]
EXC_MODES = ["noexc", "exc"]
LEVELS = ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]

# Source key in python_windows.json -> key used in windows.html DATA
FRAMEWORK_MAPPING = [
    ("logging", "logging"),
    ("logging-optimized", "logging OPTIMIZED"),
    ("loguru", "loguru"),
    ("fastlogging", "fastlogging"),
    ("fastlogging-threads", "fastlogging THREADS"),
    ("fastlogging-rs", "fastlogging-rs Logging"),
    ("fastlogging-rs-root", "fastlogging-rs root"),
]


def _load_json(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def _transform(source: dict) -> dict:
    """Transform source JSON to the DATA layout used by windows.html."""
    out: dict[str, dict] = {}
    for sink in SINKS:
        out[sink] = {}
        for size in SIZES:
            out[sink][size] = {}
            for exc in EXC_MODES:
                out[sink][size][exc] = {}
                for level in LEVELS:
                    src_level = source[size][exc][sink][level]
                    out[sink][size][exc][level] = {
                        dst_key: src_level[src_key]
                        for src_key, dst_key in FRAMEWORK_MAPPING
                    }
    return out


def _replace_data_block(html_text: str, data_obj: dict) -> tuple[str, int]:
    """Replace 'const DATA = ...;' with the transformed data object."""
    js_data = json.dumps(data_obj, ensure_ascii=False, separators=(",", ":"))
    replacement = f"const DATA = {js_data};"
    pattern = r"const\s+DATA\s*=\s*\{.*?\};"
    return re.subn(pattern, replacement, html_text, count=1, flags=re.DOTALL)


def main() -> int:
    base = Path(__file__).resolve().parent

    linux_json_path = base / "python_linux.json"
    windows_json_path = base / "python_windows.json"
    html_path = base / "benchmarks.html"

    if not linux_json_path.exists():
        raise FileNotFoundError(f"JSON file not found: {linux_json_path}")
    if not windows_json_path.exists():
        raise FileNotFoundError(f"JSON file not found: {windows_json_path}")
    if not html_path.exists():
        raise FileNotFoundError(f"HTML file not found: {html_path}")

    linux_source = _load_json(linux_json_path)
    windows_source = _load_json(windows_json_path)
    linux_transformed = _transform(linux_source)
    windows_transformed = _transform(windows_source)
    transformed = {
        "linux": linux_transformed,
        "windows": windows_transformed,
    }

    with html_path.open("r", encoding="utf-8", newline="") as f:
        html_text = f.read()

    updated_html, replacements = _replace_data_block(html_text, transformed)
    if replacements != 1:
        raise RuntimeError(
            "Could not find exactly one 'const DATA = ...;' block in target HTML"
        )

    with html_path.open("w", encoding="utf-8", newline="") as f:
        f.write(updated_html)

    print(f"Updated {html_path.name} using {linux_json_path.name} and {windows_json_path.name}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
