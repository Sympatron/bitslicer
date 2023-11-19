#![no_std]
#![allow(unused_macros)]

use core::ops::{Bound, RangeBounds};

pub struct BitSlice<S, BIT = Lsb0, BYTE = LittleEndian> {
    bytes: S,
    start_bit: u8,
    num_bits: usize,
    bit_order: BIT,
    byte_order: BYTE,
}

impl<'a, BIT, BYTE> From<&'a [u8]> for BitSlice<&'a [u8], BIT, BYTE>
where
    BIT: Default,
    BYTE: Default,
{
    #[inline(always)]
    fn from(bytes: &'a [u8]) -> Self {
        let len = bytes.len() * 8;
        Self::new(bytes, len)
    }
}
impl<'a, BIT, BYTE> From<&'a mut [u8]> for BitSlice<&'a mut [u8], BIT, BYTE>
where
    BIT: Default,
    BYTE: Default,
{
    #[inline(always)]
    fn from(bytes: &'a mut [u8]) -> Self {
        let len = bytes.len() * 8;
        Self::new(bytes, len)
    }
}
impl<S: AsRef<[u8]>, BIT, BYTE> BitSlice<S, BIT, BYTE> {
    #[inline(always)]
    pub fn new(bytes: S, num_bits: usize) -> Self
    where
        BIT: Default,
        BYTE: Default,
    {
        Self::new_with_order(bytes, num_bits, Default::default(), Default::default())
    }
    #[inline(always)]
    pub fn new_with_order(bytes: S, num_bits: usize, bit_order: BIT, endianness: BYTE) -> Self {
        Self {
            bytes,
            start_bit: 0,
            num_bits,
            bit_order,
            byte_order: endianness,
        }
    }
}
impl<S, BIT, BYTE> BitSlice<S, BIT, BYTE> {
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.num_bits
    }
}
impl<S: AsRef<[u8]>, BIT: BitOrder, BYTE: ByteOrder> BitSlice<S, BIT, BYTE> {
    pub fn get_bit(&self, n: usize) -> bool {
        let (byte, bit) =
            self.bit_order
                .find_bit(self.byte_order, n + self.start_bit as usize, self.num_bits);
        (self.bytes.as_ref()[byte] & (1 << bit)) > 0
    }
}
impl<S: AsMut<[u8]>, BIT: BitOrder, BYTE: ByteOrder> BitSlice<S, BIT, BYTE> {
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
impl<S: AsRef<[u8]>, BIT: BitOrder, BYTE: ByteOrder> BitSlice<S, BIT, BYTE> {
    pub fn slice<'a>(
        &'a self,
        range: impl RangeBounds<usize>,
    ) -> BitSlice<impl AsRef<[u8]> + 'a, BIT, BYTE> {
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

#[macro_export]
macro_rules! bits {
    ($($bit:expr),*) => {{
        // Create a temporary array to store the bits.
        let array = [0u8; (count_expr!($($bit),*) + 7) /8];

        let mut slice = BitSlice::new(array, count_expr!($($bit),*));

        // Set each bit in the array.
        let mut index = 0;
        $(
            slice.set_bit(index, $bit != 0);

            index += 1;
        )*

        slice
    }};
}

// Helper macro to count the number of expressions passed.
macro_rules! count_expr {
    () => (0usize);
    ($head:expr $(, $tail:expr)*) => (1usize + count_expr!($($tail),*));
}

pub struct BitIter<S, BIT, BYTE> {
    slice: BitSlice<S, BIT, BYTE>,
    idx: usize,
}

impl<S: AsRef<[u8]>, BIT: BitOrder, BYTE: ByteOrder> IntoIterator for BitSlice<S, BIT, BYTE> {
    type Item = bool;
    type IntoIter = BitIter<S, BIT, BYTE>;
    fn into_iter(self) -> Self::IntoIter {
        BitIter {
            slice: self,
            idx: 0,
        }
    }
}
impl<S: AsMut<[u8]>, BIT: BitOrder, BYTE: ByteOrder> BitIter<S, BIT, BYTE> {
    pub fn set_next_bit(&mut self, value: bool) {
        self.slice.set_bit(self.idx, value);
        self.idx += 1;
    }
}
impl<S: AsRef<[u8]>, BIT: BitOrder, BYTE: ByteOrder> Iterator for BitIter<S, BIT, BYTE> {
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

fn range_to_bounds(
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
