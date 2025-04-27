CARGO = cargo
RUSTC = rustc
CROSS = CROSS_REMOTE=1 cross

all: build

build:
	trunk build

dev:
	trunk serve
