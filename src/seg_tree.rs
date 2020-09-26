use super::algebra::Monoid;
use super::util::range_from;
use std::ops::{Range, RangeBounds};

/// 便利な列 `st`
pub struct SegTree<M: Monoid> {
    len: usize,
    val: M::Item,
    child: Option<Box<(SegTree<M>, SegTree<M>)>>,
}

impl<M: Monoid> SegTree<M> {
    /// `st = [M::id(); n]`
    pub fn new(n: usize) -> Self {
        Self::from(&vec![M::id(); n][..])
    }
    /// `st[i] = v`
    pub fn set(&mut self, i: usize, v: M::Item) {
        assert!(i < self.len, "index out: {}/{}", i, self.len);
        if self.len == 1 {
            return self.val = v;
        }
        let mid = self.len / 2;
        let (left, right) = self.child.as_mut().unwrap().as_mut();
        if i < mid {
            left.set(i, v);
        } else {
            right.set(i - mid, v);
        }
        self.val = M::prod(&left.val, &right.val);
    }
    /// `st[i]`
    pub fn get(&self, i: usize) -> M::Item {
        self.fold(i..=i)
    }
    /// `st[range].iter().fold(M::id(), |a, b| M::prod(&a, b))`
    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Item {
        let Range { start, end } = range_from(range, self.len);
        if start == end {
            return M::id();
        }
        self.fold_inner(start, end)
    }
    fn fold_inner(&self, start: usize, end: usize) -> M::Item {
        if end - start == self.len {
            return self.val.clone();
        }
        let mid = self.len / 2;
        let (left, right) = self.child.as_ref().unwrap().as_ref();
        if end <= mid {
            left.fold_inner(start, end)
        } else if mid <= start {
            right.fold_inner(start - mid, end - mid)
        } else {
            M::prod(
                &left.fold_inner(start, mid),
                &right.fold_inner(0, end - mid),
            )
        }
    }
    /// `pred(st.fold(start..end))` なる最大の `end`
    /// `pred(M::id())` が要請される
    pub fn max_end<P>(&self, start: usize, mut pred: P) -> usize
    where
        P: FnMut(&M::Item) -> bool,
    {
        assert!(start <= self.len, "index out: {}/{}", start, self.len);
        let mut acc = M::id();
        self.max_end_inner(start, &mut pred, &mut acc)
    }
    fn max_end_inner<P>(&self, start: usize, pred: &mut P, acc: &mut M::Item) -> usize
    where
        P: FnMut(&M::Item) -> bool,
    {
        if start == 0 {
            let merged = M::prod(acc, &self.val);
            if pred(&merged) {
                *acc = merged;
                return self.len;
            } else if self.len == 1 {
                return 0;
            }
        } else if start == self.len {
            return self.len;
        }
        let mid = self.len / 2;
        let (left, right) = self.child.as_ref().unwrap().as_ref();
        if start < mid {
            let res_left = left.max_end_inner(start, pred, acc);
            if res_left < mid {
                res_left
            } else {
                mid + right.max_end_inner(0, pred, acc)
            }
        } else {
            mid + right.max_end_inner(start - mid, pred, acc)
        }
    }
    /// `pred(st.fold(start..end))` なる最小の `start`
    /// `pred(M::id())` が要請される
    pub fn min_start<P>(&self, end: usize, mut pred: P) -> usize
    where
        P: FnMut(&M::Item) -> bool,
    {
        assert!(end <= self.len, "index out: {}/{}", end, self.len);
        let mut acc = M::id();
        self.min_start_inner(end, &mut pred, &mut acc)
    }
    fn min_start_inner<P>(&self, end: usize, pred: &mut P, acc: &mut M::Item) -> usize
    where
        P: FnMut(&M::Item) -> bool,
    {
        if end == self.len {
            let merged = M::prod(acc, &self.val);
            if pred(&merged) {
                *acc = merged;
                return 0;
            } else if self.len == 1 {
                return 1;
            }
        } else if end == 0 {
            return 0;
        }
        let mid = self.len / 2;
        let (left, right) = self.child.as_ref().unwrap().as_ref();
        if mid <= end {
            let res_right = right.min_start_inner(end - mid, pred, acc);
            if res_right > 0 {
                mid + res_right
            } else {
                left.min_start_inner(mid, pred, acc)
            }
        } else {
            left.min_start_inner(end, pred, acc)
        }
    }
}

impl<M: Monoid> From<&[M::Item]> for SegTree<M> {
    fn from(slice: &[M::Item]) -> Self {
        if slice.len() == 1 {
            SegTree {
                len: 1,
                val: slice[0].clone(),
                child: None,
            }
        } else {
            let mid = slice.len() / 2;
            let left = Self::from(&slice[..mid]);
            let right = Self::from(&slice[mid..]);
            Self {
                len: slice.len(),
                val: M::prod(&left.val, &right.val),
                child: Some(Box::new((left, right))),
            }
        }
    }
}
