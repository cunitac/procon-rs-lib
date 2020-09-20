//! For the modular arithmetic.

use num::{
    self,
    integer::Integer,
    traits::{Inv, Num, One, Pow, Zero},
};
use std::{
    cell::RefCell,
    convert::Infallible,
    fmt,
    iter::{Product, Sum},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
    thread::LocalKey,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ModInt<M>(u32, PhantomData<fn() -> M>);

impl<M: Modulus> ModInt<M> {
    pub fn new<V: ModU32>(val: V) -> Self {
        val.into()
    }
    pub fn modulus() -> u32 {
        M::modulus()
    }
    pub fn val(self) -> u32 {
        self.0
    }
    /// # Safety
    /// `val` must be less than `modulus`
    pub unsafe fn raw(val: u32) -> Self {
        Self(val, PhantomData)
    }
}

impl<M: Modulus> Default for ModInt<M> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<M: Modulus, V: ModU32> From<V> for ModInt<M> {
    fn from(val: V) -> Self {
        unsafe { Self::raw(val.mod_u32(Self::modulus())) }
    }
}

impl<M: Modulus> FromStr for ModInt<M> {
    type Err = Infallible;
    fn from_str(src: &str) -> Result<Self, Infallible> {
        Self::from_str_radix(src, 10)
    }
}

impl<M: Modulus> fmt::Display for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.val(), f)
    }
}
impl<M: Modulus> fmt::Debug for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.val(), f)
    }
}

impl<M: Modulus> Zero for ModInt<M> {
    fn zero() -> Self {
        unsafe { Self::raw(0) }
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
impl<M: Modulus> One for ModInt<M> {
    fn one() -> Self {
        unsafe { Self::raw(1) }
    }
}

impl<M: Modulus> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::zero() - self
    }
}
impl<M: Modulus> Inv for ModInt<M> {
    type Output = Self;
    fn inv(self) -> Self {
        assert_ne!(self.0, 0, "attempt to get inverse of zero");
        let e = i64::extended_gcd(&self.0.into(), &M::modulus().into());
        assert_ne!(e.gcd, 1, "multiplicative inverse does not exist");
        Self::from(e.x)
    }
}

impl<M: Modulus> Add for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let m = M::modulus();
        let val = self.0 + rhs.0;
        unsafe { Self::raw(if val >= m { val - m } else { val }) }
    }
}
impl<M: Modulus> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let m = M::modulus();
        let val = self.0.wrapping_sub(rhs.0);
        unsafe { Self::raw(if val >= m { val.wrapping_add(m) } else { val }) }
    }
}
impl<M: Modulus> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        unsafe { Self::raw((self.0 as u64 * rhs.0 as u64 % M::modulus() as u64) as u32) }
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: Modulus> Div for ModInt<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}
/// always return zero, unless `rhs == 0`.
impl<M: Modulus> Rem for ModInt<M> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        assert_ne!(rhs.0, 0, "attempt to divide by zero");
        Self::zero()
    }
}
macro_rules! forward_ref_binop {
    ($trait:ident, $op:ident) => {
        impl<'a, M: Modulus> $trait<ModInt<M>> for &'a ModInt<M> {
            type Output = ModInt<M>;
            fn $op(self, rhs: ModInt<M>) -> ModInt<M> {
                (*self).$op(rhs)
            }
        }
        impl<M: Modulus> $trait<&ModInt<M>> for ModInt<M> {
            type Output = ModInt<M>;
            fn $op(self, rhs: &ModInt<M>) -> ModInt<M> {
                self.$op(*rhs)
            }
        }
        impl<M: Modulus> $trait<&ModInt<M>> for &ModInt<M> {
            type Output = ModInt<M>;
            fn $op(self, rhs: &ModInt<M>) -> ModInt<M> {
                self.$op(*rhs)
            }
        }
    };
}
forward_ref_binop!(Add, add);
forward_ref_binop!(Sub, sub);
forward_ref_binop!(Mul, mul);
forward_ref_binop!(Div, div);
forward_ref_binop!(Rem, rem);

macro_rules! binop_assign {
    ($assign_trait:ident, $assign:ident, $op:ident) => {
        impl<M: Modulus> $assign_trait for ModInt<M> {
            fn $assign(&mut self, rhs: Self) {
                *self = self.$op(rhs)
            }
        }
    };
}
binop_assign!(AddAssign, add_assign, add);
binop_assign!(SubAssign, sub_assign, sub);
binop_assign!(MulAssign, mul_assign, mul);
binop_assign!(DivAssign, div_assign, div);
binop_assign!(RemAssign, rem_assign, rem);

macro_rules! forward_ref_op_assign {
    ($trait:ident, $op:ident) => {
        impl<M: Modulus> $trait<&ModInt<M>> for ModInt<M> {
            #[inline]
            fn $op(&mut self, rhs: &ModInt<M>) {
                self.$op(*rhs)
            }
        }
    };
}
forward_ref_op_assign!(AddAssign, add_assign);
forward_ref_op_assign!(SubAssign, sub_assign);
forward_ref_op_assign!(MulAssign, mul_assign);
forward_ref_op_assign!(DivAssign, div_assign);
forward_ref_op_assign!(RemAssign, rem_assign);

impl<M: Modulus> Num for ModInt<M> {
    type FromStrRadixErr = Infallible;
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, Infallible> {
        assert!(2 <= radix && radix <= 36, "radix must be in `[2, 36]`");
        assert!(!src.is_empty(), "attempt to parse empty str");
        let src = src.as_bytes();
        let (positive, digits) = match src[0] {
            b'+' => (true, &src[1..]),
            b'-' => (false, &src[1..]),
            _ => (true, src),
        };
        assert!(!digits.is_empty(), "attempt to parse sign");
        let mut result = 0_u64;
        for &c in digits {
            let x = (c as char).to_digit(radix).expect("found invalid char") as u64;
            result = (result * 10 + x) % M::modulus() as u64;
        }
        let ret = unsafe { Self::raw(result as u32) };
        Ok(if positive { ret } else { -ret })
    }
}

impl<M: Modulus> Pow<u64> for ModInt<M> {
    type Output = Self;
    fn pow(mut self, mut exp: u64) -> Self {
        let mut acc = Self::one();
        while exp > 0 {
            if exp & 1 == 1 {
                acc *= self;
            }
            self *= self;
            exp >>= 1;
        }
        acc
    }
}

impl<M: Modulus> Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}
impl<'a, M: Modulus> Sum<&'a Self> for ModInt<M> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + *x)
    }
}
impl<M: Modulus> Product for ModInt<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}
impl<'a, M: Modulus> Product<&'a Self> for ModInt<M> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + *x)
    }
}

pub trait Modulus: 'static + Copy + Eq {
    fn modulus() -> u32;
}

#[macro_export]
macro_rules! static_modulus {
    (type $type:ident : u32 = $val:expr) => {
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
        pub enum $type {}
        impl Modulus for $type {
            fn modulus() -> u32 {
                $val
            }
        }
    };
}
#[macro_export]
macro_rules! static_modint {
    (type $modint:ident = ModInt<$modulus:ident($val:expr): u32>) => {
        static_modulus!(type $modulus: u32 = $val);
        pub type $modint = ModInt<$modulus>;
    };
}
static_modint!(type ModInt998244353 = ModInt<M998244353(998244353): u32>);
static_modint!(type ModInt1000000007 = ModInt<M1000000007(1000000007): u32>);

/// default modulus is `998_244_353`
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum DynamicModulus {}
impl DynamicModulus {
    pub fn set(modulus: u32) {
        assert_ne!(modulus, 0, "modulus must not be 0");
        Self::value_key().with(|val| *val.borrow_mut() = modulus)
    }
    fn value_key() -> &'static LocalKey<RefCell<u32>> {
        thread_local!(static MOD: RefCell<u32> = RefCell::new(998_244_353));
        &MOD
    }
}
impl Modulus for DynamicModulus {
    fn modulus() -> u32 {
        Self::value_key().with(|val| *val.borrow())
    }
}

pub trait ModU32 {
    fn mod_u32(self, rhs: u32) -> u32;
}
macro_rules! mod_u32_for_small {
    ($($type:ty),*) => {$(
        impl ModU32 for $type {
            fn mod_u32(self, rhs:u32) -> u32 {
                (self as i128).rem_euclid(rhs as i128) as u32
            }
        }
    )*};
}
macro_rules! mod_u32_for_large {
    ($($type:ty),*) => {$(
        impl ModU32 for $type {
            fn mod_u32(self, rhs:u32) -> u32 {
                self.rem_euclid(rhs as $type) as u32
            }
        }
    )*};
}
mod_u32_for_small!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64);
mod_u32_for_large!(u128);
#[cfg(target_pointer_width = "32")]
mod_u32_for_small!(usize);
#[cfg(target_pointer_width = "64")]
mod_u32_for_large!(usize);
