all:
	@echo "Building library"
	cargo build --lib --release
	@echo "Building binary"
	cargo build --bin whitebeam --release
	@echo "Complete"

install:
	@echo "Installing"
	@mkdir -p /opt/whitebeam/
	@cp $(shell pwd)/target/release/whitebeam /opt/whitebeam/whitebeam
	@cp $(shell pwd)/target/release/libwhitebeam.so /opt/whitebeam/libwhitebeam.so
	@mkdir /opt/whitebeam/data/
	@ln -s /opt/whitebeam/whitebeam /usr/local/bin/whitebeam
	@echo "/opt/whitebeam/libwhitebeam.so" > /etc/ld.so.preload

test:
	@echo "libwhitebeam.so:"
	@echo "\033[4mProperties\033[0m:"
	@nm -g $(shell pwd)/target/release/libwhitebeam.so | grep 'execve'
	@file -b $(shell pwd)/target/release/libwhitebeam.so
	@echo "\033[4mTesting\033[0m:"
	LD_PRELOAD=$(shell pwd)/target/release/libwhitebeam.so /bin/bash -c "whoami"
	@echo
	@echo "whitebeam:"
	@echo "\033[4mTesting\033[0m:"
	@$(shell pwd)/target/release/whitebeam

clean:
	@echo "Cleaning up"
	cargo clean
	rm Cargo.lock
