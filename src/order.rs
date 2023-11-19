mod private {
    pub trait Sealed {}
}

/// The `BitOrder` trait defines behavior for bit ordering (e.g., MSB-first, LSB-first).
pub trait BitOrder: Copy + private::Sealed {
    /// Finds the position of a bit within a byte sequence.
    ///
    /// # Arguments
    /// * `endian` - The byte order (either big or little endian).
    /// * `n` - The bit index for which to find the position.
    /// * `num_bits` - The total number of bits in the sequence.
    ///
    /// # Returns
    /// A tuple `(usize, usize)` where the first element is the byte index and
    /// the second element is the bit index within that byte.
    fn find_bit(self, endian: impl ByteOrder, n: usize, num_bits: usize) -> (usize, usize);
}

/// Represents most significant bit first ordering.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Msb0;

/// Represents least significant bit first ordering.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Lsb0;

/// A dynamic bit order that can be either MSB0 or LSB0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynBitOrder {
    /// Represents most significant bit first ordering.
    Msb0,
    /// Represents least significant bit first ordering.
    Lsb0,
}
// Implementations of the `Sealed` trait for the bit order types.
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

impl PartialEq<Lsb0> for DynBitOrder {
    fn eq(&self, _other: &Lsb0) -> bool {
        *self == DynBitOrder::Lsb0
    }
}
impl PartialEq<Msb0> for DynBitOrder {
    fn eq(&self, _other: &Msb0) -> bool {
        *self == DynBitOrder::Msb0
    }
}
impl PartialEq<DynBitOrder> for Lsb0 {
    fn eq(&self, other: &DynBitOrder) -> bool {
        *other == DynBitOrder::Lsb0
    }
}
impl PartialEq<DynBitOrder> for Msb0 {
    fn eq(&self, other: &DynBitOrder) -> bool {
        *other == DynBitOrder::Msb0
    }
}
impl PartialEq<Msb0> for Lsb0 {
    fn eq(&self, _other: &Msb0) -> bool {
        false
    }
}
impl PartialEq<Lsb0> for Msb0 {
    fn eq(&self, _other: &Lsb0) -> bool {
        false
    }
}

/// The `ByteOrder` trait defines behavior for byte ordering.
pub trait ByteOrder: Copy + private::Sealed {
    /// Finds the byte index for a given bit index.
    ///
    /// # Arguments
    /// * `bit_no` - The bit index for which to find the byte index.
    /// * `num_bits` - The total number of bits in the sequence.
    ///
    /// # Returns
    /// The byte index corresponding to the provided bit index.
    fn find_byte(self, bit_no: usize, num_bits: usize) -> usize;
    fn is_native(self) -> bool;
}
/// Represents little endian byte ordering.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LittleEndian;

/// Represents big endian byte ordering.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BigEndian;

/// A dynamic endian type that can be either `LittleEndian` or `BigEndian`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynEndian {
    LittleEndian,
    BigEndian,
}

// Implementations of the `Sealed` trait for the byte order types.
impl private::Sealed for LittleEndian {}
impl private::Sealed for BigEndian {}
impl private::Sealed for DynEndian {}

// Implementations of `ByteOrder` trait for each endian type.
impl ByteOrder for BigEndian {
    #[inline(always)]
    fn find_byte(self, bit_no: usize, num_bits: usize) -> usize {
        assert!(bit_no < num_bits);
        let num_bytes = num_bits.div_ceil(8);
        num_bytes - bit_no / 8 - 1
    }
    fn is_native(self) -> bool {
        #[cfg(target_endian = "little")]
        return false;
        #[cfg(target_endian = "big")]
        return true;
    }
}
impl ByteOrder for LittleEndian {
    #[inline(always)]
    fn find_byte(self, bit_no: usize, _num_bits: usize) -> usize {
        bit_no / 8
    }
    fn is_native(self) -> bool {
        #[cfg(target_endian = "little")]
        return true;
        #[cfg(target_endian = "big")]
        return false;
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
    fn is_native(self) -> bool {
        #[cfg(target_endian = "little")]
        return self == DynEndian::LittleEndian;
        #[cfg(target_endian = "big")]
        return self == DynEndian::BigEndian;
    }
}

impl PartialEq<LittleEndian> for DynEndian {
    fn eq(&self, _other: &LittleEndian) -> bool {
        *self == DynEndian::LittleEndian
    }
}
impl PartialEq<BigEndian> for DynEndian {
    fn eq(&self, _other: &BigEndian) -> bool {
        *self == DynEndian::BigEndian
    }
}
impl PartialEq<DynEndian> for LittleEndian {
    fn eq(&self, other: &DynEndian) -> bool {
        *other == DynEndian::LittleEndian
    }
}
impl PartialEq<DynEndian> for BigEndian {
    fn eq(&self, other: &DynEndian) -> bool {
        *other == DynEndian::BigEndian
    }
}
impl PartialEq<LittleEndian> for BigEndian {
    fn eq(&self, _other: &LittleEndian) -> bool {
        false
    }
}
impl PartialEq<BigEndian> for LittleEndian {
    fn eq(&self, _other: &BigEndian) -> bool {
        false
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
