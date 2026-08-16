//! Serde `Serialize` and `Deserialize` impls for `BTreeMap`.
//!
//! Enabled by the `serde` cargo feature. Mirrors std's BTreeMap serde impls:
//! serializes as a map (sequence of key-value pairs), deserializes by
//! collecting into a new `BTreeMap` with the `Global` allocator.

use crate::BTreeMap;
use core::fmt;
use core::marker::PhantomData;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<K, V> Serialize for BTreeMap<K, V>
where
    K: Serialize + Ord,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self)
    }
}

impl<'de, K, V> Deserialize<'de> for BTreeMap<K, V>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BTreeMapVisitor<K, V> {
            _marker: PhantomData<BTreeMap<K, V>>,
        }

        impl<'de, K, V> Visitor<'de> for BTreeMapVisitor<K, V>
        where
            K: Deserialize<'de> + Ord,
            V: Deserialize<'de>,
        {
            type Value = BTreeMap<K, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values = BTreeMap::new();

                while let Some((key, value)) = map.next_entry()? {
                    values.insert(key, value);
                }

                Ok(values)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = BTreeMap::new();

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
