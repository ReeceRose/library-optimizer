# Targets
.PHONY: all build dev clean

all: build

build:
	@echo "Building library optimizer"
	cargo build --release

test:
	@echo "Running tests..."
	cargo test

dev:
	@echo "Starting services in development mode..."
	mkdir -p ./data/tv
	mkdir -p ./data/movies
	TV_DIR=./data/tv MOVIE_DIR=./data/movies cargo watch -x run

clean:
	@echo "Cleaning up build artifacts..."
	rm library-optimizer.db*
	cargo clean
