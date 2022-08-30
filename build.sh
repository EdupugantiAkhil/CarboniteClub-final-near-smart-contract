RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/carbonite_nt_nft.wasm ./res/carbonite_nt_nft.wasm
