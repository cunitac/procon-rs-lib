use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bits<B: BitOps = u32>(B);

impl<B: BitOps> Bits<B> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn singleton(i: usize) -> Self {
        Self(B::ONE << i)
    }
    pub fn contains(self, i: usize) -> bool {
        !(self & Self::singleton(i)).is_empty()
    }
    pub fn is_empty(self) -> bool {
        self != Self::new()
    }
    pub fn bits(&self) -> B {
        self.0
    }
    pub fn len(self) -> usize {
        self.0.count_ones()
    }
    pub fn iter(self) -> Iter<B> {
        Iter(self)
    }
    pub fn insert(&mut self, i: usize) {
        *self |= Self::singleton(i);
    }
    pub fn remove(&mut self, i: usize) {
        *self -= Self::singleton(i);
    }
    pub fn clear(&mut self) {
        self.0 = B::ZERO;
    }
    pub fn complement_with_len(self, len: usize) -> Self {
        Self(!self.0 & ((B::ONE << len) - B::ONE))
    }
    /// 降順
    pub fn subsets(self) -> impl Iterator<Item = Self> {
        std::iter::successors(Some(self), move |&(mut bits)| {
            bits.0 -= B::ONE;
            bits &= self;
            if bits.is_empty() {
                None
            } else {
                Some(bits)
            }
        })
        .chain(std::iter::once(Self::new()))
    }
}
impl<B: BitOps> Bits<B>
where
    std::ops::Range<B>: Iterator<Item = B>,
{
    pub fn all_with_len(len: usize) -> impl Iterator<Item = Self> {
        (B::ZERO..B::ONE << len).map(Self)
    }
}

pub struct Iter<B: BitOps>(Bits<B>);
impl<B: BitOps> Iterator for Iter<B> {
    type Item = usize;
    fn last(self) -> Option<usize> {
        self.max()
    }
    fn min(mut self) -> Option<usize> {
        self.next()
    }
    fn max(mut self) -> Option<usize> {
        self.next_back()
    }
    fn next(&mut self) -> Option<usize> {
        let tz = (self.0).0.trailing_zeros();
        if tz == B::BIT_SIZE {
            None
        } else {
            self.0.remove(tz);
            Some(tz)
        }
    }
}
impl<B: BitOps> DoubleEndedIterator for Iter<B> {
    fn next_back(&mut self) -> Option<usize> {
        let lz = (self.0).0.leading_zeros();
        if lz == B::BIT_SIZE {
            None
        } else {
            let ret = B::BIT_SIZE - lz - 1;
            self.0.remove(ret);
            Some(ret)
        }
    }
}

impl<B: BitOps> Default for Bits<B> {
    fn default() -> Self {
        Self(B::ZERO)
    }
}
impl<B: BitOps> Not for Bits<B> {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}
impl<B: BitOps> BitOr for Bits<B> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl<B: BitOps> BitAnd for Bits<B> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl<B: BitOps> Sub for Bits<B> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self & !rhs
    }
}
impl<B: BitOps> BitXor for Bits<B> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

macro_rules! forward_ref_binop {
    ($imp:ident, $method:ident) => {
        impl<'a, B: BitOps + Copy> $imp<Bits<B>> for &'a Bits<B> {
            type Output = <Bits<B> as $imp>::Output;
            fn $method(self, rhs: Bits<B>) -> <Bits<B> as $imp>::Output {
                $imp::$method(*self, rhs)
            }
        }
        impl<B: BitOps> $imp<&Bits<B>> for Bits<B> {
            type Output = <Bits<B> as $imp>::Output;
            fn $method(self, rhs: &Bits<B>) -> <Bits<B> as $imp>::Output {
                $imp::$method(self, *rhs)
            }
        }
        impl<B: BitOps> $imp<&Bits<B>> for &Bits<B> {
            type Output = <Bits<B> as $imp>::Output;
            fn $method(self, rhs: &Bits<B>) -> <Bits<B> as $imp>::Output {
                $imp::$method(*self, *rhs)
            }
        }
    };
}
forward_ref_binop!(Sub, sub);
forward_ref_binop!(BitAnd, bitand);
forward_ref_binop!(BitOr, bitor);
forward_ref_binop!(BitXor, bitxor);

macro_rules! impl_assign {
    ($imp:ident, $assign:ident, $op:ident) => {
        impl<B: BitOps> $imp for Bits<B> {
            fn $assign(&mut self, rhs: Bits<B>) {
                *self = self.$op(rhs);
            }
        }
        impl<B: BitOps> $imp<&Bits<B>> for Bits<B> {
            fn $assign(&mut self, rhs: &Bits<B>) {
                *self = self.$op(rhs);
            }
        }
    };
}
impl_assign!(SubAssign, sub_assign, sub);
impl_assign!(BitAndAssign, bitand_assign, bitand);
impl_assign!(BitOrAssign, bitor_assign, bitor);
impl_assign!(BitXorAssign, bitxor_assign, bitxor);

/// 楽なので`Copy`を課してある。
pub trait BitOps:
    Eq
    + Copy
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + Not<Output = Self>
    + Sub<Output = Self>
    + SubAssign
{
    const BIT_SIZE: usize;
    const ZERO: Self;
    const ONE: Self;
    fn count_ones(self) -> usize;
    fn trailing_zeros(self) -> usize;
    fn leading_zeros(self) -> usize;
}
macro_rules! impl_bitops {
    ($int:ty, $size:expr) => {
        impl BitOps for $int {
            const BIT_SIZE: usize = $size;
            const ZERO: Self = 0;
            const ONE: Self = 1;
            fn count_ones(self) -> usize {
                self.count_ones() as usize
            }
            fn trailing_zeros(self) -> usize {
                self.trailing_zeros() as usize
            }
            fn leading_zeros(self) -> usize {
                self.leading_zeros() as usize
            }
        }
    };
}
impl_bitops!(u8, 8);
impl_bitops!(u16, 16);
impl_bitops!(u32, 32);
impl_bitops!(u64, 64);
impl_bitops!(u128, 128);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_iter() {
        let mut v = vec![0, 4, 7, 32, 63];
        let mut s = Bits::<u64>::default();
        for &i in &v {
            s.insert(i);
        }
        assert_eq!(v, s.iter().collect::<Vec<_>>());
        v.reverse();
        assert_eq!(v, s.iter().rev().collect::<Vec<_>>());
    }
}
