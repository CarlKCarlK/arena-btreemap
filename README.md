# arena-btreemap

> [!IMPORTANT]
> This is the `sorted-unique-constructor` branch of an experimental fork of
> [`guan-tends/arena-btreemap`](https://github.com/guan-tends/arena-btreemap).
> It adds unchecked sorted-unique bulk construction and fixes alloc-only target
> support. It is not the version currently published on crates.io. The work follows
> [upstream issue #1](https://github.com/guan-tends/arena-btreemap/issues/1)
> and is exercised by [range-set-blaze PR #34](https://github.com/CarlKCarlK/range-set-blaze/pull/34).

[![crates.io](https://img.shields.io/crates/v/arena-btreemap.svg)](https://crates.io/crates/arena-btreemap)
[![documentation](https://docs.rs/arena-btreemap/badge.svg)](https://docs.rs/arena-btreemap)
[![license](https://img.shields.io/crates/l/arena-btreemap.svg)](#license)

A [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) that supports custom allocators on **stable Rust**, ported directly from the standard library.

## Motivation

The standard library `BTreeMap` supports custom allocators, but only on nightly Rust (behind `#![feature(allocator_api)]` and `#![feature(btreemap_alloc)]`). The upstream crate ports that implementation to stable Rust using [`allocator-api2`](https://crates.io/crates/allocator-api2).

This fork keeps the ported B-tree implementation and exposes its existing bulk
builder for callers that can guarantee strictly increasing, unique keys.

## Installation

For the published upstream release:

```toml
[dependencies]
arena-btreemap = "0.1"
```

For this experimental branch:

```toml
[dependencies]
arena-btreemap = { git = "https://github.com/CarlKCarlK/arena-btreemap", branch = "sorted-unique-constructor" }
```

Applications should replace `branch` with a specific `rev` for reproducible
builds. For `no_std` environments, also set `default-features = false`.

For the published upstream release in a `no_std` environment:

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

### Sorted-unique bulk construction

This fork adds constructors that bypass collection, sorting, and
deduplication when the input is already known to contain strictly increasing,
unique keys:

```rust
use arena_btreemap::BTreeMap;

let map = unsafe {
    BTreeMap::from_sorted_unique_iter_unchecked([(1, "one"), (3, "three")])
};
assert_eq!(map.get(&3), Some(&"three"));
```

The allocator-aware equivalent is
`BTreeMap::from_sorted_unique_iter_in_unchecked`. Both functions are `unsafe`:
passing duplicate or out-of-order keys violates the constructor contract.

### API parity with std

The upstream API closely follows `std::collections::BTreeMap`. All read sites (`.iter()`, `.get()`, `.keys()`, `.range()`) work unchanged. Construction sites change from `BTreeMap::new()` to `BTreeMap::new_in(alloc)` when using a custom allocator. This fork additionally exposes the two sorted-unique constructors above.

## What the upstream port changed from std

Only what was necessary to compile on stable:

- Import paths swapped from `std::alloc` → `allocator_api2::alloc`
- Nightly-only attributes (`#[unstable]`, `#[stable]`, `#[rustc_diagnostic_item]`, `#[may_dangle]`) stripped
- `TrustedLen` impls removed (unstable trait)
- `extend_one` methods removed (unstable)
- `write_length_prefix` → stable equivalent
- `slice_ptr_get` → manual pointer arithmetic

The core B-tree implementation is derived directly from the standard library.

## What this experimental branch adds

- Unchecked sorted-unique constructors for the global and custom allocator cases
- Direct construction through the existing B-tree bulk builder
- Rust 1.87 compatibility fixes
- Correct `allocator-api2` feature wiring for alloc-only embedded and WASM targets

## `no_std` and WASM

This branch compiles for alloc-only embedded and WASM targets with
`--no-default-features`:

```sh
cargo check --target wasm32-unknown-unknown --no-default-features
cargo check --target wasm32-wasip1 --no-default-features
cargo check --target thumbv7m-none-eabi --no-default-features
```

The inherited `no_std` feature is a compatibility marker. Disabling the
default `std` feature is what selects an alloc-only build; Cargo features are
additive, so `features = ["no_std"]` by itself does not disable `std`.

## Testing

The fork adds four sorted-unique constructor tests to the upstream suites. With
all features enabled, 141 unit, integration, and documentation tests pass:

| Suite | Tests | Description |
|-------|-------|-------------|
| Unit | 32 | Node structure, search, iteration |
| Arena | 9 | Custom allocator verification |
| Sorted unique | 4 | Empty, single-level, multi-level, and custom allocator construction |
| Stress | 12 | 10K elements, random ops, repeated cycles |
| Std compat | 12 | Direct comparison with std `BTreeMap` |
| Doctests | 72 | std doctests adapted to this crate |

```sh
cargo +1.87 test --all-features
cargo +1.87 check --no-default-features
cargo check --target wasm32-wasip1 --no-default-features
cargo check --target thumbv7m-none-eabi --no-default-features
```

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option. This matches the licensing of the Rust standard library source from which this crate is ported.

## Support upstream

If the upstream crate is useful to you, consider supporting its original development:

- **Solana**: `Eu8wQcW68TKMs1a6eqzZu8znzU52QLqQugAMG8uCD6y6`
- **EVM** (Ethereum / Base / Arbitrum / Optimism / Polygon): `0x2733ff7c865C56d565a99BE1DC11B81cc76850A5`
- **XRP Ledger**: `r4X6e7McAQj7e8vBCeued1RYu4mCJrREDG`
