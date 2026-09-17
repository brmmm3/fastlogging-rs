#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"
PROFILE="${PROFILE:-release}"
unset CARGO_TARGET_DIR

cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --release

mkdir -p "$SCRIPT_DIR/FastLogging/lib"
case "$(uname -s)" in
    Darwin)
        cp "$REPO_ROOT/target/$PROFILE/libjfastlogging_ffm.dylib" \
            "$SCRIPT_DIR/FastLogging/lib/libjfastlogging.dylib"
        ;;
    *)
        cp "$REPO_ROOT/target/$PROFILE/libjfastlogging_ffm.so" \
            "$SCRIPT_DIR/FastLogging/lib/libjfastlogging.so"
        ;;
esac

mvn -f "$SCRIPT_DIR/FastLogging/pom.xml" clean compile
JAR="$SCRIPT_DIR/FastLogging/target/FastLogging-0.9.0-ffm.jar"
jar --create --file "$JAR" \
    -C "$SCRIPT_DIR/FastLogging/target/classes" . \
    -C "$SCRIPT_DIR/FastLogging" lib
printf 'Created %s\n' "$JAR"
