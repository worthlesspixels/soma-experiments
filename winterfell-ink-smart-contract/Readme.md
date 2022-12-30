Concerns with the integration:

1. The wasm blob that a contract is compiled down to cannot be larger than 128KB and this contract, even with optimizations applied, generates a 300KB WASM blob. Most of this is due to the `verify` function. 
