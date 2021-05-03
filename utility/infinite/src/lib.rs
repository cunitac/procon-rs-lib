use {
    std::{
        fmt,
        ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    },
    MaybeInf::*,
};

/// 良い感じに加減乗算や比較ができる。
/// 良い感じにできないときは panic する。
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaybeInf<T> {
    NegInf,
    Finite(T),
    PosInf,
}

impl<T> MaybeInf<T> {
    /// `Finite` でなければ panic
    pub fn unwrap(self) -> T {
        match self {
            Finite(v) => v,
            _ => panic!("not finite"),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for MaybeInf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Finite(v) => v.fmt(f),
            PosInf => f.write_str("∞"),
            NegInf => f.write_str("-∞"),
        }
    }
}

impl<T> From<MaybeInf<T>> for Option<T> {
    /// `Finite` のときに `Some`
    fn from(v: MaybeInf<T>) -> Option<T> {
        match v {
            Finite(v) => Some(v),
            _ => None,
        }
    }
}

impl<T> From<T> for MaybeInf<T> {
    fn from(v: T) -> Self {
        Finite(v)
    }
}

impl<T: Neg> Neg for MaybeInf<T> {
    type Output = MaybeInf<T::Output>;
    fn neg(self) -> Self::Output {
        match self {
            Finite(a) => Finite(-a),
            PosInf => NegInf,
            NegInf => PosInf,
        }
    }
}

impl<T: Add<U>, U> Add<MaybeInf<U>> for MaybeInf<T> {
    type Output = MaybeInf<T::Output>;
    /// # Panics
    /// - `PosInf + NegInf`
    /// - `NegInf + PosInf`
    fn add(self, rhs: MaybeInf<U>) -> Self::Output {
        match (self, rhs) {
            (Finite(a), Finite(b)) => Finite(a + b),
            (PosInf, NegInf) => panic!("PosInf + NegInf"),
            (NegInf, PosInf) => panic!("NegInf + PosInf"),
            (PosInf, _) | (_, PosInf) => PosInf,
            (NegInf, _) | (_, NegInf) => NegInf,
        }
    }
}

impl<T: AddAssign<U>, U> AddAssign<MaybeInf<U>> for MaybeInf<T> {
    /// # Panics
    /// - `PosInf += NegInf`
    /// - `NegInf += PosInf`
    fn add_assign(&mut self, rhs: MaybeInf<U>) {
        if let Finite(a) = self {
            if let Finite(b) = rhs {
                return *a += b;
            }
        }
        match (self, rhs) {
            (PosInf, NegInf) => panic!("PosInf += NegInf"),
            (NegInf, PosInf) => panic!("NegInf += PosInf"),
            _ => {}
        }
    }
}

impl<T: Sub<U>, U> Sub<MaybeInf<U>> for MaybeInf<T> {
    type Output = MaybeInf<T::Output>;
    /// # Panics
    /// - `PosInf - PosInf`
    /// - `NegInf - NegInf`
    fn sub(self, rhs: MaybeInf<U>) -> Self::Output {
        match (self, rhs) {
            (Finite(a), Finite(b)) => Finite(a - b),
            (PosInf, PosInf) => panic!("PosInf - PosInf"),
            (NegInf, NegInf) => panic!("NegInf - NegInf"),
            (PosInf, _) | (_, NegInf) => PosInf,
            (NegInf, _) | (_, PosInf) => NegInf,
        }
    }
}

impl<T: SubAssign<U>, U> SubAssign<MaybeInf<U>> for MaybeInf<T> {
    /// # Panics
    /// - `PosInf -= PosInf`
    /// - `NegInf -= NegInf`
    fn sub_assign(&mut self, rhs: MaybeInf<U>) {
        if let Finite(a) = self {
            if let Finite(b) = rhs {
                return *a -= b;
            }
        }
        match (self, rhs) {
            (PosInf, PosInf) => panic!("PosInf -= PosInf"),
            (NegInf, NegInf) => panic!("NegInf -= NegInf"),
            _ => {}
        }
    }
}

impl<T: Mul<U> + Signed, U: Signed> Mul<MaybeInf<U>> for MaybeInf<T> {
    type Output = MaybeInf<T::Output>;
    /// # Panics
    /// - `Zero * PosInf`
    /// - `Zero * NegInf`
    /// - `PosInf * Zero`
    /// - `NegInf * Zero`
    fn mul(self, rhs: MaybeInf<U>) -> Self::Output {
        use Signum::*;
        match (self, rhs) {
            (Finite(a), Finite(b)) => Finite(a * b),
            (PosInf, b) => match b.signum() {
                Positive => PosInf,
                Zero => panic!("PosInf * Zero"),
                Negative => NegInf,
            },
            (NegInf, b) => match b.signum() {
                Positive => NegInf,
                Zero => panic!("NegInf * Zero"),
                Negative => PosInf,
            },
            (a, PosInf) => match a.signum() {
                Positive => PosInf,
                Zero => panic!("Zero * PosInf"),
                Negative => NegInf,
            },
            (a, NegInf) => match a.signum() {
                Positive => NegInf,
                Zero => panic!("Zero * NegInf"),
                Negative => PosInf,
            },
        }
    }
}

impl<T: MulAssign<U> + Signed, U: Signed> MulAssign<MaybeInf<U>> for MaybeInf<T> {
    /// # Panics
    /// - `Zero *= PosInf`
    /// - `Zero *= NegInf`
    /// - `PosInf *= Zero`
    /// - `NegInf *= Zero`
    fn mul_assign(&mut self, rhs: MaybeInf<U>) {
        use Signum::*;
        match self {
            Finite(a) => match rhs {
                Finite(b) => *a *= b,
                PosInf => match a.signum() {
                    Zero => panic!("Zero *= PosInf"),
                    Positive => *self = PosInf,
                    Negative => *self = NegInf,
                },
                NegInf => match a.signum() {
                    Zero => panic!("Zero *= NegInf"),
                    Positive => *self = NegInf,
                    Negative => *self = PosInf,
                },
            },
            PosInf => match rhs.signum() {
                Zero => panic!("PosInf *= Zero"),
                Positive => {}
                Negative => *self = NegInf,
            },
            NegInf => match rhs.signum() {
                Zero => panic!("NegInf *= Zero"),
                Positive => {}
                Negative => *self = PosInf,
            },
        }
    }
}

pub trait Signed {
    fn signum(&self) -> Signum;
}
pub enum Signum {
    Positive,
    Zero,
    Negative,
}
macro_rules! impl_singed {
    ($($t:ty),*) => {$(
        impl Signed for $t {
            #[allow(clippy::float_cmp)]
            fn signum(&self) -> Signum {
                if *self == 0 as $t {
                    Signum::Zero
                } else if *self > 0 as $t {
                    Signum::Positive
                } else {
                    Signum::Negative
                }
            }
        }
    )*};
}
impl_singed!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
impl<T: Signed> Signed for MaybeInf<T> {
    fn signum(&self) -> Signum {
        match self {
            Finite(a) => a.signum(),
            PosInf => Signum::Positive,
            NegInf => Signum::Negative,
        }
    }
}

pub trait Zero {
    fn zero() -> Self;
}
macro_rules! impl_zero {
    ($($t:ty),*) => {$(
        impl Zero for $t {
            fn zero() -> $t {
                0 as $t
            }
        }
    )*};
}
impl_zero!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

#[cfg(test)]
mod tests {
    use super::MaybeInf::*;
    #[test]
    fn test_unwrap() {
        assert_eq!(Finite(1).unwrap(), 1);
    }
    #[test]
    #[should_panic]
    fn test_unwrap_panic_0() {
        let _ = PosInf::<i32>.unwrap();
    }
    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Finite(42)), "42");
        assert_eq!(format!("{:?}", PosInf::<i32>), "∞");
        assert_eq!(format!("{:?}", NegInf::<i32>), "-∞");
    }
    #[test]
    #[should_panic]
    fn test_unwrap_panic_1() {
        let _ = NegInf::<i32>.unwrap();
    }
    #[test]
    fn test_add() {
        assert_eq!(Finite(1) + Finite(2), Finite(3));
        assert_eq!(Finite(1) + PosInf::<i32>, PosInf);
        assert_eq!(PosInf::<i32> + Finite(1), PosInf);
        assert_eq!(PosInf::<i32> + PosInf::<i32>, PosInf);
        assert_eq!(Finite(1) + NegInf::<i32>, NegInf);
        assert_eq!(NegInf::<i32> + Finite(1), NegInf);
        assert_eq!(NegInf::<i32> + NegInf::<i32>, NegInf);
    }
    #[test]
    #[should_panic]
    fn test_add_panic_0() {
        let _ = PosInf::<i32> + NegInf::<i32>;
    }
    #[test]
    #[should_panic]
    fn test_add_panic_1() {
        let _ = NegInf::<i32> + PosInf::<i32>;
    }
    #[test]
    fn test_sub() {
        assert_eq!(Finite(1) - Finite(2), Finite(-1));
        assert_eq!(Finite(1) - PosInf::<i32>, NegInf);
        assert_eq!(PosInf::<i32> - Finite(1), PosInf);
        assert_eq!(PosInf::<i32> - NegInf::<i32>, PosInf);
        assert_eq!(Finite(1) - NegInf::<i32>, PosInf);
        assert_eq!(NegInf::<i32> - Finite(1), NegInf);
        assert_eq!(NegInf::<i32> - PosInf::<i32>, NegInf);
    }
    #[test]
    #[should_panic]
    #[allow(clippy::eq_op)]
    fn test_sub_panic_0() {
        let _ = PosInf::<i32> - PosInf::<i32>;
    }
    #[test]
    #[should_panic]
    #[allow(clippy::eq_op)]
    fn test_sub_panic_1() {
        let _ = NegInf::<i32> - NegInf::<i32>;
    }
    #[test]
    fn test_mul() {
        assert_eq!(Finite(1) * Finite(2), Finite(2));
        assert_eq!(Finite(1) * PosInf::<i32>, PosInf);
        assert_eq!(Finite(-1) * PosInf::<i32>, NegInf);
        assert_eq!(PosInf::<i32> * Finite(1), PosInf);
        assert_eq!(PosInf::<i32> * Finite(-1), NegInf);
        assert_eq!(PosInf::<i32> * PosInf::<i32>, PosInf);
        assert_eq!(PosInf::<i32> * NegInf::<i32>, NegInf);
        assert_eq!(Finite(1) * NegInf::<i32>, NegInf);
        assert_eq!(Finite(-1) * NegInf::<i32>, PosInf);
        assert_eq!(NegInf::<i32> * Finite(1), NegInf);
        assert_eq!(NegInf::<i32> * Finite(-1), PosInf);
        assert_eq!(NegInf::<i32> * PosInf::<i32>, NegInf);
        assert_eq!(NegInf::<i32> * NegInf::<i32>, PosInf);
    }
    #[test]
    #[should_panic]
    fn test_mul_panic_0() {
        let _ = Finite(0) * PosInf::<i32>;
    }
    #[test]
    #[should_panic]
    fn test_mul_panic_1() {
        let _ = Finite(0) * NegInf::<i32>;
    }
    #[test]
    #[should_panic]
    fn test_mul_panic_2() {
        let _ = PosInf::<i32> * Finite(0);
    }
    #[test]
    #[should_panic]
    fn test_mul_panic_3() {
        let _ = NegInf::<i32> * Finite(0);
    }
    #[test]
    fn test_neg() {
        assert_eq!(-Finite(1), Finite(-1));
        assert_eq!(-PosInf::<i32>, NegInf);
        assert_eq!(-NegInf::<i32>, PosInf);
    }
    #[test]
    fn test_ord() {
        assert!(NegInf < Finite(i32::min_value()));
        assert!(Finite(i32::min_value()) < Finite(i32::max_value()));
        assert!(Finite(i32::max_value()) < PosInf);
        assert!(NegInf::<i32> < PosInf::<i32>);
    }
}
