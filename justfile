help:
    just --list

build:
    cargo build

run *ARGS:
    cargo run {{ARGS}}

push:
    git push
    just push-cc

push-cc:
    # They still use `master` for some unknown reason
    git push codecrafters main:master
