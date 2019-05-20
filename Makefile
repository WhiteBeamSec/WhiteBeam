all:
	@echo "Building WhiteBeam"
	cargo build --lib --release

test:
	@echo "Library properties:"
	file $(shell pwd)/target/release/libwhitebeam.so
	nm -g $(shell pwd)/target/release/libwhitebeam.so
	@echo "Testing:"
	LD_PRELOAD=$(shell pwd)/target/release/libwhitebeam.so id

clean:
	@echo "Cleaning up"
	cargo clean
	rm Cargo.lock
