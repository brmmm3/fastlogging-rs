ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

all: build-debug build-static-debug

.PHONY: build-all
build-all: build build-static

build-static:
	@cp ../target/release/libcfastlogging.a lib/
	go build -o bin/console_static examples/console_static/main.go

run-console-static:
	LD_LIBRARY_PATH=../target/release ./bin/console_static

build-static-debug:
	@cp ../target/debug/libcfastlogging.a lib/
	go build -o bin/console_static examples/console_static/main.go

run-console-static-debug:
	LD_LIBRARY_PATH=../target/debug ./bin/console_static

build:
	@cp ../target/release/libcfastlogging.so lib/
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/default examples/default/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console examples/console/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_add_writer examples/console_add_writer/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file examples/file/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file_add_writer examples/file_add_writer/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/net_unencrypted_one_client examples/net_unencrypted_one_client/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/syslog examples/syslog/main.go
#	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/callback examples/callback/main.go

run-console: build
	@./examples/console

build-debug:
	@cp ../target/debug/libcfastlogging.so lib/
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/default examples/default/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console examples/console/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_add_writer examples/console_add_writer/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file examples/file/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file_add_writer examples/file_add_writer/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/net_unencrypted_one_client examples/net_unencrypted_one_client/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/syslog examples/syslog/main.go
#	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/callback examples/callback/main.go

run-console-debug: build-debug
	@./examples/console

.PHONY: clean
clean:
	rm -rf bin
	rm lib/libcfastlogging.a
	rm lib/libcfastlogging.so
