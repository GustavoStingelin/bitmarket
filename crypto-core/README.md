# Crypto Core

Rust → WASM crypto core for a browser-only Bitcoin/Nostr "Oracle Marketplace".

## Components

This workspace contains the following crates:

- **wallet-core**: Taproot descriptor wallet implementation using Bitcoin Dev Kit (BDK)
- **dlc-core**: Discreet Log Contract (DLC) builder and verifier using rust-dlc
- **musig-core**: MuSig2 aggregation utilities for multi-signature support

## Building for WASM

Each crate is configured to compile to WebAssembly with `wasm-bindgen` for browser compatibility.

To build all crates:

```bash
cd crypto-core
wasm-pack build
```

To build individual crates:

```bash
cd crypto-core/wallet-core
wasm-pack build
```

## Usage

The compiled WASM modules can be imported in JavaScript/TypeScript to enable Bitcoin and DLC functionality directly in the browser. 