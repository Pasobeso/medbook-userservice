BIN_NAME=medbook-userservice

all: build run

build:
	cargo build

run: build
	./target/debug/$(BIN_NAME)