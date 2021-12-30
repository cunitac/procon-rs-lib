use std::{
    ops::{Bound, Deref, DerefMut, Index, Range, RangeBounds},
    slice::SliceIndex,
};

pub struct SegTree<T, F> {
    data: Vec<T>,
    len: usize,
    id: T,
    prod: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> SegTree<T, F> {
    pub fn new(len: usize, id: T, prod: F) -> Self {
        Self {
            data: vec![id.clone(); len * 2],
            len,
            id,
            prod,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn get(&self, i: usize) -> &T {
        &self.data[self.len + i]
    }
    pub fn get_mut(&mut self, index: usize) -> GetMut<T, F> {
        GetMut {
            index: self.len + index,
            seg: self,
        }
    }
    fn update(&mut self, i: usize) {
        if i < self.len {
            self.data[i] = (self.prod)(&self.data[2 * i], &self.data[2 * i + 1])
        }
    }
    fn update_parent(&mut self, mut i: usize) {
        i >>= 1;
        while i != 0 {
            self.update(i);
            i >>= 1;
        }
    }
    pub fn prod_range(&self, range: impl RangeBounds<usize>) -> T {
        let Range { mut start, mut end } = range_from(self.len(), range);
        start += self.len;
        end += self.len;
        let mut left = self.id.clone();
        let mut right = self.id.clone();
        while start != end {
            if start & 1 != 0 {
                left = (self.prod)(&left, &self.data[start]);
                start += 1;
            }
            if end & 1 != 0 {
                end -= 1;
                right = (self.prod)(&self.data[end], &right);
            }
            start >>= 1;
            end >>= 1;
        }
        (self.prod)(&left, &right)
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

impl<T: Clone, I: SliceIndex<[T]>, F: Fn(&T, &T) -> T> Index<I> for SegTree<T, F> {
    type Output = I::Output;
    fn index(&self, index: I) -> &I::Output {
        &self.data[self.len..][index]
    }
}

pub struct GetMut<'a, T: Clone, F: Fn(&T, &T) -> T> {
    seg: &'a mut SegTree<T, F>,
    index: usize,
}
impl<T: Clone, F: Fn(&T, &T) -> T> Deref for GetMut<'_, T, F> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.seg.data[self.index]
    }
}
impl<T: Clone, F: Fn(&T, &T) -> T> DerefMut for GetMut<'_, T, F> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.seg.data[self.index]
    }
}
impl<T: Clone, F: Fn(&T, &T) -> T> Drop for GetMut<'_, T, F> {
    fn drop(&mut self) {
        self.seg.update_parent(self.index)
    }
}
