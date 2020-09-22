//! 素因数分解

use std::iter;

pub struct Sieve {
    min_factor: Vec<usize>,
}

impl Sieve {
    /// `max` 以下の正整数を素因数分解できる `Sieve` をつくる．
    pub fn new(max: usize) -> Self {
        let mut min_factor = vec![0; max + 1];
        for i in 2..=max {
            if min_factor[i] != 0 {
                continue;
            }
            for j in 1..=max / i {
                min_factor[i * j] = i;
            }
        }
        Self { min_factor }
    }
    /// 素因数分解できる最大の数
    pub fn upper_limit(&self) -> usize {
        (self.min_factor.len() + 1).pow(2) - 1
    }
    /// 高速に素因数分解できる最大の数
    pub fn fast_upper_limit(&self) -> usize {
        self.min_factor.len()
    }
    /// 素因数と指数の組を，素因数の昇順に見る
    /// 最後まで見て O(lg n)
    pub fn factor_pairs(&self, mut n: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        iter::from_fn(move || {
            if n == 1 {
                return None;
            }
            let mut cnt = 0;
            let mf = self.min_factor[n];
            while self.min_factor[n] == mf {
                n /= mf;
                cnt += 1;
            }
            Some((mf, cnt))
        })
    }
}
