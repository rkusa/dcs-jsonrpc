build:
	cargo build

# build must run before test so that the dynamic library is created
test: build
	cargo test

test_old: build
	lua5.1 test.lua

example:
	cd example && cargo run

repl:
	cd repl && cargo run

.PHONY: example repl