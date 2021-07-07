

## Building DNAs

Assume all commands are run inside of `nix-shell`

### Compile to WASM

```bash
cd dnas/dnarepo/
RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown --package storage

// or
make target/wasm32-unknown-unknown/release/storage.wasm
```

### Bundle DNA

```bash
hc dna pack bundled/dnas/dnas.dna

// or
make bundled/dnas/dnas.dna
```

## Testing

### Test DNA

```bash
RUST_LOG=[debug]=debug TRYORAMA_LOG_LEVEL=info RUST_BACKTRACE=full TRYORAMA_HOLOCHAIN_PATH="holochain" npx mocha src/test_dnas.js

// or
make test-dnas-debug
```
