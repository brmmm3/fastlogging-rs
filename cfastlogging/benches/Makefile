# Makefile for benchmark — compares cfastlogging with zlog
#
# Usage:
#   make zlog        — build zlog from source (in zlog_src/)
#   make build       — build the benchmark binary
#   make run         — build and run the benchmark
#   make clean       — clean build artifacts
#
# On Windows, this Makefile requires MSYS2 MinGW64 (gcc + make).
# The cfastlogging DLL must already be built (cargo build --release).

GCC_BIN ?= gcc

# cfastlogging paths — assumes cargo build --release was run
# Override CFL_TARGET_DIR if your target directory is elsewhere, e.g.:
#   make build CFL_TARGET_DIR=/path/to/target/release
CFL_TARGET_DIR ?= ../../target/release
CFL_INCLUDE = ..

# zlog paths
ZLOG_SRC_DIR = ../zlog_src
ZLOG_INCLUDE = $(ZLOG_SRC_DIR)/src
ZLOG_LIB_DIR = $(ZLOG_SRC_DIR)/src

# Compiler flags
CFLAGS = -O2 -Wall -std=c11
INC = -I$(CFL_INCLUDE) -I$(ZLOG_INCLUDE)

# Detect platform
UNAME := $(shell uname 2>/dev/null || echo Windows)
ifeq ($(UNAME),Linux)
    CFL_LINK = -L$(CFL_TARGET_DIR) -lcfastlogging
    ZLOG_LINK = -L$(ZLOG_LIB_DIR) -lzlog
    WSA_LIB =
else
    # Windows / MinGW
    CFL_LINK = -L$(CFL_TARGET_DIR) -l:cfastlogging.dll.lib
    ZLOG_LINK = -L$(ZLOG_LIB_DIR) -lzlog
    WSA_LIB = -lws2_32
endif

LIBS = $(CFL_LINK) $(ZLOG_LINK) $(WSA_LIB)

# Output
BIN = bin
BENCHMARK = $(BIN)/benchmark

.PHONY: all build run zlog clean

all: build

# Build zlog from source
zlog:
	@echo "=== Building zlog from source ==="
	@if [ ! -d "$(ZLOG_SRC_DIR)" ]; then \
		echo "Cloning zlog..."; \
		git clone --depth 1 https://github.com/HardySimpson/zlog.git $(ZLOG_SRC_DIR); \
	fi
	cd $(ZLOG_SRC_DIR)/src && $(MAKE) clean
	cd $(ZLOG_SRC_DIR)/src && $(MAKE) UNAME=MINGW
	@echo "=== zlog built ==="

# Build the benchmark
build:
	@mkdir -p $(BIN)
	@mkdir -p ../doc/benchmarks
	$(GCC_BIN) $(CFLAGS) -o $(BENCHMARK) ../examples/benchmark.c $(INC) $(LIBS)

# Run the benchmark
run: build
	./$(BENCHMARK)

clean:
	rm -rf $(BIN)
	rm -f ../doc/benchmarks/*.json ../doc/benchmarks/*.html
