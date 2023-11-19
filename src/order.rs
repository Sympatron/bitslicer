mod private {
    pub trait Sealed {}
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
        let (byte, bit) = Lsb0::find_bit(Lsb0, endian, n, num_bits);
        (byte, 7 - bit)
    }
}
impl BitOrder for Lsb0 {
    #[inline(always)]
    fn find_bit(self, endian: impl ByteOrder, n: usize, num_bits: usize) -> (usize, usize) {
        let byte = endian.find_byte(n, num_bits);
        let bit = n % 8;
        (byte, bit)
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

impl ByteOrder for BigEndian {
    #[inline(always)]
    fn find_byte(self, bit_no: usize, num_bits: usize) -> usize {
        assert!(bit_no < num_bits);
        let num_bytes = num_bits.div_ceil(8);
        num_bytes - bit_no / 8 - 1
    }
}
impl ByteOrder for LittleEndian {
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
    fn test_big_endian_find_byte() {
        assert_eq!(BigEndian.find_byte(10, 32), 2);
        assert_eq!(BigEndian.find_byte(0, 16), 1);
    }

    #[test]
    fn test_little_endian_find_byte() {
        assert_eq!(LittleEndian.find_byte(10, 32), 1);
        assert_eq!(LittleEndian.find_byte(0, 16), 0);
    }

    #[test]
    fn test_dyn_endian_find_byte() {
        assert_eq!(DynEndian::BigEndian.find_byte(10, 32), 2);
        assert_eq!(DynEndian::LittleEndian.find_byte(10, 32), 1);
    }

    #[test]
    fn test_msb0_find_bit() {
        assert_eq!(Msb0.find_bit(LittleEndian, 10, 32), (1, 5));
    }

    #[test]
    fn test_lsb0_find_bit() {
        assert_eq!(Lsb0.find_bit(BigEndian, 10, 32), (2, 2));
    }

    #[test]
    fn test_dyn_bit_order_find_bit() {
        assert_eq!(DynBitOrder::Msb0.find_bit(LittleEndian, 10, 32), (1, 5));
        assert_eq!(DynBitOrder::Lsb0.find_bit(BigEndian, 10, 32), (2, 2));
    }
}
