GXX_BIN ?= $(shell which g++)
CARGO_BIN ?= $(shell which cargo)

CXXBRIDGE_INCLUDE_DEBUG := $(shell find ../target/debug/build -maxdepth 4 -type d -path '*/cxxfastlogging-*/out/cxxbridge/include' 2>/dev/null | head -n1)
CXXBRIDGE_INCLUDE_RELEASE := $(shell find ../target/release/build -maxdepth 4 -type d -path '*/cxxfastlogging-*/out/cxxbridge/include' 2>/dev/null | head -n1)

run: build-debug
	./bin/console

clean:
	rm -rf bin

build:
	$(CARGO_BIN) build --release
	@mkdir -p bin
	$(GXX_BIN) -std=c++17 -o ./bin/console ./examples/console.cpp -I.. -I$(CXXBRIDGE_INCLUDE_RELEASE) -L../target/release -l:libcxxfastlogging.a -lpthread -ldl -lm

build-debug:
	$(CARGO_BIN) build
	@mkdir -p bin
	$(GXX_BIN) -std=c++17 -o ./bin/console ./examples/console.cpp -I.. -I$(CXXBRIDGE_INCLUDE_DEBUG) -L../target/debug -l:libcxxfastlogging.a -lpthread -ldl -lm
