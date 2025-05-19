
CARGO = cargo


RELEASE_FLAGS = --release


LINUX_TARGET = x86_64-unknown-linux-gnu
WINDOWS_TARGET = x86_64-pc-windows-gnu
MACOS_TARGET = x86_64-apple-darwin


WAYLAND_DISABLE = env WAYLAND_DISPLAY=

.PHONY: all build debug release run clean install uninstall \
        build-linux build-windows build-macos

all: build


build:
	@$(CARGO) build


run:
	@$(WAYLAND_DISABLE) $(CARGO) run

release:
	@$(CARGO) build $(RELEASE_FLAGS)


run-release: release
	@$(WAYLAND_DISABLE) $(CARGO) run $(RELEASE_FLAGS)


clean:
	@$(CARGO) clean

install: release
	@echo "Installing to /usr/local/bin"
	@install -Dm755 target/release/rust-searcher /usr/local/bin/rust-searcher

uninstall:
	@rm -f /usr/local/bin/rust-searcher

build-linux:
	@$(CARGO) build $(RELEASE_FLAGS) --target=$(LINUX_TARGET)

build-windows:
	@$(CARGO) build $(RELEASE_FLAGS) --target=$(WINDOWS_TARGET)

build-macos:
	@$(CARGO) build $(RELEASE_FLAGS) --target=$(MACOS_TARGET)


build-all: build-linux build-windows build-macos
