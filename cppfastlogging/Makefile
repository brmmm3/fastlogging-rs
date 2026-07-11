GXX_BIN  ?= g++
CARGO_BIN ?= cargo

CXXFLAGS = -std=c++17 -I.

run: build-debug

clean:
	rm -rf bin

build:
	$(CARGO_BIN) build --release
	@mkdir -p bin
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/default              ./examples/default.cpp              -L../target/release -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/console              ./examples/console.cpp              -L../target/release -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/console_add_writer   ./examples/console_add_writer.cpp   -L../target/release -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/console_static       ./examples/console_static.cpp       -L../target/release -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/file                 ./examples/file.cpp                 -L../target/release -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/file_add_writer      ./examples/file_add_writer.cpp      -L../target/release -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/syslog               ./examples/syslog.cpp               -L../target/release -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/callback             ./examples/callback.cpp             -L../target/release -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/threads              ./examples/threads.cpp              -L../target/release -l:libcfastlogging.so -lpthread
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.cpp -L../target/release -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/get_server_addresses_ports ./examples/get_server_addresses_ports.cpp -L../target/release -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/ext_config           ./examples/ext_config.cpp           -L../target/release -l:libcfastlogging.so

build-debug:
	$(CARGO_BIN) build
	@mkdir -p bin
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/default              ./examples/default.cpp              -L../target/debug -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/console              ./examples/console.cpp              -L../target/debug -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/console_add_writer   ./examples/console_add_writer.cpp   -L../target/debug -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/console_static       ./examples/console_static.cpp       -L../target/debug -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/file                 ./examples/file.cpp                 -L../target/debug -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/file_add_writer      ./examples/file_add_writer.cpp      -L../target/debug -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/syslog               ./examples/syslog.cpp               -L../target/debug -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/callback             ./examples/callback.cpp             -L../target/debug -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/threads              ./examples/threads.cpp              -L../target/debug -l:libcfastlogging.so -lpthread
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/net_unencrypted_one_client ./examples/net_unencrypted_one_client.cpp -L../target/debug -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/get_server_addresses_ports ./examples/get_server_addresses_ports.cpp -L../target/debug -l:libcfastlogging.so
	$(GXX_BIN) $(CXXFLAGS) -o ./bin/ext_config           ./examples/ext_config.cpp           -L../target/debug -l:libcfastlogging.so
