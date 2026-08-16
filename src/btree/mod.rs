//! B-Tree implementation ported from Rust's standard library.
//!
//! This module is a wholesale port of `library/alloc/src/collections/btree/`
//! from the Rust standard library, adapted to use `allocator-api2` for
//! stable Rust compatibility with custom allocators.
//!
//! ## Origin and License
//!
//! The original source is licensed under MIT OR Apache-2.0 as part of the
//! Rust standard library. This port retains that dual license.
//!
//! ## What Changed
//!
//! - Import paths: `crate::alloc` → our `crate::alloc` (re-exporting `allocator-api2`)
//! - Import paths: `crate::boxed::Box` → `crate::alloc::Box`
//! - Import paths: `crate::vec::Vec` → `crate::alloc::Vec`
//! - Import paths: `core::alloc::Allocator` → `crate::alloc::Allocator`
//! - Stripped nightly-only attributes: `#[stable]`, `#[unstable]`, `#[rustc_*]`, ``
//! - Replaced `core::intrinsics::abort()` with `std::process::abort()`
//! - Removed specialization (`default fn`) in `set_val.rs` — not needed without BTreeSet
//! - BTreeSet (`set.rs`) is NOT ported — scope discipline

mod append;
mod borrow;
mod dedup_sorted_iter;
mod fix;
pub mod map;
mod mem;
mod merge_iter;
mod navigate;
mod node;
mod remove;
mod search;
mod set_val;
mod split;
