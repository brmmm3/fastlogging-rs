GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: clean build

clean:
	$(CARGO_BIN) clean
	rm -f ./bin/console_static
	rm -f ./bin/console
	rm -f ./bin/file
	rm -f ./bin/threads

build:
	$(CARGO_BIN) build --release
	$(GCC_BIN) -o ./bin/console_static ./examples/console_static.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console ./examples/console.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file ./examples/file.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/threads ./examples/threads.c -I. -L. -l:../target/release/libcfastlogging.so

build-debug:
	$(CARGO_BIN) build
	$(GCC_BIN) -o ./bin/console_static ./examples/console_static.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console ./examples/console.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file ./examples/file.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/threads ./examples/threads.c -I. -L. -l:../target/debug/libcfastlogging.so
