ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

all: build-dynamic-debug build-static-debug

.PHONY: build-all
build-all: build-dynamic build-static

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

build-dynamic:
	@cp ../target/release/libcfastlogging.so lib/
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_dynamic examples/console_dynamic/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_add_writer_dynamic examples/console_add_writer_dynamic/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file_dynamic examples/file_dynamic/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file_add_writer_dynamic examples/file_add_writer_dynamic/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/net_unencrypted_one_client examples/net_unencrypted_one_client/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/syslog_dynamic examples/syslog_dynamic/main.go
#	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/callback_dynamic examples/callback_dynamic/main.go

run-console-dynamic: build-dynamic
	@./examples/console_dynamic

build-dynamic-debug:
	@cp ../target/debug/libcfastlogging.so lib/
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_dynamic examples/console_dynamic/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_add_writer_dynamic examples/console_add_writer_dynamic/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file_dynamic examples/file_dynamic/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/file_add_writer_dynamic examples/file_add_writer_dynamic/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/net_unencrypted_one_client examples/net_unencrypted_one_client/main.go
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/syslog_dynamic examples/syslog_dynamic/main.go
#	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/callback_dynamic examples/callback_dynamic/main.go

run-console-dynamic-debug: build-dynamic-debug
	@./examples/console_dynamic

.PHONY: clean
clean:
	rm -rf bin
	rm lib/libcfastlogging.a
	rm lib/libcfastlogging.so
