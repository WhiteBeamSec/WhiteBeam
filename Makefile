# Requirements
ifeq ($(shell which cc),)
$(error "cc not found in PATH, consider running: apt update && apt install -y build-essential")
endif
ifeq ($(shell which rustup),)
$(error "rustup not found in PATH, consider running: wget -q --https-only --secure-protocol=TLSv1_2 https://sh.rustup.rs -O - | sh /dev/stdin -y && source $$HOME/.cargo/env")
endif
ifeq ($(shell which pkg-config),)
$(error "pkg-config not found in PATH, consider running: apt update && apt install -y pkg-config libssl-dev")
endif
ifeq ($(shell rustup show|grep stable),)
$(error "No stable Rust found in toolchain, consider running: rustup toolchain install stable")
endif
ifeq ($(shell rustup show|grep nightly),)
$(error "No nightly Rust found in toolchain, consider running: rustup toolchain install nightly")
endif
ifneq ($(shell which sudo),)
sudo_pfx = sudo
endif

all:	library binary

library:
	@echo "Building library"
	RUSTFLAGS='-C link-arg=-s' cargo +nightly build --package libwhitebeam --lib --release
	@echo "Completed. Size:"
	@du -h $(shell pwd)/target/release/libwhitebeam.so | cut -f1

binary:
	@echo "Building binary"
	RUSTFLAGS='-C link-arg=-s' cargo +stable build --package whitebeam --bin whitebeam --release
	@echo "Completed. Size:"
	@du -h $(shell pwd)/target/release/whitebeam | cut -f1

install:
	@echo "Installing"
	@$(sudo_pfx) mkdir -p /opt/WhiteBeam/
	@$(sudo_pfx) cp $(shell pwd)/target/release/whitebeam /opt/WhiteBeam/whitebeam
	@$(sudo_pfx) cp $(shell pwd)/target/release/libwhitebeam.so /opt/WhiteBeam/libwhitebeam.so
	@$(sudo_pfx) mkdir /opt/WhiteBeam/data/
	@$(sudo_pfx) ln -s /opt/WhiteBeam/whitebeam /usr/local/bin/whitebeam
	@$(sudo_pfx) cp $(shell pwd)/src/extra/init.sh /etc/init.d/whitebeam
	@$(sudo_pfx) ln -s /etc/init.d/whitebeam /etc/rc3.d/S01whitebeam
	@$(sudo_pfx) /etc/init.d/whitebeam start
	@echo "/opt/WhiteBeam/libwhitebeam.so" | $(sudo_pfx) tee -a /etc/ld.so.preload
	@echo "Complete"

# TODO: Move into cargo test
test:
	@echo "Building test library"
	RUSTFLAGS='-C link-arg=-s' cargo +nightly build --package libwhitebeam --manifest-path $(shell pwd)/src/library/Cargo.toml --lib --release --features="whitelist_test"
	@echo "Completed. Size:"
	@du -h $(shell pwd)/target/release/libwhitebeam.so | cut -f1
	@echo "libwhitebeam.so:"
	@echo "\033[4mProperties\033[0m:"
	@objdump -T -j .text $(shell pwd)/target/release/libwhitebeam.so | grep -v rust_eh_personality
	@file -b $(shell pwd)/target/release/libwhitebeam.so
	@echo "\033[4mTesting\033[0m:"
	@echo "Whitelisted binary"
	LD_PRELOAD=$(shell pwd)/target/release/libwhitebeam.so /bin/bash -c 'if /usr/bin/whoami; then echo Success; else echo Fail; fi' || true
	@echo "Non-whitelisted binary"
	LD_PRELOAD=$(shell pwd)/target/release/libwhitebeam.so /bin/bash -c 'if ! /usr/bin/id; then echo Success; else echo Fail; fi' || true
	@echo
	@echo "whitebeam:"
	@echo "\033[4mTesting\033[0m:"
	@$(shell pwd)/target/release/whitebeam || true

clean:
	@echo "Cleaning up"
	cargo clean
	rm Cargo.lock
