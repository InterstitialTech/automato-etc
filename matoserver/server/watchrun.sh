TIMECLONK_STATIC_PATH=static RUST_LOG=info  cargo watch -x build -s ./run.sh -c -w src -w Cargo.toml -w ../../rustlib/src/ -w ../../rustlib/Cargo.toml

