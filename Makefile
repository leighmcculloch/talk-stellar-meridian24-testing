build:
	cd token && stellar contract build

test: build
	cd token && cargo test
