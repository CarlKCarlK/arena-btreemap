//! Arena allocator integration tests.
//!
//! These tests verify that BTreeMap works correctly with custom allocators,
//! which is the entire value proposition of this crate. We use a counting
//! allocator wrapper to prove the custom allocator path is exercised.

use arena_btreemap::BTreeMap;
use arena_btreemap::alloc::{Allocator, Global, AllocError};
use std::ptr::NonNull;
use std::alloc::Layout;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A simple counting allocator that wraps `Global` and tracks allocations.
/// This proves the custom allocator path is actually exercised.
struct CountingAllocator {
    inner: Global,
    alloc_count: AtomicUsize,
}

impl CountingAllocator {
    fn new() -> Self {
        Self {
            inner: Global,
            alloc_count: AtomicUsize::new(0),
        }
    }

    fn alloc_count(&self) -> usize {
        self.alloc_count.load(Ordering::Relaxed)
    }
}

unsafe impl Allocator for &CountingAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.alloc_count.fetch_add(1, Ordering::Relaxed);
        self.inner.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { self.inner.deallocate(ptr, layout) }
    }
}

// &T is automatically Clone in Rust — no need for explicit impl

#[test]
fn custom_allocator_basic() {
    let alloc = CountingAllocator::new();
    let mut map = BTreeMap::new_in(&alloc);
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");

    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), Some(&"b"));
    assert_eq!(map.get(&3), Some(&"c"));
    assert!(alloc.alloc_count() > 0);
}

#[test]
fn custom_allocator_large_tree() {
    let alloc = CountingAllocator::new();
    let mut map = BTreeMap::new_in(&alloc);

    for i in 0..500 {
        map.insert(i, i * 2);
    }

    assert_eq!(map.len(), 500);
    for i in 0..500 {
        assert_eq!(map.get(&i), Some(&(i * 2)));
    }

    // The allocator should have been called for tree node allocations
    assert!(alloc.alloc_count() > 10, "Expected many allocations, got {}", alloc.alloc_count());
}

#[test]
fn custom_allocator_clone() {
    let alloc = CountingAllocator::new();
    let mut map = BTreeMap::new_in(&alloc);
    map.insert(1, "a");
    map.insert(2, "b");

    // Cloning should use the same allocator type
    let map2 = map.clone();
    assert_eq!(map, map2);
}

#[test]
fn custom_allocator_remove_and_reinsert() {
    let alloc = CountingAllocator::new();
    let mut map = BTreeMap::new_in(&alloc);

    for i in 0..100 {
        map.insert(i, i);
    }

    // Remove all
    for i in 0..100 {
        assert_eq!(map.remove(&i), Some(i));
    }
    assert!(map.is_empty());

    // Reinsert all
    for i in 0..100 {
        map.insert(i, i * 2);
    }
    assert_eq!(map.len(), 100);

    for i in 0..100 {
        assert_eq!(map.get(&i), Some(&(i * 2)));
    }
}

#[test]
fn global_allocator_parity_with_std() {
    // Verify our BTreeMap with Global allocator produces identical results
    // to std::collections::BTreeMap for the same operations
    use std::collections::BTreeMap as StdBTreeMap;

    let mut ours = BTreeMap::new();
    let mut theirs = StdBTreeMap::new();

    let data = [(5, "five"), (1, "one"), (3, "three"), (4, "four"), (2, "two")];

    for (k, v) in data {
        ours.insert(k, v);
        theirs.insert(k, v);
    }

    // Same length
    assert_eq!(ours.len(), theirs.len());

    // Same keys in same order
    let our_keys: Vec<_> = ours.keys().copied().collect();
    let their_keys: Vec<_> = theirs.keys().copied().collect();
    assert_eq!(our_keys, their_keys);

    // Same values
    for k in our_keys {
        assert_eq!(ours.get(&k), theirs.get(&k));
    }

    // Same removal behavior
    assert_eq!(ours.remove(&3), theirs.remove(&3));
    assert_eq!(ours.len(), theirs.len());

    // Still same keys
    let our_keys2: Vec<_> = ours.keys().copied().collect();
    let their_keys2: Vec<_> = theirs.keys().copied().collect();
    assert_eq!(our_keys2, their_keys2);
}

#[test]
fn entry_api_with_custom_allocator() {
    let alloc = CountingAllocator::new();
    let mut map: BTreeMap<String, Vec<i32>, &CountingAllocator> = BTreeMap::new_in(&alloc);

    // or_default with custom allocator
    map.entry("a".to_string()).or_default().push(1);
    map.entry("a".to_string()).or_default().push(2);
    map.entry("b".to_string()).or_default().push(3);

    assert_eq!(map.get("a"), Some(&vec![1, 2]));
    assert_eq!(map.get("b"), Some(&vec![3]));
}

#[test]
fn drop_does_not_panic() {
    // Verify that dropping a map with a custom allocator doesn't panic
    let alloc = CountingAllocator::new();
    {
        let mut map = BTreeMap::new_in(&alloc);
        for i in 0..200 {
            map.insert(i, format!("value_{}", i));
        }
        // map drops here
    }
    // If we reach here, drop didn't panic
}

#[test]
fn split_off_with_custom_allocator() {
    let alloc = CountingAllocator::new();
    let mut map = BTreeMap::new_in(&alloc);

    for i in 0..20 {
        map.insert(i, i);
    }

    let right = map.split_off(&10);

    let left_keys: Vec<_> = map.keys().copied().collect();
    let right_keys: Vec<_> = right.keys().copied().collect();

    assert_eq!(left_keys, (0..10).collect::<Vec<_>>());
    assert_eq!(right_keys, (10..20).collect::<Vec<_>>());
}

#[test]
fn retain_with_custom_allocator() {
    let alloc = CountingAllocator::new();
    let mut map = BTreeMap::new_in(&alloc);

    for i in 0..50 {
        map.insert(i, i);
    }

    map.retain(|&k, &mut _v| k % 2 == 0);

    assert_eq!(map.len(), 25);
    for k in map.keys() {
        assert_eq!(k % 2, 0);
    }
}
