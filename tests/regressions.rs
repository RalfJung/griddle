use griddle::hash_map::Entry;
use griddle::HashMap;

use fnv::FnvHasher;
use std::hash::BuildHasherDefault;
type FnvBuilder = BuildHasherDefault<FnvHasher>;
type FnvHashMap<K, V> = HashMap<K, V, FnvBuilder>;

/// All quickcheck tests here are run this many times to account for random hashing
const N: usize = 10;

#[test]
fn reserve_shrink_add() {
    for _ in 0..N {
        // [Reserve(18303), ShrinkToFit, Add(-94, -96)]
        let mut map = HashMap::new();
        map.reserve(18303);
        map.shrink_to_fit();
        map.insert(-94i8, -96i8);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn carry_moves_exactly() {
    for _ in 0..N {
        // [AddEntry(-14, 67), Add(6, -14), AddEntry(29, 67), AddEntry(10, 82), Add(-33, -44), Add(37, 88), AddEntry(72, 73), Add(-90, 74), Reserve(45114), AddEntry(-75, -31), Remove(29), RemoveEntry(72), ShrinkToFit, AddEntry(42, -34)]
        let mut map = HashMap::new();
        map.entry(-14i8).or_insert(67);
        map.insert(6, -14);
        map.entry(29).or_insert(67);
        map.entry(10).or_insert(82);
        map.insert(-33, -44);
        map.insert(37, 88);
        map.entry(72).or_insert(73);
        map.entry(-90).or_insert(74);
        map.reserve(45114);
        map.entry(-75).or_insert(-31);
        map.remove(&29);
        if let Entry::Occupied(e) = map.entry(72) {
            e.remove_entry();
        }
        map.shrink_to_fit();
        map.entry(42).or_insert(-34);
    }
}

#[test]
fn clone_with_leftovers() {
    for _ in 0..N {
        // [Add(78, -67), Add(-82, 32), Add(30, 17), AddEntry(-93, -94), Add(-64, -46), Add(66, 90), Add(-85, -83), Add(82, -7), AddEntry(-77, -21), AddEntry(20, -12), AddEntry(-90, 61), Add(-75, 96), AddEntry(45, -23), AddEntry(95, -6), AddEntry(-16, 7), Add(-20, 37), Add(-96, -15), AddEntry(-31, -50), AddEntry(63, -22), AddEntry(-21, 25), AddEntry(-72, -14), AddEntry(-26, 99), AddEntry(3, -12), AddEntry(-3, -78), Add(-84, -93), Add(-79, -58), Add(12, 39), Add(-89, -53), Add(97, 63), ReplaceWithClone]
        let mut map = HashMap::new();
        map.insert(78, -67);
        map.insert(-82, 32);
        map.insert(30, 17);
        map.entry(-93).or_insert(-94);
        map.insert(-64, -46);
        map.insert(66, 90);
        map.insert(-85, -83);
        map.insert(82, -7);
        map.entry(-77).or_insert(-21);
        map.entry(20).or_insert(-12);
        map.entry(-90).or_insert(61);
        map.insert(-75, 96);
        map.entry(45).or_insert(-23);
        map.entry(95).or_insert(-6);
        map.entry(-16).or_insert(7);
        map.insert(-20, 37);
        map.insert(-96, -15);
        map.entry(-31).or_insert(-50);
        map.entry(63).or_insert(-22);
        map.entry(-21).or_insert(25);
        map.entry(-72).or_insert(-14);
        map.entry(-26).or_insert(99);
        map.entry(3).or_insert(-12);
        map.entry(-3).or_insert(-78);
        map.insert(-84, -93);
        map.insert(-79, -58);
        map.insert(12, 39);
        map.insert(-89, -53);
        map.insert(96, 63);
        let map1 = map;
        let map2 = map1.clone();

        assert_eq!(map1.len(), map2.len());

        // every item yielded by the iterator should exist
        for (k, v) in map2.iter() {
            assert_eq!(map2.get(k), Some(v));
        }

        // every item in map1 should exist in map2, and vice versa
        for (k, v) in map1.iter() {
            assert_eq!(map2.get(k), Some(v));
        }
        for (k, v) in map2.iter() {
            assert_eq!(map1.get(k), Some(v));
        }
    }
}

#[test]
fn accidental_key_eq_in_uninit_mem() {
    for _ in 0..N {
        // Add(54, -56), AddEntry(9, -90), AddEntry(-17, 40), AddEntry(-76, 4), AddEntry(-10, -24), Add(94, 71), AddEntry(-74, 3), Add(-9, 89), Add(-16, 92), Add(-89, 87), AddEntry(-8, -14), ReplaceWithClone, AddEntry(59, 83), AddEntry(-90, 28), RemoveEntry(-40), Add(73, -94), Add(98, -99), AddEntry(71, -62), Add(43, -21), AddEntry(85, -8), Add(26, 43), AddEntry(48, -9), Add(-77, 22), AddEntry(-78, -56), AddEntry(-45, -69), Add(57, -95), RemoveEntry(-19), Add(-23, 73), Add(-37, 88), Add(12, -24), AddEntry(93, 63), AddEntry(-14, -66), ReplaceWithClone
        // pruned slightly
        macro_rules! ops {
            ($map:ident) => {{
                $map.insert(54, 54);
                $map.insert(9, 9);
                $map.insert(-17, -17);
                $map.insert(-76, -76);
                $map.insert(-10, -10);
                $map.insert(94, 94);
                $map.insert(-74, -74);
                $map.insert(-9, -9);
                $map.insert(-16, -16);
                $map.insert(-89, -89);
                $map.insert(-8, -8);
                // $map = $map.clone();
                $map.insert(59, 59);
                $map.insert(-90, -90);
                // $map.remove(&-40);
                $map.insert(73, 73);
                $map.insert(98, 98);
                $map.insert(71, 71);
                $map.insert(43, 43);
                $map.insert(85, 85);
                $map.insert(26, 26);
                $map.insert(48, 48);
                $map.insert(-77, -77);
                $map.insert(-78, -78);
                $map.insert(-45, -45);
                $map.insert(57, 57);
                // $map.remove(&-19);
                $map.insert(-23, -23);
                $map.insert(-37, -37);
                $map.insert(12, 12);
                $map.insert(93, 93);
                $map.insert(-14, -14);
                $map = $map.clone();
            }};
        }
        let mut map: FnvHashMap<i8, i8> = FnvHashMap::default();
        let mut reference = std::collections::HashMap::new();
        ops!(map);
        ops!(reference);

        assert_eq!(map.len(), reference.len());
        // check self-lookup first
        for (k, v) in map.iter() {
            assert_eq!(map.get(k), Some(v), "k = {}", k);
        }
        for (k, v) in map.iter() {
            assert_eq!(reference.get(k), Some(v), "k = {}", k);
        }
        for (k, v) in reference.iter() {
            assert_eq!(map.get(k), Some(v), "k = {}", k);
        }
    }
}
