# atc-router-wasm

WebAssembly bindings for Kong's [ATC Router library](https://github.com/Kong/atc-router)

## Build

1. Install wasm-pack via the [quick start](https://rustwasm.github.io/wasm-pack/book/quickstart.html) guide or ...
   ```shell
   make setup
   ```
2. Then, build the WASM package:

   ```shell
   make build
   ```

## Example

1. Build the WASM package
2. Install [node](https://nodejs.org/en/download) and [pnpm](https://pnpm.io/installation)
3. Install dependencies and start a dev server
   ```
   cd example
   pnpm i
   pnpm dev
   ```
