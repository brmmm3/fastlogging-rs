GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: build-debug

clean:
	rm -rf bin

build:
	$(CARGO_BIN) build --release
	mkdir -p bin
	$(GCC_BIN) -o ./bin/callback ./examples/callback.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console ./examples/console.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file ./examples/file.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/threads ./examples/threads.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/syslog ./examples/syslog.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/get_server_addresses_ports ./examples/get_server_addresses_ports.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/ext_config ./examples/ext_config.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console_static ./examples/console_static.c -I. -L. -l:../target/release/libcfastlogging.so

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

run-callback:
	LD_LIBRARY_PATH=../target/release ./bin/callback

build-debug:
	$(CARGO_BIN) build
	mkdir -p bin
	$(GCC_BIN) -o ./bin/callback ./examples/callback.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console ./examples/console.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console_add_writer ./examples/console_add_writer.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file ./examples/file.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file_add_writer ./examples/file_add_writer.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/threads ./examples/threads.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/syslog ./examples/syslog.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/get_server_addresses_ports ./examples/get_server_addresses_ports.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/ext_config ./examples/ext_config.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console_static ./examples/console_static.c -I. -L. -l:../target/debug/libcfastlogging.so

run-console-static-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/console_static

run-console-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/console

run-console-add-writer-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/console_add_writer

run-file-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/file

run-file-add-writer-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/file_add_writer

run-threads-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/threads

run-net-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/net_unencrypted_one_client

run-syslog-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/syslog

run-callback-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/callback
