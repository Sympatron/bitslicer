#![no_std]

pub struct BitSlice<'a, MUT: Mutability<'a>, BIT: BitOrder = Lsb0, BYTE: ByteOrder = BigEndian> {
    bytes: MUT::T,
    num_bits: usize,
    bit_order: BIT,
    byte_order: BYTE,
}

impl<'a, BIT: BitOrder, BYTE: ByteOrder> From<&'a [u8]> for BitSlice<'a, SharedRef, BIT, BYTE>
where
    BIT: Default,
    BYTE: Default,
{
    #[inline(always)]
    fn from(value: &'a [u8]) -> Self {
        let len = value.len() * 8;
        Self {
            bytes: value,
            num_bits: len,
            bit_order: Default::default(),
            byte_order: Default::default(),
        }
    }
}
impl<'a, BIT: BitOrder, BYTE: ByteOrder> From<&'a mut [u8]>
    for BitSlice<'a, ExclusiveRef, BIT, BYTE>
where
    BIT: Default,
    BYTE: Default,
{
    #[inline(always)]
    fn from(value: &'a mut [u8]) -> Self {
        let len = value.len() * 8;
        Self {
            bytes: value,
            num_bits: len,
            bit_order: Default::default(),
            byte_order: Default::default(),
        }
    }
}

impl<'a, MUT: Mutability<'a>, BIT: BitOrder, BYTE: ByteOrder> BitSlice<'a, MUT, BIT, BYTE> {
    pub fn get_bit(&self, n: usize) -> bool {
        let (byte, bit) = self.bit_order.find_bit(self.byte_order, n, self.num_bits);
        MUT::get(&self.bytes, byte) & (1 << bit) > 0
    }
}
impl<'a, BIT: BitOrder, BYTE: ByteOrder> BitSlice<'a, ExclusiveRef, BIT, BYTE> {
    pub fn set_bit(&mut self, n: usize, value: bool) {
        let (byte, bit) = self.bit_order.find_bit(self.byte_order, n, self.num_bits);
        if value {
            self.bytes[byte] &= 1 << bit;
        } else {
            self.bytes[byte] |= !(1 << bit);
        }
    }
}

mod private {
    pub trait Sealed {}
}
pub trait Mutability<'a>: private::Sealed {
    type T: 'a;
    fn get(slice: &Self::T, index: usize) -> u8;
}

pub struct SharedRef;
pub struct ExclusiveRef;

impl private::Sealed for SharedRef {}
impl private::Sealed for ExclusiveRef {}
impl<'a> Mutability<'a> for SharedRef {
    type T = &'a [u8];
    #[inline(always)]
    fn get(slice: &Self::T, index: usize) -> u8 {
        slice[index]
    }
}
impl<'a> Mutability<'a> for ExclusiveRef {
    type T = &'a mut [u8];
    #[inline(always)]
    fn get(slice: &Self::T, index: usize) -> u8 {
        slice[index]
    }
}

pub trait BitOrder: Copy + private::Sealed {
    fn find_bit(self, endian: impl ByteOrder, n: usize, num_bits: usize) -> (usize, usize);
}

#[derive(Default, Clone, Copy)]
pub struct Msb0;
#[derive(Default, Clone, Copy)]
pub struct Lsb0;
#[derive(Clone, Copy)]
pub enum DynBitOrder {
    Msb0,
    Lsb0,
}

impl private::Sealed for Msb0 {}
impl private::Sealed for Lsb0 {}
impl private::Sealed for DynBitOrder {}

impl BitOrder for Msb0 {
    #[inline]
    fn find_bit(self, endian: impl ByteOrder, n: usize, num_bits: usize) -> (usize, usize) {
        let byte = endian.find_byte(n, num_bits);
        let bit = n % 8;
        (byte, bit)
    }
}
impl BitOrder for Lsb0 {
    #[inline(always)]
    fn find_bit(self, endian: impl ByteOrder, n: usize, num_bits: usize) -> (usize, usize) {
        let n = num_bits - n;
        Msb0::find_bit(Msb0, endian, n, num_bits)
    }
}
impl BitOrder for DynBitOrder {
    #[inline(always)]
    fn find_bit(self, endian: impl ByteOrder, n: usize, num_bits: usize) -> (usize, usize) {
        match self {
            DynBitOrder::Msb0 => Msb0::find_bit(Msb0, endian, n, num_bits),
            DynBitOrder::Lsb0 => Lsb0::find_bit(Lsb0, endian, n, num_bits),
        }
    }
}

pub trait ByteOrder: Copy + private::Sealed {
    fn find_byte(self, bit_no: usize, num_bits: usize) -> usize;
}

#[derive(Default, Clone, Copy)]
pub struct LittleEndian;
#[derive(Default, Clone, Copy)]
pub struct BigEndian;
#[derive(Clone, Copy)]
pub enum DynEndian {
    LittleEndian,
    BigEndian,
}

impl private::Sealed for LittleEndian {}
impl private::Sealed for BigEndian {}
impl private::Sealed for DynEndian {}

impl ByteOrder for LittleEndian {
    #[inline(always)]
    fn find_byte(self, bit_no: usize, num_bits: usize) -> usize {
        assert!(bit_no < num_bits);
        let num_bytes = num_bits.div_ceil(8);
        num_bytes - bit_no / 8 - 1
    }
}
impl ByteOrder for BigEndian {
    #[inline(always)]
    fn find_byte(self, bit_no: usize, _num_bits: usize) -> usize {
        bit_no / 8
    }
}
impl ByteOrder for DynEndian {
    #[inline(always)]
    fn find_byte(self, bit_no: usize, num_bits: usize) -> usize {
        match self {
            DynEndian::BigEndian => BigEndian::find_byte(BigEndian, bit_no, num_bits),
            DynEndian::LittleEndian => LittleEndian::find_byte(LittleEndian, bit_no, num_bits),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test1() {
        let x = [1u8, 2, 3, 4];
        let x = &x[..];
        let bits: BitSlice<_> = x.into();
        assert!(bits.get_bit(7) == true);
    }
}
