cargo clean
cargo build --release
cd target/release
tar caf jinx.tar.xz jinx ../../templates
7z a jinx.7z jinx ../../templates
cd ../../