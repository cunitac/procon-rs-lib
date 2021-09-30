use acl_modint::ModIntBase;

pub struct Factorial<M> {
    pub val: Vec<M>,
    pub inv: Vec<M>,
}

impl<M: ModIntBase> Factorial<M> {
    /// (n-1)! まで
    /// 2 ≦ n < modulus
    pub fn new(n: usize) -> Self {
        let mut val = vec![M::raw(1); n];
        for i in 2..n {
            val[i] = val[i - 1] * M::raw(i as u32);
        }
        let mut inv = vec![M::raw(1); n];
        inv[n - 1] = val[n - 1].inv();
        for i in (2..n).rev() {
            inv[i] = inv[i + 1] * M::raw((i + 1) as u32);
        }
        Self { val, inv }
    }
    pub fn binom(&self, n: usize, r: usize) -> M {
        if n < r {
            M::raw(0)
        } else {
            self.val[n] * self.inv[r] * self.inv[n - r]
        }
    }
}
