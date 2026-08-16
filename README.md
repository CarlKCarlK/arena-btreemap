# arena-btreemap

A [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) that supports custom allocators on **stable Rust**, ported directly from the standard library.

[![crates.io](https://img.shields.io/crates/v/arena-btreemap.svg)](https://crates.io/crates/arena-btreemap)

## Why?

The standard library's `BTreeMap` supports custom allocators, but only on nightly Rust (behind `#![feature(allocator_api)]` and `#![feature(btreemap_alloc)]`). This crate ports the **exact same implementation** to stable Rust using [`allocator-api2`](https://crates.io/crates/allocator-api2).

**No algorithm changes.** Every line of code is ported from `library/alloc/src/collections/btree/` in the Rust source tree. The only modifications are:

- Import paths swapped from `std::alloc` → `allocator_api2::alloc`
- Nightly-only attributes (`#[unstable]`, `#[stable]`, `#[rustc_diagnostic_item]`, `#[may_dangle]`) stripped
- `TrustedLen` impls removed (unstable trait)
- `extend_one` methods removed (unstable)
- `write_length_prefix` → stable equivalent
- `slice_ptr_get` → manual pointer arithmetic

## Usage

```rust
use arena_btreemap::BTreeMap;

// Works just like std::collections::BTreeMap:
let mut map = BTreeMap::new();
map.insert("hello", "world");
assert_eq!(map.get(&"hello"), Some(&"world"));
```

With a custom arena allocator (e.g. [`bumpalo`](https://crates.io/crates/bumpalo)):

```rust,ignore
use arena_btreemap::BTreeMap;
use bumpalo::Bump;

let bump = Bump::new();
let mut map = BTreeMap::<&str, &str, &Bump>::new_in(&bump);
map.insert("hello", "world");
// When `map` drops, the arena's bump pointer resets — no per-node deallocation.
// When `bump` drops, all memory is freed in one operation.
```

## Drop-in Replacement

The API is identical to `std::collections::BTreeMap`. Switching is a type alias change: `std::collections::BTreeMap<K, V>` → `arena_btreemap::BTreeMap<K, V>`. Construction sites change from `BTreeMap::new()` to `BTreeMap::new_in(alloc)` when using a custom allocator. All read sites (`.iter()`, `.get()`, `.keys()`) work unchanged.

## Features

- `std` (default): Enables `std`-dependent functionality. Disable for `no_std` targets.
- `no_std`: Enables `alloc`-only mode. Works on `wasm32-unknown-unknown` and other constrained targets.

## `no_std` and WASM

This crate compiles on `wasm32-unknown-unknown` with `--no-default-features`:

```sh
cargo check --target wasm32-unknown-unknown --no-default-features
```

## Testing

132 tests verify behavioral parity with `std::collections::BTreeMap`:

- 28 unit tests (node structure, search, iteration)
- 9 arena integration tests (custom allocator verification)
- 12 stress tests (10K elements, random ops, repeated cycles)
- 12 std compatibility tests (direct comparison with std)
- 71 doctests

```sh
cargo test                    # debug mode
cargo test --release          # release mode
cargo clippy --all-targets    # 0 warnings
```

## License

MIT OR Apache-2.0 — same as the Rust standard library source from which this crate is ported.
