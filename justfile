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

push:
    git push
    just push-cc

push-cc:
    # They still use `master` for some unknown reason
    git push codecrafters main:master
