# Основные команды
.PHONY: all build release run clean install uninstall

all: build

build:
	@cargo build

release:
	@cargo build --release

run:
	@cargo run

clean:
	@cargo clean

build-linux:
	@cargo build --release --target=x86_64-unknown-linux-gnu

build-windows:
	@cargo build --release --target=x86_64-pc-windows-gnu