java_build: lib
	javac org/logging/FastLogging.java

.PHONY: lib

lib:
	cargo build --release
	mkdir -p examples/lib
	cp ../target/release/libjfastlogging.so FastLogging/lib

.PHONY: lib-debug

lib-debug:
	cargo build
	mkdir -p examples/lib
	cp ../target/debug/libjfastlogging.so FastLogging/lib
	cp org/logging/FastLogging.java FastLogging/src/main/java/org/logging
