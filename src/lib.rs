#![no_std]
#![allow(unused_macros)]

use core::ops::{Bound, RangeBounds};

pub struct BitSlice<S, B = Lsb0, Endian = LittleEndian> {
    bytes: S,
    start_bit: u8,
    num_bits: usize,
    bit_order: B,
    byte_order: Endian,
}

impl<'a, B, Endian> From<&'a [u8]> for BitSlice<&'a [u8], B, Endian>
where
    B: Default,
    Endian: Default,
{
    #[inline(always)]
    fn from(bytes: &'a [u8]) -> Self {
        let len = bytes.len() * 8;
        Self::new(bytes, len)
    }
}
impl<'a, B, Endian> From<&'a mut [u8]> for BitSlice<&'a mut [u8], B, Endian>
where
    B: Default,
    Endian: Default,
{
    #[inline(always)]
    fn from(bytes: &'a mut [u8]) -> Self {
        let len = bytes.len() * 8;
        Self::new(bytes, len)
    }
}
impl<S: AsRef<[u8]>, B, Endian> BitSlice<S, B, Endian> {}
impl<S, B, Endian> BitSlice<S, B, Endian> {
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.num_bits
    }
}
impl<S: AsRef<[u8]>, B, Endian> BitSlice<S, B, Endian> {
    #[inline(always)]
    pub fn new(bytes: S, num_bits: usize) -> Self
    where
        B: Default,
        Endian: Default,
    {
        Self::new_with_order(bytes, num_bits, Default::default(), Default::default())
    }
    #[inline(always)]
    pub const fn new_with_order(
        bytes: S,
        num_bits: usize,
        bit_order: B,
        endianness: Endian,
    ) -> Self {
        Self {
            bytes,
            start_bit: 0,
            num_bits,
            bit_order,
            byte_order: endianness,
        }
    }
    pub fn get_bit(&self, n: usize) -> bool
    where
        B: BitOrder,
        Endian: ByteOrder,
    {
        let (byte, bit) =
            self.bit_order
                .find_bit(self.byte_order, n + self.start_bit as usize, self.num_bits);
        (self.bytes.as_ref()[byte] & (1 << bit)) > 0
    }
    pub fn slice<'a>(
        &'a self,
        range: impl RangeBounds<usize>,
    ) -> BitSlice<impl AsRef<[u8]> + 'a, B, Endian>
    where
        B: Copy,
        Endian: Copy,
    {
        let (start_bit, end_excl_bit) = range_to_bounds(
            range.start_bound().cloned(),
            range.end_bound().cloned(),
            self.num_bits,
        );
        BitSlice {
            bytes: self.bytes.as_ref(),
            num_bits: end_excl_bit - start_bit,
            start_bit: start_bit as u8,
            bit_order: self.bit_order,
            byte_order: self.byte_order,
        }
    }
}
impl<S: AsMut<[u8]>, B: BitOrder, Endian: ByteOrder> BitSlice<S, B, Endian> {
    pub fn set_bit(&mut self, n: usize, value: bool) {
        let (byte, bit) =
            self.bit_order
                .find_bit(self.byte_order, n + self.start_bit as usize, self.num_bits);
        if value {
            self.bytes.as_mut()[byte] |= 1 << bit;
        } else {
            self.bytes.as_mut()[byte] &= !(1 << bit);
        }
    }
}

#[macro_export]
macro_rules! bits {
    ($($bit:expr),*) => {{
        // Create a temporary array to store the bits.
        let array = [0u8; (count_expr!($($bit),*) + 7) /8];

        let mut slice = BitSlice::new(array, count_expr!($($bit),*));

        // Set each bit in the array.
        let mut index = 0;
        $(
            slice.set_bit(index, ($bit as usize) != 0);
            index += 1;
        )*

        slice
    }};
}

/// Helper macro to count the number of expressions passed.
macro_rules! count_expr {
    () => (0usize);
    ($head:expr $(, $tail:expr)*) => (1usize + count_expr!($($tail),*));
}

pub struct BitIter<S, B, Endian> {
    slice: BitSlice<S, B, Endian>,
    idx: usize,
}

impl<S: AsRef<[u8]>, B: BitOrder, Endian: ByteOrder> IntoIterator for BitSlice<S, B, Endian> {
    type Item = bool;
    type IntoIter = BitIter<S, B, Endian>;
    fn into_iter(self) -> Self::IntoIter {
        BitIter {
            slice: self,
            idx: 0,
        }
    }
}
impl<S: AsMut<[u8]>, B: BitOrder, Endian: ByteOrder> BitIter<S, B, Endian> {
    pub fn set_next_bit(&mut self, value: bool) {
        self.slice.set_bit(self.idx, value);
        self.idx += 1;
    }
}
impl<S: AsRef<[u8]>, B: BitOrder, Endian: ByteOrder> Iterator for BitIter<S, B, Endian> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.slice.len() {
            let bit = self.slice.get_bit(self.idx);
            self.idx += 1;
            Some(bit)
        } else {
            None
        }
    }
}

const fn range_to_bounds(
    start_bound: Bound<usize>,
    end_bound: Bound<usize>,
    len: usize,
) -> (usize, usize) {
    let start_index = match start_bound {
        Bound::Included(idx) => idx,
        Bound::Excluded(idx) => idx + 1,
        Bound::Unbounded => 0,
    };
    let end_index = match end_bound {
        Bound::Included(idx) => idx + 1,
        Bound::Excluded(idx) => idx,
        Bound::Unbounded => len,
    };
    (start_index, end_index)
}

mod order;
pub use order::*;

#[cfg(test)]
mod tests;
