use std::{
    iter::FromIterator,
    ops::{Bound, Range, RangeBounds},
};

pub trait SegTreeType {
    type Item: Clone;
    fn id() -> Self::Item;
    fn prod(a: &Self::Item, b: &Self::Item) -> Self::Item;
}

pub enum SegTree<T: SegTreeType> {
    Leaf {
        val: T::Item,
    },
    Node {
        len: usize,
        prod: T::Item,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<T: SegTreeType> SegTree<T> {
    /// `K::id()` が `n` 個
    pub fn new(n: usize) -> Self {
        assert_ne!(n, 0, "segment tree must not be empty.");
        if n == 1 {
            Self::Leaf { val: T::id() }
        } else {
            let left = Self::new(n / 2);
            let right = Self::new(n - n / 2);
            Self::Node {
                len: n,
                prod: T::id(),
                left: Box::new(left),
                right: Box::new(right),
            }
        }
    }
    pub fn len(&self) -> usize {
        match self {
            Self::Leaf { .. } => 1,
            Self::Node { len, .. } => *len,
        }
    }
    pub fn is_empty(&self) -> bool {
        false
    }
    /// 全要素の積 O(1)
    pub fn prod(&self) -> &T::Item {
        match self {
            Self::Leaf { val } => val,
            Self::Node { prod, .. } => prod,
        }
    }
    /// `i` 番目を得る O(log n)
    pub fn get(&self, i: usize) -> &T::Item {
        assert!(i < self.len(), "index out: {}/{}", i, self.len());
        match self {
            Self::Leaf { val } => val,
            Self::Node { left, right, .. } => {
                let mid = left.len();
                if i < mid {
                    left.get(i)
                } else {
                    right.get(i - mid)
                }
            }
        }
    }
    /// `i` 番目を変更する O(log n)
    pub fn modify(&mut self, i: usize, f: impl FnOnce(&mut T::Item)) {
        assert!(i < self.len(), "index out: {}/{}", i, self.len());
        match self {
            Self::Leaf { val } => f(val),
            Self::Node {
                prod, left, right, ..
            } => {
                let mid = left.len();
                if i < mid {
                    left.modify(i, f);
                } else {
                    right.modify(i - mid, f);
                }
                *prod = T::prod(left.prod(), right.prod())
            }
        }
    }
    /// `i` 番目を `v` にする O(log n)
    pub fn set(&mut self, i: usize, v: T::Item) {
        self.modify(i, |x| *x = v);
    }
    /// 添字範囲 `range` の要素の積 O(log n)
    pub fn prod_range(&self, range: impl RangeBounds<usize>) -> T::Item {
        let Range { start, end } = range_from(self.len(), range);
        if start == end {
            return T::id();
        } else if start + self.len() == end {
            return self.prod().clone();
        }
        self.prod_range_inner(start, end)
    }
    fn prod_range_inner(&self, start: usize, end: usize) -> T::Item {
        match self {
            Self::Leaf { val } => val.clone(),
            Self::Node {
                len, left, right, ..
            } => {
                let mid = left.len();
                if end <= mid {
                    left.prod_range_inner(start, end)
                } else if mid <= start {
                    right.prod_range_inner(start - mid, end - mid)
                } else if start == 0 {
                    T::prod(left.prod(), &right.prod_range_inner(0, end - mid))
                } else if end == *len {
                    T::prod(&left.prod_range_inner(start, mid), right.prod())
                } else {
                    T::prod(
                        &left.prod_range_inner(start, mid),
                        &right.prod_range_inner(0, end - mid),
                    )
                }
            }
        }
    }
    /// `pred(self.prod_range(start..end))` なる最大の `end`
    /// `pred(K::id())` が要請される
    pub fn max_end(&self, start: usize, mut p: impl FnMut(&T::Item) -> bool) -> usize {
        assert!(start <= self.len(), "index out: {}/{}", start, self.len());
        if start == self.len() {
            return start;
        }
        let mut acc = T::id();
        self.max_end_inner(start, &mut p, &mut acc)
    }
    fn max_end_inner(
        &self,
        start: usize,
        p: &mut impl FnMut(&T::Item) -> bool,
        acc: &mut T::Item,
    ) -> usize {
        match self {
            Self::Leaf { val } => {
                if p(&T::prod(val, acc)) {
                    1
                } else {
                    0
                }
            }
            Self::Node {
                prod, left, right, ..
            } => {
                let merged = T::prod(acc, prod);
                if p(&merged) {
                    *acc = merged;
                    return self.len();
                }
                let mid = left.len();
                if mid <= start {
                    return mid + right.max_end_inner(start - mid, p, acc);
                }
                let res_l = left.max_end_inner(start, p, acc);
                if res_l != mid {
                    res_l
                } else {
                    mid + right.max_end_inner(0, p, acc)
                }
            }
        }
    }
    /// `pred(self.prod_range(start..end))` なる最小の `start`
    /// `pred(K::id())` が要請される
    pub fn min_start(&self, end: usize, mut p: impl FnMut(&T::Item) -> bool) -> usize {
        assert!(end <= self.len(), "index out: {}/{}", end, self.len());
        if end == 0 {
            return 0;
        }
        let mut acc = T::id();
        self.min_start_inner(end, &mut p, &mut acc)
    }
    fn min_start_inner(
        &self,
        end: usize,
        p: &mut impl FnMut(&T::Item) -> bool,
        acc: &mut T::Item,
    ) -> usize {
        match self {
            Self::Leaf { val } => {
                if p(&T::prod(val, acc)) {
                    0
                } else {
                    1
                }
            }
            Self::Node {
                prod, left, right, ..
            } => {
                let merged = T::prod(prod, acc);
                if p(&merged) {
                    *acc = merged;
                    return 0;
                }
                let mid = left.len();
                if end <= mid {
                    return left.min_start_inner(end, p, acc);
                }
                let res_right = right.min_start_inner(end - mid, p, acc);
                if res_right != 0 {
                    res_right
                } else {
                    left.min_start_inner(mid, p, acc)
                }
            }
        }
    }
}

fn range_from(len: usize, range: impl RangeBounds<usize>) -> Range<usize> {
    use Bound::*;
    let start = match range.start_bound() {
        Included(&a) => a,
        Excluded(&a) => a + 1,
        Unbounded => 0,
    };
    let end = match range.end_bound() {
        Excluded(&a) => a,
        Included(&a) => a + 1,
        Unbounded => len,
    };
    assert!(start <= end, "invalid range: {}..{}", start, end);
    assert!(end <= len, "index out: {}/{}", end, len);
    Range { start, end }
}

impl<T: SegTreeType> From<&[T::Item]> for SegTree<T> {
    fn from(slice: &[T::Item]) -> Self {
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
                prod: T::prod(left.prod(), right.prod()),
                left: Box::new(left),
                right: Box::new(right),
            }
        }
    }
}

impl<T: SegTreeType> FromIterator<T::Item> for SegTree<T> {
    fn from_iter<I: IntoIterator<Item = T::Item>>(iter: I) -> Self {
        Self::from(&iter.into_iter().collect::<Vec<_>>()[..])
    }
}
