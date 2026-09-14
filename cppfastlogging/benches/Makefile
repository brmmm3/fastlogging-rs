# Makefile for benchmark — compares cppfastlogging, cxxfastlogging and loguru
#
# Usage:
#   make build       — build the benchmark binary
#   make run          — build and run the benchmark
#   make clean        — clean build artifacts
#
# Prerequisites:
#   - cargo build --release  (builds both cppfastlogging and cxxfastlogging)
#   - MSYS2 MinGW64 g++ (on Windows)
#   - loguru is included as a header-only library in loguru_src/

GXX_BIN ?= g++

# cfastlogging / cppfastlogging paths — assumes cargo build --release was run
CFL_TARGET_DIR ?= ../../target/release
CPP_INCLUDE = ..

# cxxfastlogging paths
# The cxxbridge generated header is at:
#   $(CFL_TARGET_DIR)/build/cxxfastlogging/*/out/cxxbridge/include/cxxfastlogging/src/lib.rs.h
# We use wildcard to find it.
CXXBRIDGE_INCLUDE := $(dir $(wildcard $(CFL_TARGET_DIR)/build/cxxfastlogging/*/out/cxxbridge/include/cxxfastlogging/src/lib.rs.h))

# loguru paths
LOGURU_INCLUDE = ../loguru_src

# Compiler flags
CXXFLAGS = -O2 -Wall -std=c++17
INC = -I$(CPP_INCLUDE) -I$(CXXBRIDGE_INCLUDE) -I$(LOGURU_INCLUDE)

# Detect platform
UNAME := $(shell uname 2>/dev/null || echo Windows)
ifeq ($(UNAME),Linux)
    CFL_LINK = -L$(CFL_TARGET_DIR) -lcfastlogging
    CXX_LINK = -L$(CFL_TARGET_DIR) -lcxxfastlogging
    WSA_LIB =
    THREAD_LIB = -lpthread
    SYS_LIBS =
else
    # Windows / MinGW
    CFL_LINK = -L$(CFL_TARGET_DIR) -l:cfastlogging.dll.lib
    CXX_LINK = -L$(CFL_TARGET_DIR) -lcxxfastlogging
    WSA_LIB = -lws2_32
    THREAD_LIB = -lpthread
    # libcxxfastlogging.a (Rust static lib) needs these Windows system libs
    SYS_LIBS = -lntdll -luserenv -ladvapi32
endif

LIBS = $(CFL_LINK) $(CXX_LINK) $(WSA_LIB) $(THREAD_LIB) $(SYS_LIBS)

# Output
BIN = bin
BENCHMARK = $(BIN)/benchmark

.PHONY: all build run clean

all: build

# Build the benchmark
build:
	@mkdir -p $(BIN)
	@mkdir -p ../doc/benchmarks
	$(GXX_BIN) $(CXXFLAGS) -o $(BENCHMARK) ../examples/benchmark.cpp ../examples/benchmark_cxx.cpp $(INC) $(LIBS)

# Run the benchmark
run: build
	./$(BENCHMARK)

clean:
	rm -rf $(BIN)
	rm -f ../doc/benchmarks/*.json ../doc/benchmarks/*.html
