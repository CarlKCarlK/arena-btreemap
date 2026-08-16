//! # arena-btreemap
//!
//! A [`BTreeMap`] that supports custom allocators on **stable Rust**, ported
//! from the standard library's implementation.
//!
//! ## Why?
//!
//! The standard library's `BTreeMap` is already fully parameterized over
//! `A: Allocator + Clone = Global` — but `new_in(alloc)` is gated behind the
//! nightly-only `allocator_api` feature. This crate unlocks that existing
//! support on stable Rust by porting the implementation with
//! [`allocator-api2`](https://crates.io/crates/allocator-api2).
//!
//! ## O(1) Drop with Arena Allocation
//!
//! When paired with [`bumpalo`](https://crates.io/crates/bumpalo), a bump
//! allocator, dropping a `BTreeMap` becomes O(1) — the arena resets a single
//! pointer instead of walking every tree node.
//!
//! ```rust
//! use arena_btreemap::{Allocator, BTreeMap, Global};
//!
//! // With the default global allocator, works just like std::collections::BTreeMap:
//! let mut map = BTreeMap::new();
//! map.insert("hello", "world");
//! map.insert("foo", "bar");
//! assert_eq!(map.get(&"hello"), Some(&"world"));
//! ```
//!
//! With a custom arena allocator (e.g. `bumpalo`), dropping the map is O(1):
//!
//! ```text
//! use arena_btreemap::BTreeMap;
//! use bumpalo::Bump;
//!
//! let bump = Bump::new();
//! let mut map = BTreeMap::<&str, &str, &Bump>::new_in(&bump);
//! map.insert("hello", "world");
//! // When `map` drops, the arena's bump pointer resets — no per-node deallocation.
//! // When `bump` drops, all memory is freed in one operation.
//! ```
//!
//! ## Drop-in Replacement
//!
//! The API is identical to `std::collections::BTreeMap`. Switching is a
//! type alias change: `BTreeMap<K, V>` → `arena_btreemap::BTreeMap<K, V>`.
//! Construction sites change from `BTreeMap::new()` to `BTreeMap::new_in(alloc)`.
//! All read sites (`.iter()`, `.get()`, `.keys()`) work unchanged.
//!
//! ## License
//!
//! MIT OR Apache-2.0 — same as the Rust standard library source from which
//! this crate is ported.

#![cfg_attr(not(feature = "std"), no_std)]
// The ported std source has bounds in both impl headers and where clauses,
// which triggers clippy::multiple_bound_locations. This is inherent to the
// std source code and not worth modifying the proven algorithm to fix.
#![allow(clippy::multiple_bound_locations)]
// The std BTreeMap uses complex types intentionally — the type definitions
// encode invariants about the tree structure. Suppressing these avoids
// modifying the proven algorithm for style preferences.
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::drop_non_drop)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::manual_strip)]
#![allow(clippy::deref_addrof)]
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::question_mark)]
#![allow(clippy::unnecessary_mut_passed)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::module_inception)]
#![allow(clippy::manual_map)]
#![allow(clippy::mem_replace_option_with_none)]
// Dead code: items used by BTreeSet (not exported) or internal test helpers.
#![allow(dead_code)]
// Unexpected cfg: randomized_layouts is a std test cfg, not used in our crate
#![allow(unexpected_cfgs)]

/// Re-exports of allocator types from `allocator-api2`.
///
/// This module mirrors the structure of `std::alloc` but redirects to
/// `allocator-api2` for stable Rust compatibility with custom allocators.
pub mod alloc;

/// B-Tree implementation ported from std.
pub mod btree;

pub use btree::map::BTreeMap;

// Re-export allocator types for convenience
pub use crate::alloc::{Allocator, AllocError, Global};
