run: 
    cargo run

build:
    cargo build

test:
    cargo test -- --show-output

watch-test:
    cargo watch -x 'test -- --show-output'
