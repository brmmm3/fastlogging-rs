#!/bin/sh

jextract --output java_project/src/main/java -t com.example -l my_rust_lib target/debug/build/jffitest-1c282fc2630adbc1/out/my_rust.h
