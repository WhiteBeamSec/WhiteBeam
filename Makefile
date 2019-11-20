# Requirements
ifeq ($(shell which cc),)
$(error "cc not found in PATH, consider running: apt update && apt install -y build-essential")
endif
ifeq ($(shell which rustup),)
$(error "rustup not found in PATH, consider running: wget -q --https-only --secure-protocol=TLSv1_2 https://sh.rustup.rs -O - | sh /dev/stdin -y && source $$HOME/.cargo/env")
endif
ifeq ($(shell rustup show|grep stable),)
$(error "No stable Rust found in toolchain, consider running: rustup toolchain install stable")
endif
ifeq ($(shell rustup show|grep nightly),)
$(error "No nightly Rust found in toolchain, consider running: rustup toolchain install nightly")
endif

all:	library binary

library:
	@echo "Building library"
	cargo +nightly build --lib --release --features=libraries
	strip $(shell pwd)/target/release/libwhitebeam.so
	@echo "Completed. Size:"
	@du -h $(shell pwd)/target/release/libwhitebeam.so | cut -f1

binary:
	@echo "Building binary"
	cargo build --bin whitebeam --release --features=binaries
	strip $(shell pwd)/target/release/whitebeam
	@echo "Completed. Size:"
	@du -h $(shell pwd)/target/release/whitebeam | cut -f1

install:
	@echo "Installing"
	@sudo mkdir -p /opt/WhiteBeam/
	@sudo cp $(shell pwd)/target/release/whitebeam /opt/WhiteBeam/whitebeam
	@sudo cp $(shell pwd)/target/release/libwhitebeam.so /opt/WhiteBeam/libwhitebeam.so
	@sudo mkdir /opt/WhiteBeam/data/
	@sudo ln -s /opt/WhiteBeam/whitebeam /usr/local/bin/whitebeam
	@sudo cp $(shell pwd)/src/extra/init.sh /etc/init.d/whitebeam
	@sudo ln -s /etc/init.d/whitebeam /etc/rc3.d/S01whitebeam
	@echo "/opt/WhiteBeam/libwhitebeam.so" | sudo tee -a /etc/ld.so.preload
	@sudo /opt/WhiteBeam/whitebeam --start
	@echo "Complete"

test:
	@echo "Building test library"
	cargo build --lib --release --features="libraries,whitelist_test"
	strip $(shell pwd)/target/release/libwhitebeam.so
	@echo "Completed. Size:"
	@du -h $(shell pwd)/target/release/libwhitebeam.so | cut -f1
	@echo "libwhitebeam.so:"
	@echo "\033[4mProperties\033[0m:"
	@objdump -T -j .text $(shell pwd)/target/release/libwhitebeam.so
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
