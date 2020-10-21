use std::{
    iter::FromIterator,
    ops::{Bound, Deref, DerefMut, Index, Range, RangeBounds},
    slice::SliceIndex,
};

pub trait Type {
    type Item: Clone;
    fn id() -> Self::Item;
    fn prod(a: &Self::Item, b: &Self::Item) -> Self::Item;
}

pub struct SegTree<T: Type> {
    data: Vec<T::Item>,
    len: usize,
}

impl<T: Type> SegTree<T> {
    pub fn new(len: usize) -> Self {
        Self {
            data: vec![T::id(); len * 2],
            len,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn get(&self, i: usize) -> &T::Item {
        &self.data[self.len + i]
    }
    pub fn get_mut(&mut self, index: usize) -> GetMut<T> {
        GetMut {
            index: self.len + index,
            seg: self,
        }
    }
    pub fn range(&self, range: impl RangeBounds<usize>) -> &[T::Item] {
        let Range { start, end } = range_from(self.len(), range);
        &self.data[start + self.len..end + self.len]
    }
    pub fn range_mut(&mut self, range: impl RangeBounds<usize>) -> RangeMut<T> {
        let Range { start, end } = range_from(self.len(), range);
        RangeMut {
            range: self.len + start..self.len + end,
            seg: self,
        }
    }
    fn update(&mut self, i: usize) {
        if i < self.len {
            self.data[i] = T::prod(&self.data[2 * i], &self.data[2 * i + 1])
        }
    }
    fn update_parent(&mut self, mut i: usize) {
        i >>= 1;
        while i != 0 {
            self.update(i);
            i >>= 1;
        }
    }
    fn update_range(&mut self, Range { mut start, mut end }: Range<usize>) {
        end -= 1;
        start >>= 1;
        end >>= 1;
        while start != end {
            for i in start..end {
                self.update(i)
            }
            start >>= 1;
            end >>= 1;
        }
        self.update_parent(start)
    }
    pub fn prod_range(&self, range: impl RangeBounds<usize>) -> T::Item {
        let Range { mut start, mut end } = range_from(self.len(), range);
        start += self.len;
        end += self.len;
        let mut left = T::id();
        let mut right = T::id();
        while start != end {
            if start & 1 != 0 {
                left = T::prod(&left, &self.data[start]);
                start += 1;
            }
            if end & 1 != 0 {
                end -= 1;
                right = T::prod(&self.data[end], &right);
            }
            start >>= 1;
            end >>= 1;
        }
        T::prod(&left, &right)
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

impl<T: Type> From<Vec<T::Item>> for SegTree<T> {
    fn from(mut data: Vec<T::Item>) -> Self {
        let len = data.len();
        for i in 0..len {
            data.push(data[i].clone());
        }
        let mut ret = Self { data, len };
        for i in (1..len).rev() {
            ret.update(i);
        }
        ret
    }
}
impl<T: Type> From<&[T::Item]> for SegTree<T> {
    fn from(slice: &[T::Item]) -> Self {
        let len = slice.len();
        let data = slice.iter().chain(slice.iter()).cloned().collect();
        let mut ret = Self { data, len };
        for i in (1..len).rev() {
            ret.update(i);
        }
        ret
    }
}
impl<T: Type> FromIterator<T::Item> for SegTree<T> {
    fn from_iter<I: IntoIterator<Item = T::Item>>(iter: I) -> Self {
        Self::from(&iter.into_iter().collect::<Vec<_>>()[..])
    }
}
impl<T: Type, I: SliceIndex<[T::Item]>> Index<I> for SegTree<T> {
    type Output = I::Output;
    fn index(&self, index: I) -> &I::Output {
        &self.data[self.len..][index]
    }
}

pub struct GetMut<'a, T: Type> {
    seg: &'a mut SegTree<T>,
    index: usize,
}
impl<T: Type> Deref for GetMut<'_, T> {
    type Target = T::Item;
    fn deref(&self) -> &T::Item {
        &self.seg.data[self.index]
    }
}
impl<T: Type> DerefMut for GetMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T::Item {
        &mut self.seg.data[self.index]
    }
}
impl<T: Type> Drop for GetMut<'_, T> {
    fn drop(&mut self) {
        self.seg.update_parent(self.index)
    }
}

pub struct RangeMut<'a, T: Type> {
    seg: &'a mut SegTree<T>,
    range: Range<usize>,
}
impl<T: Type> Deref for RangeMut<'_, T> {
    type Target = [T::Item];
    fn deref(&self) -> &[T::Item] {
        &self.seg[self.range.clone()]
    }
}
impl<T: Type> DerefMut for RangeMut<'_, T> {
    fn deref_mut(&mut self) -> &mut [T::Item] {
        &mut self.seg.data[self.range.clone()]
    }
}
impl<T: Type> Drop for RangeMut<'_, T> {
    fn drop(&mut self) {
        self.seg.update_range(self.range.clone())
    }
}
