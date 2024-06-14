GXX_BIN ?= $(shell which g++)
CARGO_BIN ?= $(shell which cargo)

run: build-debug

clean:
	rm -rf bin

build:
	$(CARGO_BIN) build --release
	mkdir -p bin
	$(GXX_BIN) -o ./bin/console_static ./examples/console_static.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/console ./examples/console.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/file ./examples/file.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/threads ./examples/threads.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.cpp -I. -L. -l:../target/release/libcfastlogging.so
	$(GXX_BIN) -o ./bin/syslog ./examples/syslog.cpp -I. -L. -l:../target/release/libcfastlogging.so

build-debug:
	$(CARGO_BIN) build
	mkdir -p bin
	$(GXX_BIN) -o ./bin/console_static ./examples/console_static.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/console ./examples/console.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/file ./examples/file.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/threads ./examples/threads.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.cpp -I. -L. -l:../target/debug/libcfastlogging.so
	$(GXX_BIN) -o ./bin/syslog ./examples/syslog.cpp -I. -L. -l:../target/debug/libcfastlogging.so
