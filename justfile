set quiet

help:
  just --list

clean:
  cargo clean

check:
  cargo clippy --workspace --all-targets

format:
  cargo fmt --all

format-ci:
  cargo fmt --all --check

push:
  git push
  git push codecrafters main:master

run binary *arguments:
  cargo run --quiet --bin {{binary}} {{arguments}} || true

test *arguments:
  cargo build --release --workspace --all-targets
  cargo nextest run --release {{arguments}}
