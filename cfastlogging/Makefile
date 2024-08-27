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

run-console-static:
	LD_LIBRARY_PATH=../target/release ./bin/console_static

run-console:
	LD_LIBRARY_PATH=../target/release ./bin/console

run-file:
	LD_LIBRARY_PATH=../target/release ./bin/file

run-threads:
	LD_LIBRARY_PATH=../target/release ./bin/threads

run-net:
	LD_LIBRARY_PATH=../target/release ./bin/net_unencrypted_one_client

run-syslog:
	LD_LIBRARY_PATH=../target/release ./bin/syslog

build-debug:
	$(CARGO_BIN) build
	mkdir -p bin
	$(GCC_BIN) -o ./bin/console ./examples/console.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file ./examples/file.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/threads ./examples/threads.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/syslog ./examples/syslog.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console_static ./examples/console_static.c -I. -L. -l:../target/debug/libcfastlogging.so

run-console-static-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/console_static

run-console-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/console

run-file-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/file

run-threads-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/threads

run-net-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/net_unencrypted_one_client

run-syslog-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/syslog
