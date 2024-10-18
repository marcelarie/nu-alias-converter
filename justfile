run: 
    cargo run

build:
    cargo build

test:
    cargo test -- --show-output

wtest:
    cargo watch -x 'test -- --show-output'
