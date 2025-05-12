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
  cargo run --quiet --package {{binary}} --bin {{binary}} {{arguments}}

test:
  cargo build --release --workspace --bins
  cargo nextest run --release
