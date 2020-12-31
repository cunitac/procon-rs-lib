//! Iterator の拡張

impl<I> IterExt for I where I: Iterator {}

pub trait IterExt: Iterator {
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
    fn group_by<F>(self, f: F) -> GroupBy<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        GroupBy {
            iter: self.peekable(),
            f,
        }
    }
    fn group_by_key<K, F>(self, f: F) -> GroupByKey<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> K,
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
    fn vec(self) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        self.collect()
    }
    /// `Vec` に `collect` してから `rev`
    fn collect_rev(self) -> std::iter::Rev<std::vec::IntoIter<Self::Item>>
    where
        Self: Sized,
    {
        self.collect::<Vec<_>>().into_iter().rev()
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

pub struct GroupBy<I: Iterator, F> {
    iter: std::iter::Peekable<I>,
    f: F,
}
impl<I, F> Iterator for GroupBy<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item, &I::Item) -> bool,
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut group = vec![self.iter.next()?];
        while let Some(next) = self.iter.peek() {
            if (self.f)(&group[group.len() - 1], next) {
                group.push(self.iter.next().unwrap())
            } else {
                break;
            }
        }
        Some(group)
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

pub fn repeat_app<T, F>(first: T, f: F) -> RepeatApp<T, F>
where
    F: FnMut(&T) -> T,
{
    RepeatApp { next: first, f }
}
pub struct RepeatApp<T, F> {
    next: T,
    f: F,
}
impl<T, F> Iterator for RepeatApp<T, F>
where
    F: FnMut(&T) -> T,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let mut next = (self.f)(&self.next);
        std::mem::swap(&mut next, &mut self.next);
        Some(next)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sorted() {
        let mut vec = vec![4, 7, 7, 2, 3, 2, 6, 32, 3, 6, 7, 55, 1, 2];
        let sorted_vec = vec.iter().copied().sorted().vec();
        vec.sort();
        assert_eq!(sorted_vec, vec);
    }
    #[test]
    fn test_sorted_by() {
        let mut vec = vec![4, 7, 7, 2, 3, 2, 6, 32, 3, 6, 7, 55, 1, 2];
        let sorted_vec = vec.iter().copied().sorted_by(|a, b| b.cmp(a)).vec();
        vec.sort_by(|a, b| b.cmp(a));
        assert_eq!(sorted_vec, vec);
    }
    #[test]
    fn test_sorted_by_key() {
        let mut vec = vec![4, 7, 7, 2, 3, 2, 6, 32, 3, 6, 7, 55, 1, 2];
        let sorted_vec = vec
            .iter()
            .copied()
            .sorted_by_key(|&a| std::cmp::Reverse(a))
            .vec();
        vec.sort_by_key(|&a| std::cmp::Reverse(a));
        assert_eq!(sorted_vec, vec);
    }
    #[test]
    fn test_accumulate() {
        let cum: Vec<_> = [1, 2, 3].iter().accumulate(1, |a, b| a * b).collect();
        assert_eq!(cum, vec![1, 1, 2, 6]);
    }
    #[test]
    fn test_group_by_key() {
        let vec = vec![1_i32, 2, 2, -3, -3, 0, 1, 2, 3, -2];
        let groups = vec.iter().copied().group_by_key(|a| a.signum()).vec();
        assert_eq!(
            groups,
            vec![
                (1, vec![1, 2, 2]),
                (-1, vec![-3, -3]),
                (0, vec![0]),
                (1, vec![1, 2, 3]),
                (-1, vec![-2])
            ]
        );
    }
    #[test]
    fn test_group_by() {
        let vec = vec![1_i32, 2, 2, -3, -3, 0, 1, 2, 3, -2];
        let groups = vec.iter().copied().group_by(|a, b| a < b).vec();
        assert_eq!(
            groups,
            vec![
                vec![1, 2],
                vec![2],
                vec![-3],
                vec![-3, 0, 1, 2, 3],
                vec![-2]
            ]
        );
    }
}
