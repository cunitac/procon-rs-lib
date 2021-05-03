use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Add,
};

pub fn dijkstra<N, C, A, I>(start: N, goal: Option<N>, mut adj: A) -> HashMap<N, C>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Add<C, Output = C> + Clone,
    A: FnMut(N) -> I,
    I: IntoIterator<Item = (N, C)>,
{
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start.clone(), C::zero());
    heap.push(KeyValue(C::zero(), start));

    while let Some(KeyValue(dist_v, v)) = heap.pop() {
        if dist[&v] != dist_v {
            continue;
        }
        if goal.as_ref().map_or(false, |goal| &v == goal) {
            break;
        }
        for (u, c) in adj(v) {
            use std::collections::hash_map::Entry;
            let dist_u_new = dist_v.clone() + c;
            match dist.entry(u.clone()) {
                Entry::Occupied(mut entry) => {
                    let dist_u = entry.get_mut();
                    if *dist_u > dist_u_new {
                        *dist_u = dist_u_new;
                        heap.push(KeyValue(dist_u.clone(), u));
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(dist_u_new.clone());
                    heap.push(KeyValue(dist_u_new, u));
                }
            }
        }
    }

    dist
}

pub trait Zero {
    fn zero() -> Self;
}
macro_rules! zero {
    ($($t:ty),*) => {
        $(
            impl Zero for $t {
                fn zero() -> $t {
                    0 as $t
                }
            }
        )*
    };
}
zero!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

struct KeyValue<K, V>(K, V);
impl<K: PartialEq, V> PartialEq for KeyValue<K, V> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}
impl<K: Eq, V> Eq for KeyValue<K, V> {}
impl<K: PartialOrd, V> PartialOrd for KeyValue<K, V> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        rhs.0.partial_cmp(&self.0)
    }
}
impl<K: Ord, V> Ord for KeyValue<K, V> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        rhs.0.cmp(&self.0)
    }
}
