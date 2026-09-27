use arena_btreemap::{BTreeMap, Global};

#[test]
fn builds_empty_map() {
    let map = unsafe { BTreeMap::<i32, i32>::from_sorted_unique_iter_unchecked([]) };

    assert!(map.is_empty());
}

#[test]
fn builds_single_level_map() {
    let input = [(1, "one"), (3, "three"), (8, "eight")];
    let map = unsafe { BTreeMap::from_sorted_unique_iter_unchecked(input) };

    assert_eq!(map.into_iter().collect::<Vec<_>>(), input);
}

#[test]
fn builds_multi_level_map() {
    let input = (0..1_000).map(|key| (key, key * 2));
    let map = unsafe { BTreeMap::from_sorted_unique_iter_unchecked(input) };

    assert_eq!(map.len(), 1_000);
    assert_eq!(map.first_key_value(), Some((&0, &0)));
    assert_eq!(map.last_key_value(), Some((&999, &1_998)));
    assert!(map.into_iter().eq((0..1_000).map(|key| (key, key * 2))));
}

#[test]
fn builds_with_explicit_allocator() {
    let input = [(2, "two"), (5, "five")];
    let map = unsafe { BTreeMap::from_sorted_unique_iter_in_unchecked(input, Global) };

    assert_eq!(map.into_iter().collect::<Vec<_>>(), input);
}
