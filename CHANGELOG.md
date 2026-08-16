# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-08-17

### Changed

- `Default`, `From<[(K, V); N]>`, and `FromIterator<(K, V)>` impls are now generic over `A: Allocator + Clone + Default`, enabling `BTreeMap::default()` and `BTreeMap::from(...)` to work with custom allocators.
- Doctests updated with type annotations to resolve type inference ambiguity introduced by the generic impls.

## [0.1.1] - 2026-08-16

### Added

- `serde` feature with `Serialize`/`Deserialize` impls for `BTreeMap<K, V, A>`.

## [0.1.0] - 2026-08-16

### Added

- Initial release.
- Full `BTreeMap` implementation ported from `library/alloc/src/collections/btree/` in the Rust standard library.
- Custom allocator support on stable Rust via `allocator-api2`.
- `no_std` and `wasm32-unknown-unknown` support.
- 132 tests: unit, arena integration, stress, std compatibility, and doctests.
- Behavioral parity verified against `std::collections::BTreeMap`.

### Changed from std source

- Import paths: `std::alloc` → `allocator_api2::alloc`
- Stripped nightly-only attributes (`#[unstable]`, `#[stable]`, `#[rustc_diagnostic_item]`, `#[may_dangle]`)
- Removed `TrustedLen` impls (unstable trait)
- Removed `extend_one` methods (unstable)
- `write_length_prefix` → stable equivalent
- `slice_ptr_get` → manual pointer arithmetic
- `core::intrinsics::abort()` → `std::process::abort()` (std) / panic (no_std)

[0.1.0]: https://github.com/guan-tends/arena-btreemap/releases/tag/v0.1.0
