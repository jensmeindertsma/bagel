help: 
  just --list

build: 
  cargo build

check:
  cargo clippy

clean: 
  cargo clean

run *ARGS: 
  cargo run {{ARGS}}

test: 
  cargo nextest run --release

test-cc: 
  codecrafters test

# Push to both remotes
push:
  git push
  just push-cc

# They still use `master` for some unknown reason
push-cc:
  git push codecrafters main:master
