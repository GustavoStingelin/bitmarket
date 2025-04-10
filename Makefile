.PHONY: wasm test

wasm:
	cd crypto-core/wallet-core && wasm-pack build --target web --out-dir ../../pkg/wallet-core
	cd crypto-core/dlc-core && wasm-pack build --target web --out-dir ../../pkg/dlc-core
	cd crypto-core/musig-core && wasm-pack build --target web --out-dir ../../pkg/musig-core

test:
	cd crypto-core && cargo test --all 