all:	library binary

library:
	@echo "Building library"
	cargo build --lib --release --features=libraries
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
	@mkdir -p /opt/WhiteBeam/
	@cp $(shell pwd)/target/release/whitebeam /opt/WhiteBeam/whitebeam
	@cp $(shell pwd)/target/release/libwhitebeam.so /opt/WhiteBeam/libwhitebeam.so
	@mkdir /opt/WhiteBeam/data/
	@ln -s /opt/WhiteBeam/whitebeam /usr/local/bin/whitebeam
	@echo "/opt/WhiteBeam/libwhitebeam.so" > /etc/ld.so.preload

test:
	@echo "libwhitebeam.so:"
	@echo "\033[4mProperties\033[0m:"
	@objdump -T -j .text $(shell pwd)/target/release/libwhitebeam.so
	@file -b $(shell pwd)/target/release/libwhitebeam.so
	@echo "\033[4mTesting\033[0m:"
	LD_PRELOAD=$(shell pwd)/target/release/libwhitebeam.so /bin/bash -c "whoami" || true
	@echo
	@echo "whitebeam:"
	@echo "\033[4mTesting\033[0m:"
	@$(shell pwd)/target/release/whitebeam || true

clean:
	@echo "Cleaning up"
	cargo clean
	rm Cargo.lock
