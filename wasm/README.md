# ProveIt WASM Build Configuration

This directory contains configuration for building ProveIt for WebAssembly (WASM).

## Building for WASM

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### Build

```bash
# Build for web
wasm-pack build --target web crates/proveit-core

# Build for Node.js
wasm-pack build --target nodejs crates/proveit-core

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler crates/proveit-core
```

## Features in WASM Build

- ✅ Core type checking
- ✅ Spatial graph engine
- ✅ Formal methods (TTT, SCTT, homotopy type theory)
- ⚠️ GPU acceleration (via WebGPU when available)
- ⚠️ Spatial audio (via Web Audio API)

## Usage Example

```javascript
import init, { ProveItCore } from './pkg/proveit_core.js';

async function run() {
    await init();
    const core = new ProveItCore();
    // Use ProveIt functionality
}

run();
```
