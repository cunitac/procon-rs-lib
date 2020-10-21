use std::{
    iter::FromIterator,
    ops::{Bound, Range, RangeBounds},
};

pub trait LazySegTreeType {
    type Item: Clone;
    type Operator: Clone;
    fn id() -> Self::Item;
    fn prod(a: &Self::Item, b: &Self::Item) -> Self::Item;
    fn composition(a: &Self::Operator, b: &Self::Operator) -> Self::Operator;
    /// 長さ `1` と見なす．
    fn operate(val: &mut Self::Item, op: &Self::Operator) {
        Self::operate_with_len(val, op, 1)
    }
    fn operate_with_len(val: &mut Self::Item, op: &Self::Operator, _len: usize) {
        Self::operate(val, op)
    }
}

pub enum LazySegTree<T: LazySegTreeType> {
    Leaf {
        val: T::Item,
    },
    Node {
        len: usize,
        prod: T::Item,
        lazy: Option<T::Operator>,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<T: LazySegTreeType> From<&[T::Item]> for LazySegTree<T> {
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
                prod: T::id(),
                lazy: None,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
    }
}

impl<T: LazySegTreeType> LazySegTree<T> {
    /// `K::id_item()` が `n` 個
    pub fn new(n: usize) -> Self {
        Self::from(&vec![T::id(); n][..])
    }
    fn propagate(&mut self) {
        match self {
            Self::Leaf { .. } => (),
            Self::Node {
                prod,
                len,
                lazy,
                left,
                right,
                ..
            } => {
                if let Some(lazy) = lazy.as_ref().take() {
                    T::operate_with_len(prod, lazy, *len);
                    left.compose_lazy(lazy);
                    right.compose_lazy(lazy);
                }
            }
        }
    }
    fn compose_lazy(&mut self, op: &T::Operator) {
        match self {
            Self::Leaf { val } => T::operate(val, op),
            Self::Node {
                lazy: Some(lazy), ..
            } => *lazy = T::composition(lazy, op),
            Self::Node { lazy, .. } => *lazy = Some(op.clone()),
        }
    }
    /// 全要素の積
    pub fn prod(&mut self) -> &T::Item {
        match self {
            Self::Leaf { val } => return val,
            Self::Node {
                prod, lazy: None, ..
            } => return prod,
            _ => (),
        };
        self.propagate();
        match self {
            Self::Node {
                prod, lazy: None, ..
            } => prod,
            _ => unreachable!(),
        }
    }
    fn len(&self) -> usize {
        match self {
            Self::Leaf { .. } => 1,
            Self::Node { len, .. } => *len,
        }
    }
    /// `i` 番目を得る
    pub fn get(&mut self, i: usize) -> &T::Item {
        assert!(i < self.len(), "index out: {}/{}", i, self.len());
        self.propagate();
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
    /// `i` 番目を `v` にする
    pub fn set(&mut self, i: usize, v: T::Item) {
        assert!(i < self.len(), "index out: {}/{}", i, self.len());
        self.propagate();
        match self {
            Self::Leaf { val } => *val = v,
            Self::Node {
                prod, left, right, ..
            } => {
                let mid = left.len();
                if i < mid {
                    left.set(i, v)
                } else {
                    right.set(i - mid, v)
                }
                *prod = T::prod(left.prod(), right.prod());
            }
        }
    }
    /// 添え字範囲 `range` に `|x| K::operate(x, op)` をする
    pub fn operate(&mut self, range: impl RangeBounds<usize>, op: &T::Operator) {
        let Range { start, end } = self.range_from(range);
        if start == end {
            return;
        }
        self.operate_inner(start, end, op);
    }
    fn operate_inner(&mut self, start: usize, end: usize, op: &T::Operator) {
        self.propagate();
        match self {
            Self::Leaf { val } => T::operate(val, op),
            Self::Node {
                len, left, right, ..
            } => {
                if start + *len == end {
                    return self.compose_lazy(op);
                }
                let mid = left.len();
                if end <= mid {
                    left.operate_inner(start, end, op);
                } else if mid <= start {
                    right.operate_inner(start - mid, end - mid, op);
                } else {
                    left.operate_inner(start, mid, op);
                    right.operate_inner(0, end - mid, op);
                }
            }
        }
    }
    /// `lst[range].iter().fold(M::id(), |a, b| M::prod(&a, b))`
    pub fn prod_range(&mut self, range: impl RangeBounds<usize>) -> T::Item {
        let Range { start, end } = self.range_from(range);
        if start == end {
            return T::id();
        }
        self.prod_range_inner(start, end).clone()
    }
    fn prod_range_inner(&mut self, start: usize, end: usize) -> T::Item {
        self.propagate();
        match self {
            Self::Leaf { val } => val.clone(),
            Self::Node {
                len,
                prod,
                left,
                right,
                ..
            } => {
                if start + *len == end {
                    return prod.clone();
                }
                let mid = left.len();
                if end <= mid {
                    left.prod_range_inner(start, end)
                } else if mid <= start {
                    right.prod_range_inner(start - mid, end - mid)
                } else {
                    T::prod(
                        &left.prod_range_inner(start, mid),
                        &right.prod_range_inner(0, end - mid),
                    )
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

impl<K: LazySegTreeType> FromIterator<K::Item> for LazySegTree<K> {
    fn from_iter<I: IntoIterator<Item = K::Item>>(iter: I) -> Self {
        Self::from(&iter.into_iter().collect::<Vec<_>>()[..])
    }
}
