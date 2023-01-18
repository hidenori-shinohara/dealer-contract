build:
	cargo build --target wasm32-unknown-unknown --release
run:
	soroban invoke \
    --wasm target/wasm32-unknown-unknown/release/dealer.wasm \
    --id 1 \
    --fn deal

