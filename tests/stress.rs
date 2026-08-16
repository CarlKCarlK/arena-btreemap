//! Stress tests — large-scale operations, random workloads, and edge cases
//! that exercise tree splitting, merging, and rebalancing.

use arena_btreemap::BTreeMap;
use std::collections::BTreeMap as StdBTreeMap;

#[test]
fn stress_10k_inserts_then_removes() {
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    let mut std_map: StdBTreeMap<i32, i32> = StdBTreeMap::new();

    // Insert 10K elements in reverse order to stress splitting
    for i in (0..10_000i32).rev() {
        map.insert(i, i * 3);
        std_map.insert(i, i * 3);
    }

    assert_eq!(map.len(), 10_000);
    assert_eq!(map.len(), std_map.len());

    // Verify all present and correct
    for i in 0..10_000i32 {
        let our_val = map.get(&i);
        let std_val = std_map.get(&i);
        if our_val != Some(&(i * 3)) {
            panic!("OUR MAP WRONG at i={}: got {:?}, expected Some({})", i, our_val, i * 3);
        }
        if our_val != std_val {
            panic!("MISMATCH at i={}: our={:?}, std={:?}", i, our_val, std_val);
        }
    }

    // Remove every other element
    for i in (0..10_000i32).step_by(2) {
        let our_removed = map.remove(&i);
        let std_removed = std_map.remove(&i);
        assert_eq!(our_removed, Some(i * 3));
        assert_eq!(our_removed, std_removed);
    }

    assert_eq!(map.len(), 5_000);
    assert_eq!(map.len(), std_map.len());

    // Verify remaining
    for i in (1..10_000).step_by(2) {
        assert_eq!(map.get(&i), Some(&(i * 3)));
    }
}

#[test]
fn stress_random_operations() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut map = BTreeMap::new();
    let mut std_map = StdBTreeMap::new();

    // Pseudo-random but deterministic sequence
    let mut seed: u64 = 12345;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        seed
    };

    for _ in 0..5_000 {
        let key = (next() % 1000) as i32;
        let op = next() % 3;

        match op {
            0 => {
                // Insert
                let val = key as i32 * 2;
                assert_eq!(map.insert(key, val), std_map.insert(key, val));
            }
            1 => {
                // Remove
                assert_eq!(map.remove(&key), std_map.remove(&key));
            }
            _ => {
                // Lookup
                assert_eq!(map.get(&key), std_map.get(&key));
            }
        }
    }

    // Final state should match std
    assert_eq!(map.len(), std_map.len());

    let our_keys: Vec<_> = map.keys().copied().collect();
    let std_keys: Vec<_> = std_map.keys().copied().collect();
    assert_eq!(our_keys, std_keys);
}

#[test]
fn stress_repeated_insert_remove_cycle() {
    let mut map = BTreeMap::new();

    // Repeatedly insert and remove the same keys to test
    // node reuse and structural integrity
    for cycle in 0..50 {
        for i in 0..200 {
            map.insert(i, cycle);
        }
        for i in 0..200 {
            assert_eq!(map.remove(&i), Some(cycle));
        }
        assert!(map.is_empty(), "Map should be empty after cycle {}", cycle);
    }
}

#[test]
fn stress_large_clone() {
    let mut map = BTreeMap::new();
    for i in 0..5_000 {
        map.insert(i, format!("value_{}", i));
    }

    let cloned = map.clone();

    // Both maps should have identical content
    assert_eq!(map.len(), cloned.len());

    for i in 0..5_000 {
        assert_eq!(map.get(&i), cloned.get(&i));
    }

    // Modify original, clone should be unaffected
    map.insert(9999, "modified".to_string());
    assert_eq!(cloned.get(&9999), None);
}

#[test]
fn stress_range_on_large_map() {
    let mut map = BTreeMap::new();
    for i in 0..10_000 {
        map.insert(i, i);
    }

    // Range queries
    let r: Vec<_> = map.range(100..200).map(|(&k, &v)| (k, v)).collect();
    assert_eq!(r.len(), 100);
    assert_eq!(r[0], (100, 100));
    assert_eq!(r[99], (199, 199));

    // Inclusive range
    let r: Vec<_> = map.range(100..=200).map(|(&k, &v)| (k, v)).collect();
    assert_eq!(r.len(), 101);

    // Range from a single element
    let r: Vec<_> = map.range(500..=500).map(|(&k, &v)| (k, v)).collect();
    assert_eq!(r, vec![(500, 500)]);
}

#[test]
fn stress_clear_and_reuse() {
    let mut map = BTreeMap::new();

    for round in 0..10 {
        for i in 0..1000 {
            map.insert(i, round);
        }
        assert_eq!(map.len(), 1000);
        map.clear();
        assert!(map.is_empty());
    }
}

#[test]
fn stress_append_large() {
    let mut a = BTreeMap::new();
    let mut b = BTreeMap::new();

    for i in 0..5_000 {
        a.insert(i, i);
    }
    for i in 5_000..10_000 {
        b.insert(i, i);
    }

    a.append(&mut b);

    assert_eq!(a.len(), 10_000);
    assert!(b.is_empty());

    for i in 0..10_000 {
        assert_eq!(a.get(&i), Some(&i));
    }
}

#[test]
fn edge_case_single_element() {
    let mut map = BTreeMap::new();
    map.insert(42, "answer");

    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&42), Some(&"answer"));
    assert!(map.contains_key(&42));
    assert!(!map.contains_key(&0));

    let keys: Vec<_> = map.keys().copied().collect();
    assert_eq!(keys, vec![42]);

    assert_eq!(map.remove(&42), Some("answer"));
    assert!(map.is_empty());
}

#[test]
fn edge_case_empty_operations() {
    let map: BTreeMap<i32, i32> = BTreeMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    assert_eq!(map.get(&1), None);
    assert!(!map.contains_key(&1));
    assert_eq!(map.first_key_value(), None);
    assert_eq!(map.last_key_value(), None);

    let keys: Vec<_> = map.keys().collect();
    assert!(keys.is_empty());

    let values: Vec<_> = map.values().collect();
    assert!(values.is_empty());
}

#[test]
fn edge_case_duplicate_keys() {
    let mut map = BTreeMap::new();

    for i in 0..100 {
        map.insert(1, i);
    }

    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&99));
}

#[test]
fn edge_case_sequential_keys() {
    // Sequential keys stress the tree's in-order insertion path
    let mut map = BTreeMap::new();
    for i in 0..1000 {
        map.insert(i, i);
    }

    let keys: Vec<_> = map.keys().copied().collect();
    assert_eq!(keys, (0..1000).collect::<Vec<_>>());
}

#[test]
fn edge_case_string_keys() {
    let mut map = BTreeMap::new();
    let words = ["apple", "banana", "cherry", "date", "elderberry", "fig", "grape"];

    for word in &words {
        map.insert(word.to_string(), word.len());
    }

    // Should be in sorted order
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys, words);

    assert_eq!(map.get("cherry"), Some(&6));
    assert_eq!(map.get("fig"), Some(&3));
}
