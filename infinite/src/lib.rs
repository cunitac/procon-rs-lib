use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use MaybeInf::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaybeInf<T> {
    NegInf,
    Finite(T),
    PosInf,
}

impl<T> MaybeInf<T> {
    /// `Finite` でなければパニック
    pub fn unwrap(self) -> T {
        match self {
            Finite(v) => v,
            _ => panic!("not finite"),
        }
    }
    pub fn unwrap_or(self, when_posinf: impl FnOnce() -> T, when_neginf: impl FnOnce() -> T) -> T {
        match self {
            Finite(v) => v,
            PosInf => when_posinf(),
            NegInf => when_neginf(),
        }
    }
    pub fn unwrap_or_posinf(self, when_posinf: impl FnOnce() -> T) -> T {
        self.unwrap_or(when_posinf, || panic!("neither Finite nor PosInf"))
    }
    pub fn unwrap_or_neginf(self, when_neginf: impl FnOnce() -> T) -> T {
        self.unwrap_or(|| panic!("neither Finite nor NegInf"), when_neginf)
    }
    /// `PosInf` か `NegInf` なら変えない
    pub fn map<U>(&self, f: impl FnOnce(&T) -> MaybeInf<U>) -> MaybeInf<U> {
        match self {
            Finite(v) => f(v),
            PosInf => PosInf,
            NegInf => NegInf,
        }
    }
    pub fn map_or<U>(
        &self,
        when_posinf: impl FnOnce() -> U,
        when_neginf: impl FnOnce() -> U,
        f: impl FnOnce(&T) -> U,
    ) -> U {
        match self {
            Finite(v) => f(v),
            PosInf => when_posinf(),
            NegInf => when_neginf(),
        }
    }
    pub fn map_or_posinf<U>(&self, when_posinf: impl FnOnce() -> U, f: impl FnOnce(&T) -> U) -> U {
        self.map_or(when_posinf, || panic!("neither Finite nor PosInf"), f)
    }
    pub fn map_or_neginf<U>(&self, when_neginf: impl FnOnce() -> U, f: impl FnOnce(&T) -> U) -> U {
        self.map_or(|| panic!("neither Finite nor NegInf"), when_neginf, f)
    }
}

impl<T> From<MaybeInf<T>> for Option<T> {
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
    fn mul(self, rhs: MaybeInf<U>) -> Self::Output {
        use Signum::*;
        match (self, rhs) {
            (Finite(a), Finite(b)) => Finite(a * b),
            (PosInf, Finite(b)) => match b.signum() {
                Positive => PosInf,
                Zero => panic!("PosInf * Zero"),
                Negative => NegInf,
            },
            (NegInf, Finite(b)) => match b.signum() {
                Positive => NegInf,
                Zero => panic!("NegInf * Zero"),
                Negative => PosInf,
            },
            (Finite(a), PosInf) => match a.signum() {
                Positive => PosInf,
                Zero => panic!("Zero * PosInf"),
                Negative => NegInf,
            },
            (Finite(a), NegInf) => match a.signum() {
                Positive => NegInf,
                Zero => panic!("Zero * NegInf"),
                Negative => PosInf,
            },
            (PosInf, PosInf) | (NegInf, NegInf) => PosInf,
            (PosInf, NegInf) | (NegInf, PosInf) => NegInf,
        }
    }
}

impl<T: MulAssign<U> + Signed, U: Signed> MulAssign<MaybeInf<U>> for MaybeInf<T> {
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
mod tests {}
