//! 素因数分解

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
            let mut j = i;
            while j <= max {
                if min_factor[j] == 0 {
                    min_factor[j] = i;
                }
                j += i;
            }
        }
        Self { min_factor }
    }
    /// 素因数分解できる最大の数
    pub fn upper_limit(&self) -> usize {
        self.min_factor.len() - 1
    }
    /// 素因数と指数の組を，素因数の昇順に見る
    pub fn factor_pairs(&self, val: usize) -> FactorPairs {
        FactorPairs {
            min_factor: &self.min_factor,
            val,
        }
    }
}

pub struct FactorPairs<'a> {
    min_factor: &'a [usize],
    val: usize,
}

impl Iterator for FactorPairs<'_> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.val == 1 {
            return None;
        }
        let mut cnt = 0;
        let mf = self.min_factor[self.val];
        while self.min_factor[self.val] == mf {
            self.val /= mf;
            cnt += 1;
        }
        Some((mf, cnt))
    }
}

#[test]
fn test_factor_pairs() {
    let sieve = Sieve::new(200);
    let factor: Vec<_> = sieve.factor_pairs(200).collect();
    assert_eq!(factor, vec![(2, 3), (5, 2)])
}
