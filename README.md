# arena-btreemap

[![crates.io](https://img.shields.io/crates/v/arena-btreemap.svg)](https://crates.io/crates/arena-btreemap)
[![documentation](https://docs.rs/arena-btreemap/badge.svg)](https://docs.rs/arena-btreemap)
[![license](https://img.shields.io/crates/l/arena-btreemap.svg)](https://github.com/guan-tends/arena-btreemap#license)
[![tests](https://img.shields.io/badge/tests-132%20passing-brightgreen.svg)](https://github.com/guan-tends/arena-btreemap)
[![clippy](https://img.shields.io/badge/clippy-0%20warnings-brightgreen.svg)](https://github.com/guan-tends/arena-btreemap)

A [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) that supports custom allocators on **stable Rust**, ported directly from the standard library.

## Motivation

The standard library's `BTreeMap` supports custom allocators, but only on nightly Rust (behind `#![feature(allocator_api)]` and `#![feature(btreemap_alloc)]`). This crate ports the **exact same implementation** to stable Rust using [`allocator-api2`](https://crates.io/crates/allocator-api2).

**No algorithm changes.** Every line of code is ported from `library/alloc/src/collections/btree/` in the Rust source tree.

## Installation

```toml
[dependencies]
arena-btreemap = "0.1"
```

For `no_std` environments:

```toml
[dependencies]
arena-btreemap = { version = "0.1", default-features = false }
```

## Usage

### Default allocator (drop-in std replacement)

```rust
use arena_btreemap::BTreeMap;

let mut map = BTreeMap::new();
map.insert("hello", "world");
assert_eq!(map.get(&"hello"), Some(&"world"));
```

### Custom arena allocator

With a bump allocator like [`bumpalo`](https://crates.io/crates/bumpalo), dropping the map is O(1) — no per-node deallocation:

```rust,ignore
use arena_btreemap::BTreeMap;
use bumpalo::Bump;

let bump = Bump::new();
let mut map = BTreeMap::<&str, &str, &Bump>::new_in(&bump);
map.insert("hello", "world");

// map drops — no per-node deallocation, just a bump pointer reset
// bump drops — all memory freed in one operation
```

### API parity with std

The API is identical to `std::collections::BTreeMap`. All read sites (`.iter()`, `.get()`, `.keys()`, `.range()`) work unchanged. Construction sites change from `BTreeMap::new()` to `BTreeMap::new_in(alloc)` when using a custom allocator.

## What changed from std

Only what was necessary to compile on stable:

- Import paths swapped from `std::alloc` → `allocator_api2::alloc`
- Nightly-only attributes (`#[unstable]`, `#[stable]`, `#[rustc_diagnostic_item]`, `#[may_dangle]`) stripped
- `TrustedLen` impls removed (unstable trait)
- `extend_one` methods removed (unstable)
- `write_length_prefix` → stable equivalent
- `slice_ptr_get` → manual pointer arithmetic

The B-tree algorithm itself is **byte-for-byte identical** to the std implementation.

## `no_std` and WASM

This crate compiles on `wasm32-unknown-unknown` with `--no-default-features`:

```sh
cargo check --target wasm32-unknown-unknown --no-default-features
```

## Testing

132 tests verify behavioral parity with `std::collections::BTreeMap`:

| Suite | Tests | Description |
|-------|-------|-------------|
| Unit | 28 | Node structure, search, iteration |
| Arena | 9 | Custom allocator verification |
| Stress | 12 | 10K elements, random ops, repeated cycles |
| Std compat | 12 | Direct comparison with std `BTreeMap` |
| Doctests | 71 | std doctests adapted to this crate |

```sh
cargo test                    # debug mode — 132 passed
cargo test --release          # release mode — 132 passed
cargo clippy --all-targets    # 0 warnings
cargo audit                   # 0 vulnerabilities
```

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option. This matches the licensing of the Rust standard library source from which this crate is ported.

## Sponsors

If this crate is useful to you, consider supporting its development:

- **Solana**: `Eu8wQcW68TKMs1a6eqzZu8znzU52QLqQugAMG8uCD6y6`
- **EVM** (Ethereum / Base / Arbitrum / Optimism / Polygon): `0x2733ff7c865C56d565a99BE1DC11B81cc76850A5`
- **XRP Ledger**: `r4X6e7McAQj7e8vBCeued1RYu4mCJrREDG`
