help:
    just --list

push:
    git push
    just push-cc

push-cc:
    # They still use `master` for some unknown reason
    git push codecrafters main:master
