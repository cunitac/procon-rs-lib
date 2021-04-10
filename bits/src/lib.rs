use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not,
    Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Bits(usize);

impl Bits {
    #[cfg(target_pointer_width = "32")]
    const CAPACITY: usize = 32;
    #[cfg(target_pointer_width = "64")]
    const CAPACITY: usize = 64;

    pub fn new() -> Self {
        Self(0)
    }
    pub fn filled(len: usize) -> Self {
        if len == Self::CAPACITY {
            Self(!0)
        } else {
            Self((1 << len) - 1)
        }
    }
    pub fn complement(self, len: usize) -> Self {
        Self(!self.0 & Self::filled(len).0)
    }
    pub fn single_bit(i: usize) -> Self {
        Self(1 << i)
    }
    pub fn sub_bits(self) -> SubBits {
        SubBits::new(self)
    }
    pub fn super_bits(self, len: usize) -> impl Iterator<Item = Self> + DoubleEndedIterator {
        SubBits::new(Self::filled(len) - self).map(move |bits| bits | self)
    }
    pub fn super_bits_in(self, all: Self) -> impl Iterator<Item = Self> + DoubleEndedIterator {
        SubBits::new(all - self).map(move |bits| bits | self)
    }
}

impl Default for Bits {
    fn default() -> Self {
        Self::new()
    }
}

impl Not for Bits {
    type Output = Self;
    fn not(self) -> Self {
        Bits(!self.0)
    }
}
impl BitOr for Bits {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Bits(self.0 | rhs.0)
    }
}
impl BitOrAssign for Bits {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}
impl BitAnd for Bits {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Bits(self.0 & rhs.0)
    }
}
impl BitAndAssign for Bits {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}
impl BitXor for Bits {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Bits(self.0 ^ rhs.0)
    }
}
impl BitXorAssign for Bits {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}
impl Sub for Bits {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self & !rhs
    }
}
impl SubAssign for Bits {
    fn sub_assign(&mut self, rhs: Self) {
        *self &= !rhs
    }
}

impl<T> Shl<T> for Bits
where
    usize: Shl<T, Output = usize>,
{
    type Output = Self;
    fn shl(self, rhs: T) -> Self {
        Self(self.0 << rhs)
    }
}
impl<T> ShlAssign<T> for Bits
where
    usize: ShlAssign<T>,
{
    fn shl_assign(&mut self, rhs: T) {
        self.0 <<= rhs;
    }
}
impl<T> Shr<T> for Bits
where
    usize: Shr<T, Output = usize>,
{
    type Output = Self;
    fn shr(self, rhs: T) -> Self {
        Self(self.0 >> rhs)
    }
}
impl<T> ShrAssign<T> for Bits
where
    usize: ShrAssign<T>,
{
    fn shr_assign(&mut self, rhs: T) {
        self.0 >>= rhs;
    }
}

/// `Bits + usize`は`Bits | single_bit(usize)`
#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<usize> for Bits {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        self | Self::single_bit(rhs)
    }
}
impl AddAssign<usize> for Bits {
    fn add_assign(&mut self, rhs: usize) {
        *self |= Self::single_bit(rhs)
    }
}
/// `Bits - usize`は`Bits - single_bit(usize)`
impl Sub<usize> for Bits {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        self - Self::single_bit(rhs)
    }
}
impl SubAssign<usize> for Bits {
    fn sub_assign(&mut self, rhs: usize) {
        *self -= Self::single_bit(rhs)
    }
}

impl Index<usize> for Bits {
    type Output = bool;
    fn index(&self, i: usize) -> &bool {
        if *self & Self::single_bit(i) != Self::new() {
            &true
        } else {
            &false
        }
    }
}

impl IntoIterator for Bits {
    type IntoIter = IntoIter;
    type Item = usize;
    fn into_iter(self) -> IntoIter {
        IntoIter { bits: self }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct IntoIter {
    bits: Bits,
}
impl Iterator for IntoIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let min = (*self).min()?;
        self.bits -= min;
        Some(min)
    }
    fn min(self) -> Option<usize> {
        let tz = self.bits.0.trailing_zeros() as usize;
        if tz == Bits::CAPACITY {
            None
        } else {
            Some(tz)
        }
    }
    fn max(self) -> Option<usize> {
        let lz = self.bits.0.leading_zeros() as usize;
        if lz == Bits::CAPACITY {
            None
        } else {
            Some(Bits::CAPACITY - lz - 1)
        }
    }
}
impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<usize> {
        let max = (*self).max()?;
        self.bits -= max;
        Some(max)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubBits {
    small: usize,
    large: usize,
    all: usize,
    finished: bool,
}
impl SubBits {
    fn new(Bits(all): Bits) -> Self {
        SubBits {
            small: 0,
            large: all,
            all,
            finished: false,
        }
    }
}
impl Iterator for SubBits {
    type Item = Bits;
    fn next(&mut self) -> Option<Bits> {
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
impl DoubleEndedIterator for SubBits {
    fn next_back(&mut self) -> Option<Bits> {
        if self.finished {
            None
        } else {
            self.finished = self.small == self.large;
            let mut ret = self.large.wrapping_sub(1) & self.all;
            std::mem::swap(&mut self.large, &mut ret);
            Some(Bits(ret))
        }
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
        let mut iter = Bits(0b1011_1101).into_iter();
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
