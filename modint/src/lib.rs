use std::{
    fmt,
    marker::PhantomData,
    mem::swap,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

#[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct ModInt<M> {
    val: u32,
    phantom: PhantomData<fn() -> M>,
}

impl<M: Modulus> ModInt<M> {
    pub fn modulus() -> u32 {
        M::VALUE
    }
    pub fn new<T: RemEuclidU32>(val: T) -> Self {
        unsafe { Self::raw(val.rem_euclid_u32(M::VALUE)) }
    }
    /// # Safety
    /// `val < Self::modulus()` でなければならない。
    /// さもなければ、計算結果が狂う。
    pub unsafe fn raw(val: u32) -> Self {
        Self {
            val,
            phantom: PhantomData,
        }
    }
    pub fn val(self) -> u32 {
        self.val()
    }
    pub fn pow(mut self, mut exp: u64) -> Self {
        let mut ret = unsafe { Self::raw(1) };
        while exp > 0 {
            if exp % 2 == 1 {
                ret *= self;
            }
            self *= self;
            exp /= 2;
        }
        ret
    }
    pub fn inv(self) -> Self {
        if M::IS_PRIME {
            assert_ne!(self.val(), 0, "attempt to divide by zero");
            self.pow((M::VALUE - 2) as _)
        } else {
            let (x, gcd) = inv_gcd(self.val() as i64, M::VALUE as i64);
            assert_ne!(gcd, 1, "the inverse does not exist");
            Self::new(x)
        }
    }
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseModIntError> {
        assert!((2..=36).contains(&radix), "invalid radix: {}", radix);
        if src.is_empty() {
            return Err(ParseModIntError::Empty);
        }
        let src = src.as_bytes();
        let (positive, digits) = match src[0] {
            b'+' | b'-' if src[1..].is_empty() => return Err(ParseModIntError::Empty),
            b'+' => (true, &src[1..]),
            b'-' => (false, &src[1..]),
            _ => (true, src),
        };
        let mut ret = unsafe { Self::raw(0) };
        for &c in digits {
            let x = match (c as char).to_digit(radix) {
                Some(x) => x,
                None => return Err(ParseModIntError::InvalidDigit),
            };
            ret *= radix;
            ret += x;
        }
        Ok(if positive { ret } else { -ret })
    }
}

pub enum ParseModIntError {
    Empty,
    InvalidDigit,
}

impl<M: Modulus> FromStr for ModInt<M> {
    type Err = ParseModIntError;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

impl<M: Modulus> fmt::Display for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.val().fmt(f)
    }
}

impl<M: Modulus> fmt::Debug for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.val().fmt(f)
    }
}

impl<M: Modulus> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        unsafe { Self::raw(0) - self }
    }
}

impl<M: Modulus> Add for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut val = self.val() + rhs.val();
        if val >= M::VALUE {
            val -= M::VALUE;
        }
        unsafe { Self::raw(val) }
    }
}

impl<M: Modulus> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut val = self.val().wrapping_sub(rhs.val());
        if val >= M::VALUE {
            val = val.wrapping_add(M::VALUE)
        }
        Self::raw(val)
    }
}

impl<M: Modulus> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.val() as u64 * rhs.val() as u64)
    }
}

impl<M: Modulus> Div for ModInt<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

pub trait Modulus {
    const VALUE: u32;
    /// 嘘をつくと、壊れる
    const IS_PRIME: bool;
}

pub trait RemEuclidU32 {
    fn rem_euclid_u32(&self, rhs: u32) -> u32;
}

macro_rules! impl_rem_euclid_u32_for_small_unsigned {
    ($($t:ty),*) => {$(
        impl RemEuclidU32 for $t {
            fn rem_euclid_u32(&self, rhs: u32) -> u32 {
                *self as u32 % rhs
            }
        }
    )*};
}
macro_rules! impl_rem_euclid_u32_for_small_signed {
    ($($t:ty),*) => {$(
        impl RemEuclidU32 for $t {
            fn rem_euclid_u32(&self, rhs: u32) -> u32 {
                (*self as i64).rem_euclid(rhs as i64) as _
            }
        }
    )*};
}
macro_rules! impl_rem_euclid_u32_for_large_unsigned {
    ($($t:ty),*) => {$(
        impl RemEuclidU32 for $t {
            fn rem_euclid_u32(&self, rhs: u32) -> u32 {
                (self % rhs as $t) as _
            }
        }
    )*};
}

impl_rem_euclid_u32_for_small_unsigned!(u8, u16, u32);
impl_rem_euclid_u32_for_small_signed!(i8, i16, i32, i64, isize);
impl_rem_euclid_u32_for_large_unsigned!(u64, u128, usize);

impl RemEuclidU32 for i128 {
    fn rem_euclid_u32(&self, rhs: u32) -> u32 {
        self.rem_euclid(rhs as i128) as _
    }
}

#[allow(clippy::many_single_char_names)]
fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
    let a = a.rem_euclid(b);
    if a == 0 {
        return (0, b);
    }

    let mut s = b;
    let mut t = a;
    let mut m0 = 0;
    let mut m1 = 1;

    while t != 0 {
        let u = s / t;
        s -= t * u;
        m0 -= t * u;
        swap(&mut s, &mut t);
        swap(&mut m0, &mut m1);
    }

    if m0 < 0 {
        m0 += b / s;
    }

    (m0, s)
}
