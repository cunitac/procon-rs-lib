pub trait Permutatable {
    type Item: Clone + Ord;
    /// 更新したら `true`、もう最後の順列なら `false`
    fn next_permutation(&mut self) -> bool;
    /// 更新したら `true`、もう最初の順列なら `false`
    fn prev_permutation(&mut self) -> bool;
    /// 自らを含み、辞書順でそれ以降の順列を全列挙する
    fn permutations_after(&self) -> Permutations<Self::Item>;
    /// 昇順ソートしてから `permutations_after`
    fn permutations(&self) -> Permutations<Self::Item>;
    /// 自らを含み、辞書順でそれ以前の順列を全列挙する
    fn permutations_before(&self) -> PermutationsRev<Self::Item>;
    /// 降順ソートしてから `permutations_before`
    fn permutations_rev(&self) -> PermutationsRev<Self::Item>;
}

impl<T: Ord + Clone> Permutatable for [T] {
    type Item = T;
    fn next_permutation(&mut self) -> bool {
        let i = match self.windows(2).rposition(|v| v[0] < v[1]) {
            Some(i) => i,
            None => return false,
        };
        let j = (i + 1..self.len()).rfind(|&j| self[j] > self[i]).unwrap();
        self.swap(i, j);
        self[i + 1..].reverse();
        true
    }
    fn prev_permutation(&mut self) -> bool {
        let i = match self.windows(2).rposition(|v| v[0] > v[1]) {
            Some(i) => i,
            None => return false,
        };
        self[i + 1..].reverse();
        let j = (i + 1..self.len())
            .rfind(|&j| self[j - 1] >= self[i])
            .unwrap();
        self.swap(i, j);
        true
    }
    fn permutations_after(&self) -> Permutations<Self::Item> {
        Permutations {
            seq: self.to_vec(),
            first: true,
        }
    }
    fn permutations(&self) -> Permutations<Self::Item> {
        let mut seq = self.to_vec();
        seq.sort();
        Permutations { seq, first: true }
    }
    fn permutations_before(&self) -> PermutationsRev<Self::Item> {
        PermutationsRev {
            seq: self.to_vec(),
            first: true,
        }
    }
    fn permutations_rev(&self) -> PermutationsRev<Self::Item> {
        let mut seq = self.to_vec();
        seq.sort_by(|a, b| b.cmp(a));
        PermutationsRev { seq, first: true }
    }
}

pub struct Permutations<T> {
    seq: Vec<T>,
    first: bool,
}
impl<T: Ord + Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        if self.first || self.seq.next_permutation() {
            self.first = false;
            Some(self.seq.clone())
        } else {
            None
        }
    }
}

pub struct PermutationsRev<T> {
    seq: Vec<T>,
    first: bool,
}
impl<T: Ord + Clone> Iterator for PermutationsRev<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        if self.first || self.seq.prev_permutation() {
            self.first = false;
            Some(self.seq.clone())
        } else {
            None
        }
    }
}
