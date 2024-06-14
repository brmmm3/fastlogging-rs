GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: build-debug

clean:
	rm -rf bin

build:
	$(CARGO_BIN) build --release
	mkdir -p bin
	$(GCC_BIN) -o ./bin/console_static ./examples/console_static.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console ./examples/console.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file ./examples/file.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/threads ./examples/threads.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/syslog ./examples/syslog.c -I. -L. -l:../target/release/libcfastlogging.so

build-debug:
	$(CARGO_BIN) build
	mkdir -p bin
	$(GCC_BIN) -o ./bin/console_static ./examples/console_static.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console ./examples/console.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file ./examples/file.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/threads ./examples/threads.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/syslog ./examples/syslog.c -I. -L. -l:../target/debug/libcfastlogging.so
