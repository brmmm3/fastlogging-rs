GXX_BIN ?= $(shell which g++)

run: build
	./examples/console

clean:
	$(CARGO_BIN) clean
	rm -f ./examples/console

build:
	$(GXX_BIN) -o ./examples/console ./examples/console.cpp -I. -L. -l:../target/release/libcfastlogging.so
