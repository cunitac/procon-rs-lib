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

        // - `adj` の先頭を Heavy Edge にする
        // - `depth` と `parent` を設定する
        // - `parent` へ向かう辺を削除する
        fn dfs1(v: usize, depth: usize, nodes: &mut Vec<Node>) {
            nodes[v].depth = depth;
            nodes[v].subtree_size = 1;
            if let Some(p) = nodes[v].parent {
                let pi = nodes[v].adj.iter().position(|&u| u == p).unwrap();
                nodes[v].adj.swap_remove(pi);
            }
            for i in 0..nodes[v].adj.len() {
                let u = nodes[v].adj[i];
                nodes[u].parent = Some(v);
                dfs1(u, depth + 1, nodes);
                nodes[v].subtree_size += nodes[u].subtree_size;
                if nodes[u].subtree_size > nodes[nodes[v].adj[0]].subtree_size {
                    nodes[v].adj.swap(0, i);
                }
            }
        }
        // `ord` と `leader` を設定する
        fn dfs2(v: usize, ord: &mut usize, nodes: &mut Vec<Node>) {
            nodes[v].ord = *ord;
            *ord += 1;
            for i in 0..nodes[v].adj.len() {
                let u = nodes[v].adj[i];
                nodes[u].parent = Some(v);
                nodes[u].leader = if i == 0 { nodes[v].leader } else { u };
                dfs2(u, ord, nodes);
            }
        }

        dfs1(root, 0, &mut nodes);
        dfs2(root, &mut 0, &mut nodes);
        Self { nodes }
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
    /// { `self.ord(u)` | `u` ∈ `v` の部分木 } に等しい `Range`
    pub fn subtree_ord_range(&self, v: usize) -> std::ops::Range<usize> {
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
}
