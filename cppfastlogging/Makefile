GXX_BIN ?= $(shell which g++)
CARGO_BIN ?= $(shell which cargo)

run: build-debug

clean:
	rm -rf bin

build:
	$(CARGO_BIN) build --release
	@mkdir -p bin
	$(GXX_BIN) -o ./bin/default ./examples/default.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/console ./examples/console.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/console_add_writer ./examples/console_add_writer.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/file ./examples/file.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/file_add_writer ./examples/file_add_writer.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/syslog ./examples/syslog.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/callback ./examples/callback.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/threads ./examples/threads.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/get_server_addresses_ports ./examples/get_server_addresses_ports.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/get_server_configs ./examples/get_server_configs.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/ext_config ./examples/ext_config.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/console_root ./examples/console_root.cpp -I. -L. -l:../target/release/libcfastlogging.so

build-debug:
	$(CARGO_BIN) build
	@mkdir -p bin
	$(GXX_BIN) -o ./bin/default ./examples/default.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/console ./examples/console.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/console_add_writer ./examples/console_add_writer.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/file ./examples/file.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/file_add_writer ./examples/file_add_writer.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/syslog ./examples/syslog.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/callback ./examples/callback.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/threads ./examples/threads.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/get_server_addresses_ports ./examples/get_server_addresses_ports.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/get_server_configs ./examples/get_server_configs.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/ext_config ./examples/ext_config.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/console_root ./examples/console_root.cpp -I. -L. -l:../target/debug/libcfastlogging.so

