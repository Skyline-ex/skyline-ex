# rustup run skyline-v3 cargo build --release --target ./aarch64-skyline-switch.json -Z build-std=core,alloc,std,panic_abort                                    
# elf2nso ./target/aarch64-skyline-switch/release/libskyline_ex.so ./target/aarch64-skyline-switch/release/libskyline_ex.nso
cargo skyline build --release --nso
# cargo +skyline-v3 build --release --target ~/.cargo/skyline/aarch64-skyline-switch.json -Z build-std=core,alloc,std,panic_abort
# linkle nso ./target/aarch64-skyline-switch/release/libskyline_ex.so ./target/aarch64-skyline-switch/release/libskyline_ex.nso
curl -T ./target/aarch64-skyline-switch/release/libskyline_ex.nso ftp://192.168.0.109:5000/atmosphere/contents/01006a800016e000/exefs/subsdk9