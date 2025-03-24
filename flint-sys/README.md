# flint-sys

Rust bindings to the [FLINT](http://flintlib.org/) library. 

FLINT (Fast Library for Number Theory) is a C library which provides a number
of number theoretic and algebraic functions and types.

Also see [`gmp-mpfr-sys`](https://crates.io/crates/gmp-mpfr-sys) for GMP, MPFR, and MPC bindings.

## Usage

See the [documentation](https://docs.rs/flint-sys/latest/flint_sys/). This crate is available on [crates.io](https://crates.io/crates/flint-sys).

## Optional features

  * `disable-make-check`: this can reduce compilation time significantly. Enabled by default.

## Caching
Built libraries are cached in the user’s cache directory as follows:

  * GNU/Linux: inside $XDG_CACHE_HOME/flint-sys or $HOME/.cache/flint-sys

  * macOS: inside $HOME/Library/Caches/flint-sys

## Notes

  * Windows is not currently supported.

  * WSL users may need to install clang.
 
  * As of version 0.6.0 the FLINT source files are now included and the library is compiled automatically. The files are cached to avoid unnecessary compilations.
