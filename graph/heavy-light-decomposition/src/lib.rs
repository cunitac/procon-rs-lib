pub struct HeavyLightDecomposition {
    nodes: Vec<Node>,
}
#[derive(Clone, Default)]
struct Node {
    ord: usize,
    subtree_size: usize,
    depth: usize,
    adj: Vec<usize>,
    parent: Option<usize>,
    leader: usize,
}
impl HeavyLightDecomposition {
    pub fn new(adj: Vec<Vec<usize>>, root: usize) -> Self {
        let mut nodes = vec![Node::default(); adj.len()];
        nodes.iter_mut().zip(adj).for_each(|(n, a)| n.adj = a);
        let mut ret = Self { nodes };
        ret.dfs1(root, 0);
        ret.dfs2(root, &mut 0);
        ret
    }
    pub fn depth(&self, v: usize) -> usize {
        self.nodes[v].depth
    }
    pub fn adj(&self, v: usize) -> &Vec<usize> {
        &self.nodes[v].adj
    }
    pub fn parent(&self, v: usize) -> Option<usize> {
        self.nodes[v].parent
    }
    pub fn ord(&self, v: usize) -> usize {
        self.nodes[v].ord
    }
    pub fn subtree_size(&self, v: usize) -> usize {
        self.nodes[v].subtree_size
    }
    pub fn subtree_range(&self, v: usize) -> std::ops::Range<usize> {
        self.ord(v)..self.ord(v) + self.subtree_size(v)
    }
    pub fn leader(&self, v: usize) -> usize {
        self.nodes[v].leader
    }
    /// 最小共通祖先
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        while self.leader(a) != self.leader(b) {
            let (la, lb) = (self.leader(a), self.leader(b));
            if self.depth(la) > self.depth(lb) {
                a = self.parent(la).unwrap();
            } else {
                b = self.parent(lb).unwrap();
            }
        }
        if self.depth(a) < self.depth(b) {
            a
        } else {
            b
        }
    }
    /// - `adj` の先頭を Heavy Edge にする
    /// - `depth` と `parent` を設定する
    /// - `parent` へ向かう辺を削除する
    fn dfs1(&mut self, v: usize, depth: usize) {
        self.nodes[v].depth = depth;
        self.nodes[v].subtree_size = 1;
        if let Some(pi) = self.adj(v).iter().position(|&u| self.parent(v) == Some(u)) {
            self.nodes[v].adj.swap_remove(pi);
        }
        for i in 0..self.adj(v).len() {
            let u = self.adj(v)[i];
            self.nodes[u].parent = Some(v);
            self.dfs1(u, depth + 1);
            self.nodes[v].subtree_size += self.subtree_size(u);
            if self.subtree_size(u) > self.subtree_size(self.adj(v)[0]) {
                self.nodes[v].adj.swap(0, i);
            }
        }
    }
    /// `ord` と `leader` を設定する
    fn dfs2(&mut self, v: usize, ord: &mut usize) {
        self.nodes[v].ord = *ord;
        *ord += 1;
        for i in 0..self.adj(v).len() {
            let u = self.adj(v)[i];
            self.nodes[u].parent = Some(v);
            self.nodes[u].leader = if i == 0 { self.leader(v) } else { u };
            self.dfs2(u, ord);
        }
    }
}
