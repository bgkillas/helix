run r="" n="":
    cd better_explosions/viewer && cargo run -- {{r}} {{n}}
run_rel r="" n="":
    cd better_explosions/viewer && cargo run --release -- {{r}} {{n}}
build:
    cargo build --target=i686-pc-windows-gnu
    ln -srf ./target/i686-pc-windows-gnu/debug/helix.dll ./mod/
    ln -srf ./target/i686-pc-windows-gnu/debug/better_explosions.dll ./better_explosions/mod/
build_rel:
    cargo build --release --target=i686-pc-windows-gnu
    ln -srf ./target/i686-pc-windows-gnu/release/helix.dll ./mod/
    ln -srf ./target/i686-pc-windows-gnu/release/better_explosions.dll ./better_explosions/mod/
miri:
    cargo miri test -- --test-threads=1 --nocapture
test:
    cargo test --features test -- --test-threads=1 --nocapture
test_rel:
    cargo test --release --features test -- --test-threads=1 --nocapture
bench:
    cargo bench --quiet -- --color always --test-threads=1 --nocapture
clippy:
    cd better_explosions/viewer && cargo fmt
    cd better_explosions/viewer && cargo clippy
    cargo fmt
    cargo clippy
