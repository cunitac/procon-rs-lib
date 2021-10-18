/// 辺集合 edge として木があるとき、(i, j) or (j, i) in edge について
/// - dp[i][j]: T = h( f{ g(dp[j][k], k) | k in c[i][j]) },  i)
/// - c[i][j] = i を根とした j の子
/// - (T, f, id) は可換モノイド
/// としたとき、ret[i] = dp[i][i] なる ret を返す
pub fn rerooting_dp<T, U, F, G, H>(edge: &[(usize, usize)], id: U, f: F, g: G, h: H) -> Vec<T>
where
    T: Clone + Default,
    U: Clone + Default,
    F: Fn(&U, &U) -> U,
    G: Fn(&T, usize) -> U,
    H: Fn(U, usize) -> T,
{
    let n = edge.len() + 1;
    let mut adj = vec![vec![]; n];
    let mut ix_for_adj = vec![vec![]; n];
    for &(i, j) in edge {
        ix_for_adj[i].push(adj[j].len());
        ix_for_adj[j].push(adj[i].len());
        adj[i].push(j);
        adj[j].push(i);
    }
    let mut order = Vec::with_capacity(n);
    let mut parent = vec![0; n];
    let mut ix_for_parent = vec![0; n];
    let mut stack = vec![0];
    while let Some(i) = stack.pop() {
        order.push(i);
        for (ix, &j) in adj[i].iter().enumerate() {
            if j != parent[i] {
                stack.push(j);
                parent[j] = i;
                ix_for_parent[j] = ix;
            }
        }
    }
    let mut dp: Vec<_> = adj.iter().map(|a| vec![T::default(); a.len()]).collect();
    for &i in order[1..].iter().rev() {
        let mut prod = id.clone();
        for (ix, &j) in adj[i].iter().enumerate().filter(|&(_, &j)| j != parent[i]) {
            prod = f(&prod, &g(&dp[i][ix], j));
        }
        dp[parent[i]][ix_for_parent[i]] = h(prod, parent[i]);
    }
    let mut ret = vec![T::default(); n];
    for &i in order.iter() {
        let mut accum_back = vec![U::default(); adj[i].len()];
        *accum_back.last_mut().unwrap() = id.clone();
        for j in (1..accum_back.len()).rev() {
            accum_back[j - 1] = f(&g(&dp[i][j], adj[i][j]), &accum_back[j]);
        }
        let mut accum = id.clone();
        for (ix, &j) in adj[i].iter().enumerate() {
            let prod = f(&accum, &accum_back[ix]);
            dp[j][ix_for_adj[i][ix]] = h(prod, j);
            accum = f(&accum, &g(&dp[i][ix], j));
        }
        ret[i] = h(accum, i);
    }
    ret
}
