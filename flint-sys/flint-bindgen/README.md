# flint-bindgen

Generate bindings to the FLINT library with bindgen.

Note bindgen requires `clang`.

## To update flint version
Rough summary, needs to be updated/improved.
1. Update flint source code `flint-sys/flint-x.y.z`.
2. Update `flint-sys/build.rs` with version number.
3. Generate bindings by building `flint-bindgen` with `INCLUDE_DIR=/path/to/local/flint/includes cargo build` e.g. `INCLUDE_DIR=$HOME/.local/include cargo build`.
4. Generate `extern.c` using included script and copy to `flint-sys/C/extern.c`.
