cargo clean
cargo build --release
cd target/release
tar caf jinx.tar.xz jinx ../../.jinx-templates
7z a jinx.7z jinx ../../.jinx-templates
cd ../../