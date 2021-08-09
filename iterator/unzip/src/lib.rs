pub trait IterExt: Iterator {
    fn great_unzip<U: Unzip<Self::Item>>(self) -> U;
}

impl<I: Iterator> IterExt for I {
    fn great_unzip<U: Unzip<Self::Item>>(self) -> U {
        let mut ret = U::default();
        self.for_each(|x| ret.push(x));
        ret
    }
}

pub trait Unzip<T>: Default {
    fn push(&mut self, x: T);
}

macro_rules! impl_unzip {
    (($($t:ident,)*), ($($a:ident,)*), ($($s:ident,)*), ($($x:ident,)*)) => {
        impl<$($t,)* $($a,)*> Unzip<($($t,)*)> for ($($a,)*)
        where $($a: Default + Extend<$t>,)*
        {
            fn push(&mut self, ($($x,)*): ($($t,)*)) {
                let ($($s,)*) = self;
                $($s.extend(std::iter::once($x));)*
            }
        }
    };
}

impl_unzip!((), (), (), ());
impl_unzip!((T0,), (A0,), (s0,), (t0,));
impl_unzip!((T0, T1,), (A0, A1,), (s0, s1,), (t0, t1,));
impl_unzip!((T0, T1, T2,), (A0, A1, A2,), (s0, s1, s2,), (t0, t1, t2,));
impl_unzip!(
    (T0, T1, T2, T3,),
    (A0, A1, A2, A3,),
    (s0, s1, s2, s3,),
    (t0, t1, t2, t3,)
);
impl_unzip!(
    (T0, T1, T2, T3, T4,),
    (A0, A1, A2, A3, A4,),
    (s0, s1, s2, s3, s4,),
    (t0, t1, t2, t3, t4,)
);
impl_unzip!(
    (T0, T1, T2, T3, T4, T5,),
    (A0, A1, A2, A3, A4, A5,),
    (s0, s1, s2, s3, s4, s5,),
    (t0, t1, t2, t3, t4, t5,)
);
impl_unzip!(
    (T0, T1, T2, T3, T4, T5, T6,),
    (A0, A1, A2, A3, A4, A5, A6,),
    (s0, s1, s2, s3, s4, s5, s6,),
    (t0, t1, t2, t3, t4, t5, t6,)
);
impl_unzip!(
    (T0, T1, T2, T3, T4, T5, T6, T7,),
    (A0, A1, A2, A3, A4, A5, A6, A7,),
    (s0, s1, s2, s3, s4, s5, s6, s7,),
    (t0, t1, t2, t3, t4, t5, t6, t7,)
);
impl_unzip!(
    (T0, T1, T2, T3, T4, T5, T6, T7, T8,),
    (A0, A1, A2, A3, A4, A5, A6, A7, A8,),
    (s0, s1, s2, s3, s4, s5, s6, s7, s8,),
    (t0, t1, t2, t3, t4, t5, t6, t7, t8,)
);
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unzip() {
        let (a, b): (Vec<_>, Vec<_>) = [(0, 1), (1, 2), (2, 3)].iter().copied().great_unzip();
        assert_eq!(a, vec![0, 1, 2]);
        assert_eq!(b, vec![1, 2, 3]);

        let (a, b, c): (Vec<_>, Vec<_>, Vec<_>) = [(0, 1, 2), (3, 4, 5), (6, 7, 8)]
            .iter()
            .copied()
            .great_unzip();
        assert_eq!(a, vec![0, 3, 6]);
        assert_eq!(b, vec![1, 4, 7]);
        assert_eq!(c, vec![2, 5, 8]);
    }
}
