CARGO = cargo
RUSTC = rustc
CROSS = CROSS_REMOTE=1 cross

all: build

build:
	trunk build

dev:
	trunk serve

release:
	trunk build --release

fix:
	leptosfmt . \
		&& cargo fix --allow-dirty --all-features \
		&& cargo fmt

clean:
	cargo clean && rm -rf dist
