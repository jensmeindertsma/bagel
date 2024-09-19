help: 
  just --list

build: 
  cargo build

clean: 
  cargo clean

run *ARGS: 
  cargo run {{ARGS}}

test: 
  cargo nextest run

test-cc: 
  codecrafters test

push:
  # Push to both remotes
  git push
  just push-cc

push-cc:
  # They still use `master` for some unknown reason
  git push codecrafters main:master
