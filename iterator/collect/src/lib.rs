use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    hash::{BuildHasher, Hash},
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
    fn to_hash_set<S>(self) -> HashSet<Self::Item>
    where
        Self::Item: Eq + Hash,
        S: BuildHasher + Default,
    {
        self.collect()
    }
}
