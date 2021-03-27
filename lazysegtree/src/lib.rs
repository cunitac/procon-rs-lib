use std::ops::{Bound, Range, RangeBounds};

pub trait Type {
    type Item: Clone;
    type Operator: Clone;
    fn id() -> Self::Item;
    fn prod(a: &Self::Item, b: &Self::Item) -> Self::Item;
    fn composition(a: &Self::Operator, b: &Self::Operator) -> Self::Operator;
    fn operate_with_len(val: &mut Self::Item, op: &Self::Operator, len: usize);
    /// 長さ `1` と見なす．
    fn operate(val: &mut Self::Item, op: &Self::Operator) {
        Self::operate_with_len(val, op, 1)
    }
}

pub struct LazySegTree<T: Type> {
    len: usize,
    root: Node<T>,
}

impl<T: Type> From<&[T::Item]> for LazySegTree<T> {
    fn from(slice: &[T::Item]) -> Self {
        Self {
            len: slice.len(),
            root: Node::from(slice),
        }
    }
}

impl<T: Type> LazySegTree<T> {
    pub fn new(n: usize) -> Self {
        Self::from(&vec![T::id(); n][..])
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn prod(&mut self) -> &T::Item {
        self.root.prod()
    }
    pub fn prod_range(&mut self, range: impl RangeBounds<usize>) -> T::Item {
        let Range { start, end } = self.range_from(range);
        self.root.prod_range(start, end)
    }
    pub fn get(&mut self, i: usize) -> &T::Item {
        assert!(i < self.len(), "index out: {}/{}", i, self.len());
        self.root.get(i)
    }
    pub fn set(&mut self, i: usize, val: T::Item) {
        assert!(i < self.len(), "index out: {}/{}", i, self.len());
        self.root.set(i, val);
    }
    pub fn operate(&mut self, range: impl RangeBounds<usize>, op: &T::Operator) {
        let Range { start, end } = self.range_from(range);
        self.root.operate(start, end, op)
    }
    fn range_from(&self, range: impl RangeBounds<usize>) -> Range<usize> {
        use Bound::*;
        let start = match range.start_bound() {
            Included(&start) => start,
            Excluded(&start) => start + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Excluded(&end) => end,
            Included(&end) => end + 1,
            Unbounded => self.len,
        };
        assert!(start <= end, "invalid range: {}..{}", start, end);
        assert!(end <= self.len, "index out of bounds: {}/{}", end, self.len);
        Range { start, end }
    }
}

enum Node<T: Type> {
    Leaf {
        val: T::Item,
    },
    Span {
        len: usize,
        prod: T::Item,
        lazy: Option<T::Operator>,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<T: Type> From<&[T::Item]> for Node<T> {
    fn from(slice: &[T::Item]) -> Self {
        if slice.len() == 1 {
            Self::Leaf { val: slice[0].clone() }
        } else {
            let mid = slice.len() / 2;
            let left = Self::from(&slice[..mid]);
            let right = Self::from(&slice[mid..]);
            Self::Span {
                len: slice.len(),
                prod: T::id(),
                lazy: None,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
    }
}

impl<T: Type> Node<T> {
    fn propagate(&mut self) {
        if let Self::Span { prod, len, lazy, left, right } = self {
            if let Some(lazy) = lazy.take() {
                T::operate_with_len(prod, &lazy, *len);
                left.compose_lazy(&lazy);
                right.compose_lazy(&lazy);
            }
        }
    }
    fn compose_lazy(&mut self, op: &T::Operator) {
        match self {
            Self::Leaf { val } => T::operate(val, op),
            Self::Span { lazy: Some(lazy), .. } => *lazy = T::composition(lazy, op),
            Self::Span { lazy, .. } => *lazy = Some(op.clone()),
        }
    }
    fn prod(&mut self) -> &T::Item {
        self.propagate();
        match self {
            Self::Leaf { val } => val,
            Self::Span { prod, .. } => prod,
        }
    }
    fn len(&self) -> usize {
        match self {
            Self::Leaf { .. } => 1,
            Self::Span { len, .. } => *len,
        }
    }
    fn get(&mut self, i: usize) -> &T::Item {
        self.propagate();
        match self {
            Self::Leaf { val } => val,
            Self::Span { left, right, .. } => {
                let mid = left.len();
                if i < mid {
                    left.get(i)
                } else {
                    right.get(i - mid)
                }
            }
        }
    }
    fn set(&mut self, i: usize, v: T::Item) {
        self.propagate();
        match self {
            Self::Leaf { val } => *val = v,
            Self::Span { prod, left, right, .. } => {
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
    fn operate(&mut self, start: usize, end: usize, op: &T::Operator) {
        self.propagate();
        match self {
            Self::Leaf { val } => T::operate(val, op),
            Self::Span { len, left, right, .. } => {
                if (start, end) == (0, *len) {
                    return self.compose_lazy(op);
                }
                let mid = left.len();
                if end <= mid {
                    left.operate(start, end, op);
                } else if mid <= start {
                    right.operate(start - mid, end - mid, op);
                } else {
                    left.operate(start, mid, op);
                    right.operate(0, end - mid, op);
                }
            }
        }
    }
    fn prod_range(&mut self, start: usize, end: usize) -> T::Item {
        self.propagate();
        match self {
            Self::Leaf { val } => val.clone(),
            Self::Span { len, prod, left, right, .. } => {
                if (start, end) == (0, *len) {
                    return prod.clone();
                }
                let mid = left.len();
                if end <= mid {
                    left.prod_range(start, end)
                } else if mid <= start {
                    right.prod_range(start - mid, end - mid)
                } else {
                    T::prod(&left.prod_range(start, mid), &right.prod_range(0, end - mid))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        pub enum SumAdd {}
        impl Type for SumAdd {
            type Item = u32;
            type Operator = u32;
            fn id() -> u32 {
                0
            }
            fn prod(a: &u32, b: &u32) -> u32 {
                a + b
            }
            fn composition(a: &u32, b: &u32) -> u32 {
                a + b
            }
            fn operate_with_len(val: &mut u32, op: &u32, len: usize) {
                *val += op * len as u32;
            }
        }
        let mut lst = LazySegTree::<SumAdd>::new(4);
        lst.operate(..3, &2);
        dbg!((0..4).map(|i| *lst.get(i)).collect::<Vec<_>>());
        dbg!((0..4).map(|i| *lst.get(i)).collect::<Vec<_>>());
        assert_eq!(lst.prod_range(..3), 6);
        assert_eq!(lst.get(0), &2);
    }
}
