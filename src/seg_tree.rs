use std::{
    iter::FromIterator,
    ops::{Bound, Range, RangeBounds},
};

pub trait SegTreeKind {
    type Item: Clone;
    fn id() -> Self::Item;
    fn prod(a: &Self::Item, b: &Self::Item) -> Self::Item;
}

pub enum SegTree<M: SegTreeKind> {
    Leaf {
        val: M::Item,
    },
    Node {
        len: usize,
        prod: M::Item,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<K: SegTreeKind> SegTree<K> {
    /// `K::id()` が `n` 個
    pub fn new(n: usize) -> Self {
        Self::from(&vec![K::id(); n][..])
    }
    fn len(&self) -> usize {
        match self {
            Self::Leaf { .. } => 1,
            Self::Node { len, .. } => *len,
        }
    }
    /// 全要素の積 O(1)
    pub fn prod(&self) -> &K::Item {
        match self {
            Self::Leaf { val } => val,
            Self::Node { prod, .. } => prod,
        }
    }
    /// `i` 番目を得る O(log n)
    pub fn get(&self, i: usize) -> &K::Item {
        assert!(i < self.len(), "index out: {}/{}", i, self.len());
        match self {
            Self::Leaf { val } => val,
            Self::Node { left, right, .. } => {
                let mid = left.len();
                if i < mid {
                    left.get(i)
                } else {
                    right.get(i)
                }
            }
        }
    }
    /// `i` 番目を `v` にする O(log n)
    pub fn set(&mut self, i: usize, v: K::Item) {
        assert!(i < self.len(), "index out: {}/{}", i, self.len());
        match self {
            Self::Leaf { val } => *val = v,
            Self::Node { left, right, .. } => {
                let mid = left.len();
                if i < mid {
                    left.set(i, v)
                } else {
                    right.set(i - mid, v)
                }
            }
        }
    }
    /// 添字範囲 `range` の要素の積 O(log n)
    pub fn prod_range(&self, range: impl RangeBounds<usize>) -> K::Item {
        let Range { start, end } = self.range_from(range);
        if start == end {
            return K::id();
        }
        self.prod_range_inner(start, end)
    }
    fn prod_range_inner(&self, start: usize, end: usize) -> K::Item {
        match self {
            Self::Leaf { val } => val.clone(),
            Self::Node {
                len,
                prod,
                left,
                right,
            } => {
                if start + len == end {
                    return prod.clone();
                }
                let mid = left.len();
                if end <= mid {
                    left.prod_range_inner(start, end)
                } else if mid <= start {
                    right.prod_range_inner(start - mid, end - mid)
                } else {
                    K::prod(
                        &left.prod_range_inner(start, end),
                        &right.prod_range_inner(start, end),
                    )
                }
            }
        }
    }
    /// `pred(self.prod_range(start..end))` なる最大の `end`
    /// `pred(K::id())` が要請される
    pub fn max_end<P>(&self, start: usize, mut pred: P) -> usize
    where
        P: FnMut(&K::Item) -> bool,
    {
        assert!(start <= self.len(), "index out: {}/{}", start, self.len());
        if start == self.len() {
            return start;
        }
        let mut acc = K::id();
        self.max_end_inner(start, &mut pred, &mut acc)
    }
    fn max_end_inner<P>(&self, start: usize, pred: &mut P, acc: &mut K::Item) -> usize
    where
        P: FnMut(&K::Item) -> bool,
    {
        match self {
            Self::Leaf { val } => {
                if pred(&K::prod(val, acc)) {
                    1
                } else {
                    0
                }
            }
            Self::Node {
                prod, left, right, ..
            } => {
                let merged = K::prod(acc, prod);
                if pred(&merged) {
                    *acc = merged;
                    return self.len();
                }
                let mid = left.len();
                if mid <= start {
                    return mid + right.max_end_inner(start - mid, pred, acc);
                }
                let res_l = left.max_end_inner(start, pred, acc);
                if res_l != mid {
                    res_l
                } else {
                    mid + right.max_end_inner(0, pred, acc)
                }
            }
        }
    }
    /// `pred(self.prod_range(start..end))` なる最小の `start`
    /// `pred(K::id())` が要請される
    pub fn min_start<P>(&self, end: usize, mut pred: P) -> usize
    where
        P: FnMut(&K::Item) -> bool,
    {
        assert!(end <= self.len(), "index out: {}/{}", end, self.len());
        if end == 0 {
            return 0;
        }
        let mut acc = K::id();
        self.min_start_inner(end, &mut pred, &mut acc)
    }
    fn min_start_inner<P>(&self, end: usize, pred: &mut P, acc: &mut K::Item) -> usize
    where
        P: FnMut(&K::Item) -> bool,
    {
        match self {
            Self::Leaf { val } => {
                if pred(&K::prod(val, acc)) {
                    0
                } else {
                    1
                }
            }
            Self::Node {
                prod, left, right, ..
            } => {
                let merged = K::prod(prod, acc);
                if pred(&merged) {
                    *acc = merged;
                    return 0;
                }
                let mid = left.len();
                if end <= mid {
                    return left.min_start_inner(end, pred, acc);
                }
                let res_right = right.min_start_inner(end - mid, pred, acc);
                if res_right != 0 {
                    res_right
                } else {
                    left.min_start_inner(mid, pred, acc)
                }
            }
        }
    }
    fn range_from(&self, range: impl RangeBounds<usize>) -> Range<usize> {
        use Bound::*;
        let start = match range.start_bound() {
            Included(&a) => a,
            Excluded(&a) => a + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Excluded(&a) => a,
            Included(&a) => a + 1,
            Unbounded => self.len(),
        };
        assert!(start <= end, "invalid range: {}..{}", start, end);
        assert!(end <= self.len(), "index out: {}/{}", end, self.len());
        Range { start, end }
    }
}

impl<M: SegTreeKind> From<&[M::Item]> for SegTree<M> {
    fn from(slice: &[M::Item]) -> Self {
        if slice.len() == 1 {
            Self::Leaf {
                val: slice[0].clone(),
            }
        } else {
            let mid = slice.len() / 2;
            let left = Self::from(&slice[..mid]);
            let right = Self::from(&slice[mid..]);
            Self::Node {
                len: slice.len(),
                prod: M::prod(&left.prod(), &right.prod()),
                left: Box::new(left),
                right: Box::new(right),
            }
        }
    }
}

impl<M: SegTreeKind> FromIterator<M::Item> for SegTree<M> {
    fn from_iter<I: IntoIterator<Item = M::Item>>(iter: I) -> Self {
        Self::from(&iter.into_iter().collect::<Vec<_>>()[..])
    }
}
