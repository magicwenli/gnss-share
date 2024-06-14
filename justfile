win_msvc := 'x86_64-pc-windows-msvc'
linux_aarch64 := 'aarch64-unknown-linux-gnu'

build:
    cargo build

cross_linux:
    cross build --target {{linux_aarch64}} --release

cross_win:
    cross build --target {{win_msvc}} --release

bump:
    just ensure_installed upgrade
    cargo update
    cargo upgrade --incompatible

lint:
    just ensure_installed sort
    cargo fmt --all -- --check
    cargo sort --workspace --check
    cargo clippy --tests --workspace -- -D warnings

format:
    just ensure_installed sort
    cargo fmt
    cargo sort --workspace

ensure_installed *args:
    #!/bin/bash
    cargo --list | grep -q {{ args }}
    if [[ $? -ne 0 ]]; then
        echo "error: cargo-{{ args }} is not installed"
        exit 1
    fi