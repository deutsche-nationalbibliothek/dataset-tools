.POSIX:

all:

all: target/release/dataset target/release/datashed

clean:
	cargo clean --workspace --release
.PHONY: clean
	
target/release/datashed:
	cargo build --all-features --release -p datashed

target/release/dataset:
	cargo build --all-features --release -p dataset-cli

install: target/release/dataset target/release/datashed
	install -Dm755 target/release/datashed /usr/local/bin/datashed
	install -Dm755 target/release/dataset /usr/local/bin/dataset

	
