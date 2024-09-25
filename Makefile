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
	mkdir ./data/tv
	mkdir ./data/movies
	TV_DIR=./data/tv MOVIE_DIR=./data/movies cargo watch -x run

clean:
	@echo "Cleaning up build artifacts..."
	cargo clean
