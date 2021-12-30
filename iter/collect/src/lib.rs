use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    hash::Hash,
};

impl<I: Iterator> IterExt for I {}
pub trait IterExt: Iterator + Sized {
    fn to_vec(self) -> Vec<Self::Item> {
        self.collect()
    }
    fn rev_to_vec(self) -> Vec<Self::Item> {
        let mut ret = self.to_vec();
        ret.reverse();
        ret
    }
    fn to_vec_rev(self) -> std::vec::IntoIter<Self::Item> {
        self.rev_to_vec().into_iter()
    }
    fn to_vec_deque(self) -> VecDeque<Self::Item> {
        self.collect()
    }
    fn to_btree_set(self) -> BTreeSet<Self::Item>
    where
        Self::Item: Ord,
    {
        self.collect()
    }
    fn to_hash_set(self) -> HashSet<Self::Item>
    where
        Self::Item: Eq + Hash,
    {
        self.collect()
    }
    fn to_btree_map<K, V>(self) -> BTreeMap<K, V>
    where
        Self: Iterator<Item = (K, V)>,
        K: Ord,
    {
        self.collect()
    }
    fn to_hash_map<K, V>(self) -> HashMap<K, V>
    where
        Self: Iterator<Item = (K, V)>,
        K: Eq + Hash,
    {
        self.collect()
    }
}
