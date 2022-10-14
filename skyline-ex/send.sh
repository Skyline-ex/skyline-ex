# rustup run skyline-v3 cargo build --release --target ./aarch64-skyline-switch.json -Z build-std=core,alloc,std,panic_abort                                    
# elf2nso ./target/aarch64-skyline-switch/release/libskyline_ex.so ./target/aarch64-skyline-switch/release/libskyline_ex.nso
cargo skyline build --release --nso --features="tcp-logger","sd-logger"
# cargo +skyline-v3 build --release --target ~/.cargo/skyline/aarch64-skyline-switch.json -Z build-std=core,alloc,std,panic_abort
# linkle nso ./target/aarch64-skyline-switch/release/libskyline_ex.so ./target/aarch64-skyline-switch/release/libskyline_ex.nso
curl -T ./target/aarch64-skyline-switch/release/libskyline_ex.nso ftp://192.168.0.192:5000/atmosphere/contents/01006a800016e000/exefs/subsdk9

npdmtool ../exlaunch/misc/npdm-json/application.json ./main.npdm
curl -T ./main.npdm ftp://192.168.0.192:5000/atmosphere/contents/01006a800016e000/exefs/main.npdm