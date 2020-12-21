//! Iterator の拡張

impl<I> IterExt for I where I: Iterator {}

pub trait IterExt: Iterator {
    /// 累積和のようなもの
    fn accumulate<B, F>(self, init: B, f: F) -> Accumulate<Self, B, F>
    where
        F: FnMut(&B, Self::Item) -> B,
        Self: Sized,
    {
        Accumulate {
            iter: self,
            next: Some(init),
            f,
        }
    }
    /// まとめる
    fn group_by_key<K, F>(self, f: F) -> GroupByKey<Self, F>
    where
        Self: Sized,
    {
        GroupByKey {
            iter: self.peekable(),
            f,
        }
    }
    fn sorted_by<F>(self, cmp: F) -> std::vec::IntoIter<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering,
    {
        let mut vec = self.collect::<Vec<_>>();
        vec.sort_by(cmp);
        vec.into_iter()
    }
    fn sorted(self) -> std::vec::IntoIter<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.sorted_by(|a, b| a.cmp(b))
    }
    fn sorted_by_key<K, F>(self, mut key: F) -> std::vec::IntoIter<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> K,
        K: Ord,
    {
        self.sorted_by(|a, b| key(a).cmp(&key(b)))
    }
    fn collect_vec(self) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        self.collect()
    }
    fn take_vec(self, len: usize) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        self.take(len).collect()
    }
}

pub struct Accumulate<I, B, F> {
    iter: I,
    next: Option<B>,
    f: F,
}
impl<I, B, F> Iterator for Accumulate<I, B, F>
where
    F: FnMut(&B, I::Item) -> B,
    I: Iterator,
{
    type Item = B;
    fn next(&mut self) -> Option<B> {
        let item = self.next.take()?;
        self.next = self.iter.next().map(|x| (self.f)(&item, x));
        Some(item)
    }
}

pub struct GroupByKey<I: Iterator, F> {
    iter: std::iter::Peekable<I>,
    f: F,
}
impl<I, F, K> Iterator for GroupByKey<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> K,
    K: PartialEq,
{
    type Item = (K, Vec<I::Item>);
    fn next(&mut self) -> Option<Self::Item> {
        let mut group = vec![self.iter.next()?];
        let key = (self.f)(&group[0]);
        while let Some(next) = self.iter.peek() {
            if (self.f)(next) == key {
                group.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }
        Some((key, group))
    }
}

#[test]
fn test_accumulate() {
    let cum: Vec<_> = [1, 2, 3].iter().accumulate(1, |a, b| a * b).collect();
    assert_eq!(cum, vec![1, 1, 2, 6]);
}
