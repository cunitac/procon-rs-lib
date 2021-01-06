pub fn product<I, J>(i: I, j: J) -> Product<I, J>
where
    I: Iterator,
    J: Iterator + Clone,
    I::Item: Clone,
    J::Item: Clone,
{
    Product::new(i, j)
}

pub struct Product<I: Iterator, J> {
    i: I,
    j_orig: J,
    i_now: Option<I::Item>,
    j: J,
}
impl<I: Iterator, J: Clone> Product<I, J> {
    pub fn new(mut i: I, j: J) -> Self {
        Self {
            i_now: i.next(),
            i,
            j_orig: j.clone(),
            j,
        }
    }
}
impl<I, J> Iterator for Product<I, J>
where
    I: Iterator,
    J: Iterator + Clone,
    I::Item: Clone,
    J::Item: Clone,
{
    type Item = (I::Item, J::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.i_now.as_ref()?;
        match self.j.next() {
            Some(j_next) => Some((self.i_now.clone().unwrap(), j_next)),
            None => {
                self.i_now = self.i.next();
                self.j = self.j_orig.clone();
                self.next()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_product() {
        assert_eq!(
            product(0..3, 1..3).collect::<Vec<_>>(),
            vec![(0, 1), (0, 2), (1, 1), (1, 2), (2, 1), (2, 2)]
        );
    }
}
