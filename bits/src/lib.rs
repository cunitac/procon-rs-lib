use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Sub,
    SubAssign,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Bits<B: BitsBase = u32>(pub B);

impl<B: BitsBase> Bits<B> {
    pub fn new() -> Self {
        Self(B::zero())
    }
    pub fn filled(len: usize) -> Self {
        Self(B::ones(len))
    }
    pub fn complement(self, len: usize) -> Self {
        !self & Self::filled(len)
    }
    pub fn single_bit(i: usize) -> Self {
        Self(B::one() << i)
    }
    pub fn set_bits(self) -> SetBits<B> {
        SetBits::new(self)
    }
    pub fn sub_bits(self) -> SubBits<B> {
        SubBits::new(self)
    }
    pub fn super_bits(self, len: usize) -> impl Iterator<Item = Self> + DoubleEndedIterator {
        SubBits::new(Self::filled(len) - self).map(move |bits| bits | self)
    }
    pub fn super_bits_in(self, all: Self) -> impl Iterator<Item = Self> + DoubleEndedIterator {
        SubBits::new(all - self).map(move |bits| bits | self)
    }
}

impl<B: BitsBase> Default for Bits<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: BitsBase> Not for Bits<B> {
    type Output = Self;
    fn not(self) -> Self {
        Bits(!self.0)
    }
}
impl<B: BitsBase> BitOr for Bits<B> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Bits(self.0 | rhs.0)
    }
}
impl<B: BitsBase> BitOrAssign for Bits<B> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}
impl<B: BitsBase> BitAnd for Bits<B> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Bits(self.0 & rhs.0)
    }
}
impl<B: BitsBase> BitAndAssign for Bits<B> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}
impl<B: BitsBase> BitXor for Bits<B> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Bits(self.0 ^ rhs.0)
    }
}
impl<B: BitsBase> BitXorAssign for Bits<B> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}
impl<B: BitsBase> Sub for Bits<B> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self & !rhs
    }
}
impl<B: BitsBase> SubAssign for Bits<B> {
    fn sub_assign(&mut self, rhs: Self) {
        *self &= !rhs
    }
}

/// `Bits + usize`は`Bits | single_bit(usize)`
impl<B: BitsBase> Add<usize> for Bits<B> {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        self | Self::single_bit(rhs)
    }
}
impl<B: BitsBase> AddAssign<usize> for Bits<B> {
    fn add_assign(&mut self, rhs: usize) {
        *self |= Self::single_bit(rhs)
    }
}
/// `Bits - usize`は`Bits - single_bit(usize)`
impl<B: BitsBase> Sub<usize> for Bits<B> {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        self - Self::single_bit(rhs)
    }
}
impl<B: BitsBase> SubAssign<usize> for Bits<B> {
    fn sub_assign(&mut self, rhs: usize) {
        *self -= Self::single_bit(rhs)
    }
}

impl<B: BitsBase> std::ops::Index<usize> for Bits<B> {
    type Output = bool;
    fn index(&self, i: usize) -> &bool {
        const TRUE: bool = true;
        const FALSE: bool = false;
        if self.0.test_bit(i) {
            &TRUE
        } else {
            &FALSE
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SetBits<B: BitsBase>(B);
impl<B: BitsBase> SetBits<B> {
    fn new(Bits(bits): Bits<B>) -> Self {
        Self(bits)
    }
}
impl<B: BitsBase> Iterator for SetBits<B> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let min = (*self).min()?;
        self.0 &= !(B::one() << min);
        Some(min)
    }
    fn min(self) -> Option<usize> {
        let tz = self.0.trailing_zeros();
        if tz == B::SIZE {
            None
        } else {
            Some(tz)
        }
    }
    fn max(self) -> Option<usize> {
        let lz = self.0.leading_zeros();
        if lz == B::SIZE {
            None
        } else {
            Some(B::SIZE - lz - 1)
        }
    }
}
impl<B: BitsBase> DoubleEndedIterator for SetBits<B> {
    fn next_back(&mut self) -> Option<usize> {
        let max = (*self).max()?;
        self.0 &= !(B::one() << max);
        Some(max)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubBits<B: BitsBase> {
    small: B,
    large: B,
    all: B,
    finished: bool,
}
impl<B: BitsBase> SubBits<B> {
    fn new(Bits(all): Bits<B>) -> Self {
        SubBits {
            small: B::zero(),
            large: all,
            all,
            finished: false,
        }
    }
}
impl<B: BitsBase> Iterator for SubBits<B> {
    type Item = Bits<B>;
    fn next(&mut self) -> Option<Bits<B>> {
        if self.finished {
            None
        } else {
            self.finished = self.small == self.large;
            let mut ret = self.small.wrapping_sub(self.all) & self.all;
            std::mem::swap(&mut self.small, &mut ret);
            Some(Bits(ret))
        }
    }
}
impl<B: BitsBase> DoubleEndedIterator for SubBits<B> {
    fn next_back(&mut self) -> Option<Bits<B>> {
        if self.finished {
            None
        } else {
            self.finished = self.small == self.large;
            let mut ret = self.large.wrapping_sub(B::one()) & self.all;
            std::mem::swap(&mut self.large, &mut ret);
            Some(Bits(ret))
        }
    }
}

pub trait BitsBase:
    Copy
    + Not<Output = Self>
    + Shl<usize, Output = Self>
    + Sub<Output = Self>
    + From<bool>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
    + Eq
{
    const SIZE: usize;
    fn one() -> Self {
        true.into()
    }
    fn zero() -> Self {
        false.into()
    }
    fn test_bit(self, i: usize) -> bool {
        (self & (Self::one() << i)) != Self::zero()
    }
    fn ones(len: usize) -> Self {
        if len == Self::SIZE {
            !Self::zero()
        } else {
            (Self::one() << len) - Self::one()
        }
    }
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn trailing_zeros(self) -> usize;
    fn leading_zeros(self) -> usize;
}
impl BitsBase for u32 {
    const SIZE: usize = 32;
    fn wrapping_sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
    fn trailing_zeros(self) -> usize {
        self.trailing_zeros() as usize
    }
    fn leading_zeros(self) -> usize {
        self.leading_zeros() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sub_bits() {
        let mut iter = Bits(0b1101).sub_bits();
        // subbits = {0000, 0001, 0100, 0101, 1000, 1001, 1100, 1101}
        assert_eq!(iter.next(), Some(Bits(0b0000)));
        assert_eq!(iter.next(), Some(Bits(0b0001)));
        assert_eq!(iter.next(), Some(Bits(0b0100)));
        assert_eq!(iter.next_back(), Some(Bits(0b1101)));
        assert_eq!(iter.next_back(), Some(Bits(0b1100)));
        assert_eq!(iter.next_back(), Some(Bits(0b1001)));
        assert_eq!(iter.next(), Some(Bits(0b0101)));
        assert_eq!(iter.next(), Some(Bits(0b1000)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
    #[test]
    fn test_super_bits() {
        let mut iter = Bits(0b0010).super_bits(4);
        // superbits = {0010, 0011, 0110, 0111, 1010, 1011, 1110, 1111}
        assert_eq!(iter.next(), Some(Bits(0b0010)));
        assert_eq!(iter.next(), Some(Bits(0b0011)));
        assert_eq!(iter.next(), Some(Bits(0b0110)));
        assert_eq!(iter.next_back(), Some(Bits(0b1111)));
        assert_eq!(iter.next_back(), Some(Bits(0b1110)));
        assert_eq!(iter.next_back(), Some(Bits(0b1011)));
        assert_eq!(iter.next(), Some(Bits(0b0111)));
        assert_eq!(iter.next(), Some(Bits(0b1010)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
    #[test]
    fn test_set_bits() {
        let mut iter = Bits(0b1011_1101).set_bits();
        // setbits = {0, 2, 3, 4, 5, 7}
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(7));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
    #[test]
    fn test_bits_index() {
        let b = Bits(0b1011_1101);
        assert_eq!(b[0], true);
        assert_eq!(b[1], false);
        assert_eq!(b[2], true);
        assert_eq!(b[3], true);
        assert_eq!(b[4], true);
        assert_eq!(b[5], true);
        assert_eq!(b[6], false);
        assert_eq!(b[7], true);
    }
}
