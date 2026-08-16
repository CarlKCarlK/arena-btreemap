//! Standard library behavioral parity tests.
//!
//! These tests verify that our ported BTreeMap produces identical results
//! to `std::collections::BTreeMap` across a wide range of operations.
//! If any of these fail, our port has diverged from std semantics.

use arena_btreemap::BTreeMap;
use std::collections::BTreeMap as Std;

fn assert_maps_equal<K: Ord + std::fmt::Debug + Copy, V: PartialEq + std::fmt::Debug>(
    ours: &BTreeMap<K, V>,
    theirs: &Std<K, V>,
) {
    assert_eq!(ours.len(), theirs.len(), "len mismatch");
    let our_keys: Vec<K> = ours.keys().copied().collect();
    let their_keys: Vec<K> = theirs.keys().copied().collect();
    assert_eq!(our_keys, their_keys, "keys mismatch");
    for k in &our_keys {
        assert_eq!(ours.get(k), theirs.get(k), "value mismatch for key {:?}", k);
    }
}

#[test]
fn parity_insert_order() {
    let mut ours = BTreeMap::new();
    let mut theirs = Std::new();

    let order = [50, 10, 90, 30, 70, 20, 80, 40, 60, 5, 95, 15, 85, 25, 75, 35, 65, 45, 55];

    for &k in &order {
        ours.insert(k, k * 10);
        theirs.insert(k, k * 10);
        assert_maps_equal(&ours, &theirs);
    }
}

#[test]
fn parity_remove_all() {
    let mut ours = BTreeMap::new();
    let mut theirs = Std::new();

    for i in 0..200 {
        ours.insert(i, i);
        theirs.insert(i, i);
    }

    // Remove in random order
    let remove_order = [100, 0, 199, 50, 150, 25, 175, 75, 125, 1, 198, 2, 197];
    for &k in &remove_order {
        assert_eq!(ours.remove(&k), theirs.remove(&k));
        assert_maps_equal(&ours, &theirs);
    }
}

#[test]
fn parity_range() {
    let mut ours = BTreeMap::new();
    let mut theirs = Std::new();

    for i in 0..100 {
        ours.insert(i, i * 2);
        theirs.insert(i, i * 2);
    }

    // Exclusive range
    let our_range: Vec<_> = ours.range(10..20).map(|(&k, &v)| (k, v)).collect();
    let their_range: Vec<_> = theirs.range(10..20).map(|(&k, &v)| (k, v)).collect();
    assert_eq!(our_range, their_range);

    // Inclusive range
    let our_range: Vec<_> = ours.range(10..=20).map(|(&k, &v)| (k, v)).collect();
    let their_range: Vec<_> = theirs.range(10..=20).map(|(&k, &v)| (k, v)).collect();
    assert_eq!(our_range, their_range);

    // Unbounded ranges
    let our_range: Vec<_> = ours.range(..5).map(|(&k, &v)| (k, v)).collect();
    let their_range: Vec<_> = theirs.range(..5).map(|(&k, &v)| (k, v)).collect();
    assert_eq!(our_range, their_range);

    let our_range: Vec<_> = ours.range(95..).map(|(&k, &v)| (k, v)).collect();
    let their_range: Vec<_> = theirs.range(95..).map(|(&k, &v)| (k, v)).collect();
    assert_eq!(our_range, their_range);
}

#[test]
fn parity_entry_api() {
    let mut ours: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    let mut theirs: Std<i32, Vec<i32>> = Std::new();

    let ops = [(1, 10), (2, 20), (1, 11), (3, 30), (2, 21), (1, 12)];

    for (k, v) in ops {
        ours.entry(k).or_default().push(v);
        theirs.entry(k).or_default().push(v);
    }

    assert_maps_equal(&ours, &theirs);
}

#[test]
fn parity_split_off() {
    let mut ours = BTreeMap::new();
    let mut theirs = Std::new();

    for i in 0..100 {
        ours.insert(i, i);
        theirs.insert(i, i);
    }

    let our_right = ours.split_off(&50);
    let their_right = theirs.split_off(&50);

    assert_maps_equal(&ours, &theirs);
    assert_maps_equal(&our_right, &their_right);
}

#[test]
fn parity_append() {
    let mut ours_a = BTreeMap::new();
    let mut ours_b = BTreeMap::new();
    let mut theirs_a = Std::new();
    let mut theirs_b = Std::new();

    for i in 0..50 {
        ours_a.insert(i, i);
        theirs_a.insert(i, i);
    }
    for i in 50..100 {
        ours_b.insert(i, i);
        theirs_b.insert(i, i);
    }

    ours_a.append(&mut ours_b);
    theirs_a.append(&mut theirs_b);

    assert_maps_equal(&ours_a, &theirs_a);
    assert_eq!(ours_b.len(), theirs_b.len());
}

#[test]
fn parity_iteration() {
    let mut ours = BTreeMap::new();
    let mut theirs = Std::new();

    let data = [42, 17, 99, 3, 55, 28, 71, 14, 63, 36, 88, 7, 21, 77, 50];
    for &k in &data {
        ours.insert(k, k);
        theirs.insert(k, k);
    }

    // Forward iteration
    let our_fwd: Vec<_> = ours.iter().map(|(&k, &v)| (k, v)).collect();
    let their_fwd: Vec<_> = theirs.iter().map(|(&k, &v)| (k, v)).collect();
    assert_eq!(our_fwd, their_fwd);

    // Reverse iteration
    let our_rev: Vec<_> = ours.iter().rev().map(|(&k, &v)| (k, v)).collect();
    let their_rev: Vec<_> = theirs.iter().rev().map(|(&k, &v)| (k, v)).collect();
    assert_eq!(our_rev, their_rev);
}

#[test]
fn parity_into_iter() {
    let mut ours = BTreeMap::new();
    let mut theirs = Std::new();

    for i in 0..100 {
        ours.insert(i, i * 2);
        theirs.insert(i, i * 2);
    }

    let our_pairs: Vec<_> = ours.into_iter().collect();
    let their_pairs: Vec<_> = theirs.into_iter().collect();
    assert_eq!(our_pairs, their_pairs);
}

#[test]
fn parity_first_last() {
    let mut ours = BTreeMap::new();
    let mut theirs = Std::new();

    let keys = [42, 17, 99, 3, 55, 28, 71, 14, 63, 36];
    for &k in &keys {
        ours.insert(k, k);
        theirs.insert(k, k);
    }

    assert_eq!(ours.first_key_value(), theirs.first_key_value());
    assert_eq!(ours.last_key_value(), theirs.last_key_value());
}

#[test]
fn parity_from_iter() {
    let data = [(5, "five"), (1, "one"), (3, "three"), (4, "four"), (2, "two")];

    let ours: BTreeMap<i32, &str> = data.into_iter().collect();
    let theirs: Std<i32, &str> = data.into_iter().collect();

    assert_maps_equal(&ours, &theirs);
}

#[test]
fn parity_extend() {
    let mut ours = BTreeMap::new();
    let mut theirs = Std::new();

    ours.extend([(1, "a"), (2, "b"), (3, "c")]);
    theirs.extend([(1, "a"), (2, "b"), (3, "c")]);

    assert_maps_equal(&ours, &theirs);
}

#[test]
fn parity_debug_format() {
    let mut ours = BTreeMap::new();
    let mut theirs = Std::new();

    ours.insert(1, "a");
    ours.insert(2, "b");
    theirs.insert(1, "a");
    theirs.insert(2, "b");

    assert_eq!(format!("{:?}", ours), format!("{:?}", theirs));
}
