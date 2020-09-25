use std::{
    fmt::Debug,
    ops::{Bound, Range, RangeBounds},
};

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
    pub fn update(&mut self, i: usize, v: M::Item) {
        assert!(i < self.len, "index out: {}/{}", i, self.len);
        if self.len == 1 {
            return self.val = v;
        }
        let mid = self.len / 2;
        let (left, right) = self.child.as_mut().unwrap().as_mut();
        if i < mid {
            left.update(i, v);
        } else {
            right.update(i - mid, v);
        }
        self.val = M::op(&left.val, &right.val);
    }
    /// `st[i]`
    pub fn get(&self, i: usize) -> M::Item {
        self.fold(i..=i)
    }
    /// `st[range].fold(M::id(), |a, b| M::op(&a, &b))`
    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Item {
        let Range { start, end } = range_from(range, self.len);
        self.fold_inner(start, end)
    }
    fn fold_inner(&self, start: usize, end: usize) -> M::Item {
        let len = end - start;
        if len == 0 {
            return M::id();
        } else if len == self.len {
            return self.val.clone();
        }
        let mid = self.len / 2;
        let (left, right) = self.child.as_ref().unwrap().as_ref();
        if end <= mid {
            left.fold_inner(start, end)
        } else if mid <= start {
            right.fold_inner(start - mid, end - mid)
        } else {
            M::op(
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
            let merged = M::op(acc, &self.val);
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
            let merged = M::op(acc, &self.val);
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

/// `[0, len)` 内の半開区間に変換
fn range_from(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
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
                val: M::op(&left.val, &right.val),
                child: Some(Box::new((left, right))),
            }
        }
    }
}

pub trait Element: Sized + Clone + Debug {}
impl<T: Sized + Clone + Debug> Element for T {}

pub trait Monoid {
    type Item: Element;
    fn id() -> Self::Item;
    fn op(a: &Self::Item, b: &Self::Item) -> Self::Item;
    fn fold<'a, I>(iterable: I) -> Self::Item
    where
        I: IntoIterator<Item = &'a Self::Item>,
        Self::Item: 'a,
    {
        iterable
            .into_iter()
            .fold(Self::id(), |a, b| Self::op(&a, b))
    }
}

#[macro_export]
macro_rules! monoid {
    (type $t:ident = ($item:ty, $op:expr, $id:expr)) => {
        enum $t {}
        impl Monoid for $t {
            type Item = $item;
            fn op(a: &$item, b: &$item) -> $item {
                $op(a, b)
            }
            fn id() -> $item {
                $id
            }
        }
    };
}

#[test]
fn test_seg_tree() {
    monoid!(type M = (i32, |a, b| a + b, 0));
    let sq = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let st = SegTree::<M>::from(&sq[..]);
    for i in 0..sq.len() {
        for j in i..sq.len() {
            assert_eq!(sq[i..j].iter().sum::<i32>(), st.fold(i..j))
        }
    }
    for start in 0..=sq.len() {
        for max in 0..=55 {
            let mut acc = 0;
            let mut right = start;
            while right < sq.len() && acc + sq[right] <= max {
                acc += sq[right];
                right += 1;
            }
            assert_eq!(st.max_end(start, |&sum| sum <= max), right);
        }
    }
    for end in 0..=sq.len() {
        for max in 0..=55 {
            let mut acc = 0;
            let mut left = end;
            while left > 0 && acc + sq[left - 1] <= max {
                left -= 1;
                acc += sq[left];
            }
            assert_eq!(
                st.min_start(end, |&sum| sum <= max),
                left,
                "{} {}",
                end,
                max
            );
        }
    }
}
