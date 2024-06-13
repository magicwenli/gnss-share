win_msvc := 'x86_64-pc-windows-msvc'
linux_aarch64 := 'aarch64-unknown-linux-gnu'

build:
    cargo build

cross_linux:
    cross build --target {{linux_aarch64}} --release

cross_win:
    cross build --target {{win_msvc}} --release