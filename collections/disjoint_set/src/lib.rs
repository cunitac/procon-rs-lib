pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_groups: usize,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: (0..n).map(|_| 1).collect(),
            num_groups: n,
        }
    }
    pub fn root_of(&mut self, mut a: usize) -> usize {
        while self.parent[a] != a {
            self.parent[a] = self.parent[self.parent[a]];
            a = self.parent[a];
        }
        a
    }
    pub fn size_of(&mut self, a: usize) -> usize {
        let a = self.root_of(a);
        self.size[a]
    }
    /// return if merged
    pub fn merge(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.root_of(a), self.root_of(b));
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.num_groups -= 1;
        true
    }
}

#[cfg(test)]
mod tests {}
