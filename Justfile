run:
    cd better_explosions/viewer && cargo run
run_rel:
    cd better_explosions/viewer && cargo run --release
build:
    cargo build --target=i686-pc-windows-gnu
build_rel:
    cargo build --release --target=i686-pc-windows-gnu
test:
    cargo test --features test -- --test-threads=1 --nocapture
test_rel:
    cargo test --features test -- --test-threads=1 --nocapture