//! Unit tests for BTreeMap basic operations.
//!
//! These tests verify that the ported BTreeMap behaves identically to
//! `std::collections::BTreeMap` for fundamental operations: insertion,
//! lookup, removal, iteration, and clone semantics.

use crate::btree::map::BTreeMap;
use crate::alloc::Global;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_map() {
        let map: BTreeMap<i32, i32> = BTreeMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn empty_map_with_allocator() {
        let map: BTreeMap<i32, i32, &Global> = BTreeMap::new_in(&Global);
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn insert_and_get() {
        let mut map = BTreeMap::new();
        assert_eq!(map.insert(1, "a"), None);
        assert_eq!(map.insert(2, "b"), None);
        assert_eq!(map.insert(3, "c"), None);

        assert_eq!(map.len(), 3);
        assert!(!map.is_empty());

        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), Some(&"b"));
        assert_eq!(map.get(&3), Some(&"c"));
        assert_eq!(map.get(&4), None);
    }

    #[test]
    fn insert_replace() {
        let mut map = BTreeMap::new();
        assert_eq!(map.insert(1, "a"), None);
        assert_eq!(map.insert(1, "b"), Some("a"));
        assert_eq!(map.get(&1), Some(&"b"));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn contains_key() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(3, "c");
        map.insert(5, "e");

        assert!(map.contains_key(&1));
        assert!(map.contains_key(&3));
        assert!(map.contains_key(&5));
        assert!(!map.contains_key(&0));
        assert!(!map.contains_key(&2));
        assert!(!map.contains_key(&4));
        assert!(!map.contains_key(&6));
    }

    #[test]
    fn remove() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        assert_eq!(map.remove(&2), Some("b"));
        assert_eq!(map.len(), 2);
        assert!(!map.contains_key(&2));

        assert_eq!(map.remove(&2), None);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn remove_entry() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(2, "b");

        assert_eq!(map.remove_entry(&1), Some((1, "a")));
        assert_eq!(map.len(), 1);
        assert!(!map.contains_key(&1));
    }

    #[test]
    fn iteration_order() {
        let mut map = BTreeMap::new();
        // Insert in non-sorted order
        map.insert(5, "five");
        map.insert(1, "one");
        map.insert(3, "three");
        map.insert(4, "four");
        map.insert(2, "two");

        let keys: Vec<_> = map.keys().copied().collect();
        assert_eq!(keys, vec![1, 2, 3, 4, 5]);

        let values: Vec<_> = map.values().copied().collect();
        assert_eq!(values, vec!["one", "two", "three", "four", "five"]);
    }

    #[test]
    fn iter_mut() {
        let mut map = BTreeMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);

        for v in map.values_mut() {
            *v *= 2;
        }

        assert_eq!(map.get(&1), Some(&20));
        assert_eq!(map.get(&2), Some(&40));
        assert_eq!(map.get(&3), Some(&60));
    }

    #[test]
    fn into_iter() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let pairs: Vec<_> = map.into_iter().collect();
        assert_eq!(pairs, vec![(1, "a"), (2, "b"), (3, "c")]);
    }

    #[test]
    fn clear() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(map.get(&1), None);
    }

    #[test]
    fn clone_equality() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");

        let map2 = map.clone();
        assert_eq!(map, map2);

        // Original is unchanged
        assert_eq!(map.get(&1), Some(&"a"));
    }

    #[test]
    fn range_queries() {
        let mut map = BTreeMap::new();
        for i in 0..10 {
            map.insert(i, i * 10);
        }

        let range: Vec<_> = map.range(2..5).map(|(&k, &v)| (k, v)).collect();
        assert_eq!(range, vec![(2, 20), (3, 30), (4, 40)]);

        let range_inclusive: Vec<_> = map.range(2..=5).map(|(&k, &v)| (k, v)).collect();
        assert_eq!(range_inclusive, vec![(2, 20), (3, 30), (4, 40), (5, 50)]);
    }

    #[test]
    fn entry_api_vacant() {
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        map.entry(1).or_insert(10);
        assert_eq!(map.get(&1), Some(&10));

        // Should not overwrite
        map.entry(1).or_insert(20);
        assert_eq!(map.get(&1), Some(&10));
    }

    #[test]
    fn entry_api_occupied() {
        let mut map = BTreeMap::new();
        map.insert(1, 10);

        map.entry(1).and_modify(|v| *v *= 2).or_insert(1);
        assert_eq!(map.get(&1), Some(&20));

        // Vacant entry with and_modify (no-op) then or_insert
        map.entry(2).and_modify(|v| *v *= 2).or_insert(1);
        assert_eq!(map.get(&2), Some(&1));
    }

    #[test]
    fn entry_api_or_default() {
        let mut map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        map.entry(1).or_default().push(42);
        map.entry(1).or_default().push(43);

        assert_eq!(map.get(&1), Some(&vec![42, 43]));
    }

    #[test]
    fn from_iter() {
        let map: BTreeMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")]
            .into_iter()
            .collect();

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), Some(&"b"));
        assert_eq!(map.get(&3), Some(&"c"));
    }

    #[test]
    fn extend() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.extend([(2, "b"), (3, "c")]);

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&2), Some(&"b"));
    }

    #[test]
    fn debug_format() {
        let mut map = BTreeMap::new();
        map.insert(1, "a");
        map.insert(2, "b");

        let s = format!("{:?}", map);
        assert!(s.contains("\"a\""));
        assert!(s.contains("\"b\""));
    }

    #[test]
    fn large_tree_stress() {
        // Insert enough elements to force multiple tree levels (B=6, CAPACITY=11)
        // With 1000 elements, we'll have internal nodes.
        let mut map = BTreeMap::new();
        for i in 0..1000 {
            map.insert(i, i * 2);
        }

        assert_eq!(map.len(), 1000);

        // Verify all entries are retrievable
        for i in 0..1000 {
            assert_eq!(map.get(&i), Some(&(i * 2)), "Missing key {} after insert", i);
        }

        // Verify iteration order
        let keys: Vec<_> = map.keys().copied().collect();
        assert_eq!(keys, (0..1000).collect::<Vec<_>>());

        // Remove half
        for i in (0..1000).step_by(2) {
            assert_eq!(map.remove(&i), Some(i * 2));
        }

        assert_eq!(map.len(), 500);

        // Verify remaining
        for i in (1..1000).step_by(2) {
            assert_eq!(map.get(&i), Some(&(i * 2)));
        }
    }

    #[test]
    fn random_order_insertion() {
        // Insert in a scrambled order, verify sorted iteration
        let order: Vec<i32> = [7, 3, 15, 1, 9, 5, 13, 11, 0, 17, 19, 2, 4, 6, 8, 10, 12, 14, 16, 18]
            .to_vec();

        let mut map = BTreeMap::new();
        for &i in &order {
            map.insert(i, i);
        }

        let keys: Vec<_> = map.keys().copied().collect();
        assert_eq!(keys, (0..20).collect::<Vec<_>>());
    }

    #[test]
    fn split_off() {
        let mut map = BTreeMap::new();
        for i in 0..10 {
            map.insert(i, i);
        }

        let right = map.split_off(&5);

        let left_keys: Vec<_> = map.keys().copied().collect();
        let right_keys: Vec<_> = right.keys().copied().collect();

        assert_eq!(left_keys, vec![0, 1, 2, 3, 4]);
        assert_eq!(right_keys, vec![5, 6, 7, 8, 9]);
    }

    #[test]
    fn append() {
        let mut a = BTreeMap::new();
        a.insert(1, "a");
        a.insert(2, "b");

        let mut b = BTreeMap::new();
        b.insert(3, "c");
        b.insert(4, "d");

        a.append(&mut b);

        assert_eq!(a.len(), 4);
        assert!(b.is_empty());

        assert_eq!(a.get(&3), Some(&"c"));
        assert_eq!(a.get(&4), Some(&"d"));
    }

    #[test]
    fn first_and_last_key_value() {
        let mut map = BTreeMap::new();
        map.insert(3, "c");
        map.insert(1, "a");
        map.insert(2, "b");

        assert_eq!(map.first_key_value(), Some((&1, &"a")));
        assert_eq!(map.last_key_value(), Some((&3, &"c")));
    }
}
