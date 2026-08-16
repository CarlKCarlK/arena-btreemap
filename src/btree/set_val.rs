/// Zero-Sized Type (ZST) for internal `BTreeSet` values.
/// Used instead of `()` to differentiate between:
/// * `BTreeMap<T, ()>` (possible user-defined map)
/// * `BTreeMap<T, SetValZST>` (internal set representation)
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Default)]
pub(super) struct SetValZST;

/// A trait to differentiate between `BTreeMap` and `BTreeSet` values.
/// Returns `true` only for type `SetValZST`, `false` for all other types.
///
/// On stable Rust (without specialization), this trait always returns `false`
/// for any type. The `SetValZST` specialization is only available on nightly.
/// Since we don't port `BTreeSet`, this blanket implementation suffices.
pub(super) trait IsSetVal {
    fn is_set_val() -> bool;
}

// Blanket implementation — always false on stable (no specialization)
impl<V> IsSetVal for V {
    fn is_set_val() -> bool {
        false
    }
}
