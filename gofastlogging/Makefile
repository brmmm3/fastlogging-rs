ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

run: build-debug

clean:
	rm -rf bin

build:
	@mkdir -p bin lib
	@cp ../target/release/libcfastlogging.so lib/
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/default examples/default/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console examples/console/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_add_writer examples/console_add_writer/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file examples/file/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file_add_writer examples/file_add_writer/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/syslog examples/syslog/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/callback examples/callback/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/threads examples/threads/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/net_unencrypted_one_client examples/net_unencrypted_one_client/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/get_server_addresses_ports examples/get_server_addresses_ports/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/get_server_configs examples/get_server_configs/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/ext_config examples/ext_config/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_root examples/console_root/main.go

build-debug:
	@mkdir -p bin lib
	@cp ../target/debug/libcfastlogging.so lib/
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/default examples/default/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console examples/console/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_add_writer examples/console_add_writer/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file examples/file/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file_add_writer examples/file_add_writer/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/syslog examples/syslog/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/callback examples/callback/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/threads examples/threads/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/net_unencrypted_one_client examples/net_unencrypted_one_client/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/get_server_addresses_ports examples/get_server_addresses_ports/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/get_server_configs examples/get_server_configs/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/ext_config examples/ext_config/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_root examples/console_root/main.go

