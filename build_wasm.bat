cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./target/wasm32-unknown-unknown/release/out/ --out-name "%1" "./target/wasm32-unknown-unknown/release/%1.wasm"
xcopy .\assets .\target\wasm32-unknown-unknown\release\out\assets /s /i
