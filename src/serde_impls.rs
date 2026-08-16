//! Serde `Serialize` and `Deserialize` impls for `BTreeMap`.
//!
//! Enabled by the `serde` cargo feature. Mirrors std's BTreeMap serde impls:
//! serializes as a map (sequence of key-value pairs), deserializes by
//! collecting into a new `BTreeMap` with the appropriate allocator.
//
// The `Serialize` impl is generic over the allocator — it only reads.
// The `Deserialize` impl is generic over `A: Allocator + Clone + Default` —
// it constructs via `BTreeMap::new_in(A::default())`.
// A blanket impl for `A: Allocator + Clone + Default` covers both `Global`
// and custom allocators like `SyncBumpArena`.

use crate::alloc::Allocator;
use crate::BTreeMap;
use core::fmt;
use core::marker::PhantomData;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ───────────────────────────────────────────────────────────────────────
// Serialize — generic over allocator (only reads)
// ───────────────────────────────────────────────────────────────────────

impl<K, V, A> Serialize for BTreeMap<K, V, A>
where
    K: Serialize + Ord,
    V: Serialize,
    A: Allocator + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self)
    }
}

// ───────────────────────────────────────────────────────────────────────
// Deserialize — generic over allocator with Default
// ───────────────────────────────────────────────────────────────────────

impl<'de, K, V, A> Deserialize<'de> for BTreeMap<K, V, A>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
    A: Allocator + Clone + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BTreeMapVisitor<K, V, A> {
            _marker: PhantomData<(K, V, A)>,
        }

        impl<'de, K, V, A> Visitor<'de> for BTreeMapVisitor<K, V, A>
        where
            K: Deserialize<'de> + Ord,
            V: Deserialize<'de>,
            A: Allocator + Clone + Default,
        {
            type Value = BTreeMap<K, V, A>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<Ma>(self, mut map: Ma) -> Result<Self::Value, Ma::Error>
            where
                Ma: MapAccess<'de>,
            {
                let mut values = BTreeMap::new_in(A::default());

                while let Some((key, value)) = map.next_entry()? {
                    values.insert(key, value);
                }

                Ok(values)
            }

            fn visit_seq<Sa>(self, mut seq: Sa) -> Result<Self::Value, Sa::Error>
            where
                Sa: SeqAccess<'de>,
            {
                let mut values = BTreeMap::new_in(A::default());

                while let Some((key, value)) = seq.next_element()? {
                    values.insert(key, value);
                }

                Ok(values)
            }
        }

        let visitor = BTreeMapVisitor {
            _marker: PhantomData,
        };

        deserializer.deserialize_map(visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_roundtrip() {
        let mut map = BTreeMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("c".to_string(), 3);

        let json = serde_json::to_string(&map).unwrap();
        let deserialized: BTreeMap<String, i32> = serde_json::from_str(&json).unwrap();

        assert_eq!(map, deserialized);
    }

    #[test]
    fn test_serde_empty_map() {
        let map: BTreeMap<String, i32> = BTreeMap::new();

        let json = serde_json::to_string(&map).unwrap();
        let deserialized: BTreeMap<String, i32> = serde_json::from_str(&json).unwrap();

        assert_eq!(map, deserialized);
        assert!(deserialized.is_empty());
    }

    #[test]
    fn test_serde_with_postcard() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), 42u64);
        map.insert("key2".to_string(), 99u64);

        let bytes = postcard::to_allocvec(&map).unwrap();
        let deserialized: BTreeMap<String, u64> = postcard::from_bytes(&bytes).unwrap();

        assert_eq!(map, deserialized);
    }

    #[test]
    fn test_serde_preserves_order() {
        let mut map = BTreeMap::new();
        // Insert in non-sorted order
        map.insert(3, "c");
        map.insert(1, "a");
        map.insert(2, "b");

        let json = serde_json::to_string(&map).unwrap();
        let deserialized: BTreeMap<i32, &str> = serde_json::from_str(&json).unwrap();

        // BTreeMap maintains sorted order
        let keys: Vec<_> = deserialized.keys().collect();
        assert_eq!(keys, vec![&1, &2, &3]);
    }
}
