java_build: lib
	javac org/logging/FastLogging.java

.PHONY: lib

lib:
	cargo build --release
	mkdir -p examples/lib
	cp ../target/release/libjfastlogging_jni.so FastLogging/lib/libjfastlogging.so

.PHONY: lib-debug

lib-debug:
	cargo build
	mkdir -p examples/lib
	cp ../target/debug/libjfastlogging_jni.so FastLogging/lib/libjfastlogging.so
	cp org/logging/FastLogging.java FastLogging/src/main/java/org/logging
