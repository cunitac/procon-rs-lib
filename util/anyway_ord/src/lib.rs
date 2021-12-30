//! `Eq` や `Ord` を無理やり実装するラッパ型 `AnywayEq`、 `AnywayOrd` を提供する。

/// `PartialOrd` を実装している型 `T` のラッパであって、`Ord` を実装している。
#[derive(PartialEq, PartialOrd)]
pub struct AnywayOrd<T: PartialOrd>(pub T);

impl<T: PartialOrd> Eq for AnywayOrd<T> {}
#[allow(clippy::derive_ord_xor_partial_ord)]
impl<T: PartialOrd> Ord for AnywayOrd<T> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

#[derive(PartialEq)]
pub struct AnywayEq<T: PartialEq>(pub T);
impl<T: PartialEq> Eq for AnywayEq<T> {}
