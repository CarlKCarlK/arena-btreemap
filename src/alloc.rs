//! Re-exports of allocator and allocation-related types from `allocator-api2`.
//!
//! This module serves as the bridge between the ported std BTreeMap source and
//! the stable allocator API. All references to `crate::alloc` in the original
//! std source resolve to this module.

pub use allocator_api2::alloc::{
    alloc, Allocator, Global, Layout, System,
};

/// Re-export of `Box` from `allocator-api2`.
/// In std, `Box` lives at `crate::boxed::Box`. Here we re-export it from
/// `allocator_api2::boxed` into our `alloc` module so that the ported code's
/// `use crate::alloc::Box` resolves correctly.
pub use allocator_api2::boxed::Box;

/// Re-export of `Vec` from `allocator-api2`.
/// In std, `Vec` lives at `crate::vec::Vec`. Here we re-export it from
/// `allocator_api2::vec` into our `alloc` module so that the ported code's
/// `use crate::alloc::Vec` resolves correctly.
pub use allocator_api2::vec::Vec;
