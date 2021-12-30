pub struct Cumsum<T, S> {
    cum: Vec<T>,
    sub: S,
}

impl<T: Clone, S: Fn(&T, &T) -> T> Cumsum<T, S> {
    /// `vec[0].clone()`: `1` 回
    /// `add(_, _)`: `n-1` 回
    pub fn new<A: Fn(&T, &T) -> T>(vec: Vec<T>, add: A, sub: S) -> Self {
        let mut cum = Vec::with_capacity(vec.len());
        cum.push(vec[0].clone());
        for i in 1..vec.len() {
            cum.push(add(&cum[i - 1], &vec[i]));
        }
        Self { cum, sub }
    }
    /// `T::clone()`: `if l==0 {1} else {0}` 回
    /// `add(_, _)`: `if l==0 {0} else {1}` 回
    pub fn sum(&self, l: usize, r: usize) -> T {
        if l == 0 {
            self.cum[r].clone()
        } else {
            (self.sub)(&self.cum[r], &self.cum[l - 1])
        }
    }
}

#[cfg(test)]
mod tests {}
