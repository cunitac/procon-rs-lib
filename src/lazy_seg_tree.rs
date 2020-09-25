use super::algebra::{Action, Monoid};
use super::util::range_from;
use std::marker::PhantomData;
use std::ops::{Range, RangeBounds};

type BoxPair<T> = Box<(T, T)>;

/// 便利な列 `lst`
pub struct LazySegTree<M: Monoid, O: Monoid, A> {
    len: usize,
    val: M::Item,
    lazy: O::Item,
    child: Option<BoxPair<LazySegTree<M, O, A>>>,
    phantom: PhantomData<A>,
}

impl<M: Monoid, O: Monoid, A> LazySegTree<M, O, A>
where
    A: Action<Item = M::Item, Operator = O::Item>,
{
    /// `[M::id(); n]`
    pub fn new(n: usize) -> Self {
        Self::from(&vec![M::id(); n][..])
    }
    fn propagate(&mut self) {
        A::act(&mut self.val, &self.lazy);
        if let Some(child) = self.child.as_mut() {
            let (left, right) = child.as_mut();
            O::op_from_left(&self.lazy, &mut left.lazy);
            O::op_from_left(&self.lazy, &mut right.lazy);
        }
        self.lazy = O::id();
    }
    /// `lst[i]`
    pub fn get(&mut self, i: usize) -> &M::Item {
        assert!(i < self.len, "index out: {}/{}", i, self.len);
        self.propagate();
        if self.len == 1 {
            return &self.val;
        }
        let mid = self.len / 2;
        let (left, right) = self.child.as_mut().unwrap().as_mut();
        if i < mid {
            left.get(i)
        } else {
            right.get(i - mid)
        }
    }
    /// `lst[i] = v`
    pub fn set(&mut self, i: usize, v: M::Item) {
        assert!(i < self.len, "index out: {}/{}", i, self.len);
        self.propagate();
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
    /// `range.for_each(|i| A::act(&mut lst[i], op)`
    pub fn act(&mut self, range: impl RangeBounds<usize>, op: O::Item) {
        let Range { start, end } = range_from(range, self.len);
        //self.act_inner(start, end, op);
    }
}

impl<M: Monoid, O: Monoid, A> From<&[M::Item]> for LazySegTree<M, O, A>
where
    A: Action<Item = M::Item, Operator = O::Item>,
{
    fn from(slice: &[M::Item]) -> Self {
        if slice.len() == 1 {
            return Self {
                len: 1,
                val: slice[0].clone(),
                lazy: O::id(),
                child: None,
                phantom: PhantomData,
            };
        }
        let mid = slice.len() / 2;
        let left = Self::from(&slice[..mid]);
        let right = Self::from(&slice[mid..]);
        Self {
            len: slice.len(),
            val: M::prod(&left.val, &right.val),
            lazy: O::id(),
            child: Some(Box::new((left, right))),
            phantom: PhantomData,
        }
    }
}
