#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"
PROFILE="${PROFILE:-release}"

cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --release

mkdir -p "$SCRIPT_DIR/FastLogging/lib"
case "$(uname -s)" in
    Darwin)
        cp "$REPO_ROOT/target/$PROFILE/libjfastlogging_jni.dylib" \
            "$SCRIPT_DIR/FastLogging/lib/libjfastlogging.dylib"
        ;;
    *)
        cp "$REPO_ROOT/target/$PROFILE/libjfastlogging_jni.so" \
            "$SCRIPT_DIR/FastLogging/lib/libjfastlogging.so"
        ;;
esac

mvn -f "$SCRIPT_DIR/FastLogging/pom.xml" clean package
printf 'Created %s\n' "$SCRIPT_DIR/FastLogging/target/FastLogging-0.9.0-jni.jar"
