[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/GHvW/clay)

# clay

A Shapefile reader.

Primary intent is to use as a WebAssembly module

### State of Development
Before starting this project I did not fully understand the limitations of compiling to the WASM target.
I've had to go through a number of refactors to be WASM ready and with each refactor comes a new WASM target issue.
I've decided to wait until more support is added out of the box for rust types and features, or until I've been able
to do a little more up front research about what is and isn't supported.

For example:
- [u8 slices](https://github.com/rustwasm/wasm-bindgen/issues/5)
- [lifetimes](https://github.com/rustwasm/wasm-bindgen/issues/423)
- etc

Rust's typesystem make it easy to successfully refactor, but that doesn't mean it's quick or fun 😆.

As of this writing, types in `clay` can successfully read a simple polygon shapefile. 
I have not tested it with some of the more complex polygons that can exist in a shapefile
