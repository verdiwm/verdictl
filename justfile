PREFIX := "usr"
BINARY := PREFIX / "bin"
DESTDIR := "/"


build:
    cargo build --release

install:
    install -Dm755 target/release/verdictl {{ DESTDIR }}{{ BINARY }}/verdictl
