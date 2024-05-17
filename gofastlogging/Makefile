ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

# PHONY means that it doesn't correspond to a file; it always runs the build commands.

.PHONY: build-all
build-all: build-dynamic build-static build-dynamic-package

.PHONY: run-all
run-all: run-dynamic run-static

.PHONY: build-static
build-static:
	@cp ../target/release/libcfastlogging.a lib/
	go build -o bin/console_static examples/console_static/main.go

.PHONY: run-static
run-static: build-static
	@./examples/console_static

.PHONY: build-dynamic
build-dynamic:
	@cp ../target/release/libcfastlogging.so lib/
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_dynamic examples/console_dynamic/main.go

.PHONY: run-dynamic
run-dynamic: build-dynamic
	@./examples/console_dynamic

.PHONY: build-dynamic-package
build-dynamic-package:
	@cp ../target/release/libcfastlogging.so lib/
	go build -x -ldflags="-r $(ROOT_DIR)lib" -o bin/console_dynamic_package examples/console_dynamic_package/main.go

.PHONY: run-dynamic-package
run-dynamic: build-dynamic-package
	@./examples/console_dynamic_package

.PHONY: clean
clean:
	rm -rf examples/console_dynamic examples/console_static examples/console_dynamic_package
