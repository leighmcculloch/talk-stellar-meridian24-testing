build:
	# cd auth && stellar contract build
	cd token && stellar contract build

test: build
	# cd auth && cargo test
	cd token && cargo test
