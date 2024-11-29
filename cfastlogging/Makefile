GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: build-debug

clean:
	rm -rf bin

build:
	$(CARGO_BIN) build --release
	@mkdir -p bin
	$(GCC_BIN) -o ./bin/default ./examples/default.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console ./examples/console.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console_add_writer ./examples/console_add_writer.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file ./examples/file.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file_add_writer ./examples/file_add_writer.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/syslog ./examples/syslog.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/callback ./examples/callback.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/threads ./examples/threads.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/get_server_addresses_ports ./examples/get_server_addresses_ports.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/get_server_configs ./examples/get_server_configs.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/ext_config ./examples/ext_config.c -I. -L. -l:../target/release/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console_root ./examples/console_root.c -I. -L. -l:../target/release/libcfastlogging.so

build-debug:
	$(CARGO_BIN) build
	@mkdir -p bin
	$(GCC_BIN) -o ./bin/default ./examples/default.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console ./examples/console.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console_add_writer ./examples/console_add_writer.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file ./examples/file.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/file_add_writer ./examples/file_add_writer.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/syslog ./examples/syslog.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/callback ./examples/callback.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/threads ./examples/threads.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/get_server_addresses_ports ./examples/get_server_addresses_ports.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/get_server_configs ./examples/get_server_configs.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/ext_config ./examples/ext_config.c -I. -L. -l:../target/debug/libcfastlogging.so
	$(GCC_BIN) -o ./bin/console_root ./examples/console_root.c -I. -L. -l:../target/debug/libcfastlogging.so

