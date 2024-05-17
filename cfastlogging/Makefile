GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

run: clean build
	./examples/console

clean:
	$(CARGO_BIN) clean
	rm -f ./examples/console

build:
	$(CARGO_BIN) build --release
	$(GCC_BIN) -o ./examples/console ./examples/console.c -I. -L. -l:../target/release/libcfastlogging.so
