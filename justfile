set quiet

help:
  just --list

clean:
  cargo clean

check:
  cargo clippy --all-targets

format:
  cargo fmt

format-ci:
  cargo fmt --check

push:
  git push
  git push codecrafters main:master

run binary *arguments:
  cargo run --quiet --package {{binary}} --bin {{binary}} {{arguments}} || true

test:
  cargo build --release --workspace --all-targets
  cargo nextest run --release
