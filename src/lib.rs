//! # bitslicer
//!
//! This crate provides utilities for bit-level operations on data. It is designed to offer flexible and efficient ways to manipulate bits in various storage formats. The primary focus of this crate is to allow detailed control over bit-level data and to cater to scenarios where such control is crucial.
//!
//! ## Features
//!
//! - **Bit Order Handling**: Support for different bit ordering (e.g., MSB-first, LSB-first), allowing users to specify how bits are read from and written to the underlying storage.
//! - **Byte Order Handling**: Support for different byte endianness (e.g., little endian, big endian), enabling interpretation of byte sequences according to the specified byte order.
//! - **[`BitSlice`] Structure**: The primary feature of this crate, [`BitSlice`] provides a view into a sequence of bits, supporting operations like reading a bit at a specific index, slicing a range of bits, and setting the value of a bit. [`BitSlice`] is flexible in terms of the underlying storage and can be parameterized with different bit and byte orders.
//! - **[`BitIter`] Iterator**: An iterator over the bits in a [`BitSlice`], offering both read and write capabilities for individual bits.
//! - **Macros for Convenience**: Macros like [`bits!`] to facilitate easy and concise creation of [`BitSlice`] instances from literal sequences of bits.
//!
//! ### Optional `alloc` Feature
//!
//! Enabling the `alloc` feature adds:
//! - Conversion of [`BitSlice`] to a bit string (e.g., "1010110").
//! - Implementation of the [`Debug`](core::fmt::Debug) trait for [`BitSlice`].
//!
//! ## Example
//!
//! ```rust
//! use bitslicer::{BitSlice, LittleEndian, Lsb0, bits};
//!
//! let mut data = [0b01010101, 0b11110010];
//! let mut bit_slice: BitSlice<_, Lsb0, LittleEndian> = BitSlice::new(&mut data, 16);
//!
//! // Iterate over bits
//! for bit in bit_slice.iter() {
//!     println!("{}", bit);
//! }
//!
//! // Use bit_slice to access and manipulate bits...
//! assert_eq!(bit_slice.get_bit(2), true);
//! bit_slice.set_bit(2, false);
//! assert_eq!(bit_slice.get_bit(2), false);
//!
//! let sub_slice = bit_slice.slice(1..10);
//! assert!(sub_slice == bits![0, 0, 0, 1, 0, 1, 0, 0, 1]);
//! ```

#![no_std]

use core::ops::{Bound, RangeBounds};

#[cfg(feature = "alloc")]
extern crate alloc;

mod order;
pub use order::*;

/// Represents a view into a sequence of bits.
///
/// This struct can handle different bit orders and byte endianness, making it flexible for various use cases.
///
/// # Type Parameters
/// - `S`: The underlying storage type, typically a byte slice.
/// - `B`: The bit order, which dictates the order in which bits are read.
/// - `Endian`: The byte order, which dictates the order in which bytes are read.
pub struct BitSlice<S, B = Lsb0, Endian = LittleEndian> {
    bytes: S,
    start_bit: usize,
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
    /// Returns the number of bits in the slice.
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.num_bits
    }
}
impl<S: AsRef<[u8]>, B, Endian> BitSlice<S, B, Endian> {
    /// Creates a new [BitSlice] from a given storage with default bit and byte order.
    ///
    /// # Arguments
    /// * `bytes` - The underlying storage for the bits.
    /// * `num_bits` - The total number of bits to be represented.
    ///
    /// # Returns
    /// A new [BitSlice] instance.
    #[inline(always)]
    pub fn new(bytes: S, num_bits: usize) -> Self
    where
        B: Default,
        Endian: Default,
    {
        Self::new_with_order(bytes, num_bits, Default::default(), Default::default())
    }
    /// Creates a new [BitSlice] with specific bit and byte order.
    ///
    /// # Arguments
    /// * `bytes` - The underlying storage for the bits.
    /// * `num_bits` - The total number of bits to be represented.
    /// * `bit_order` - The bit order to use.
    /// * `endianness` - The byte order to use.
    ///
    /// # Returns
    /// A new [BitSlice] instance with the specified ordering.
    #[inline(always)]
    pub fn new_with_order(bytes: S, num_bits: usize, bit_order: B, endianness: Endian) -> Self {
        assert!(bytes.as_ref().len() * 8 >= num_bits);
        Self {
            bytes,
            start_bit: 0,
            num_bits,
            bit_order,
            byte_order: endianness,
        }
    }
    /// Retrieves the value of a bit at a specified index.
    ///
    /// # Arguments
    /// * `n` - The index of the bit to retrieve.
    ///
    /// # Returns
    /// `true` if the bit is set; `false` otherwise.
    ///
    /// # Panics
    /// Panics if `n` is out of bounds.
    pub fn get_bit(&self, n: usize) -> bool
    where
        B: BitOrder,
        Endian: ByteOrder,
    {
        assert!(n < self.num_bits);
        let (byte, bit) =
            self.bit_order
                .find_bit(self.byte_order, n + self.start_bit, self.num_bits);
        (self.bytes.as_ref()[byte] & (1 << bit)) > 0
    }
    /// Returns a [BitSlice] representing a sub-slice of the current slice.
    ///
    /// # Arguments
    /// * `range` - The range of bits to include in the sub-slice.
    ///
    /// # Returns
    /// A new [BitSlice] representing the specified range.
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
        assert!(start_bit <= end_excl_bit);
        assert!(end_excl_bit <= self.num_bits);
        BitSlice {
            bytes: self.bytes.as_ref(),
            num_bits: end_excl_bit - start_bit,
            start_bit: start_bit,
            bit_order: self.bit_order,
            byte_order: self.byte_order,
        }
    }
    pub fn iter(&self) -> BitIter<&[u8], B, Endian>
    where
        B: Copy,
        Endian: Copy,
    {
        BitIter {
            slice: BitSlice {
                bytes: self.bytes.as_ref(),
                start_bit: self.start_bit,
                num_bits: self.num_bits,
                bit_order: self.bit_order,
                byte_order: self.byte_order,
            },
            idx: 0,
        }
    }
    /// Converts the [BitSlice] to string of bits.
    #[cfg(feature = "alloc")]
    pub fn bits_to_string(&self) -> alloc::string::String
    where
        B: BitOrder,
        Endian: ByteOrder,
    {
        (0..self.num_bits)
            .map(|n| if self.get_bit(n) { '1' } else { '0' })
            .collect()
    }
}
impl<S: AsMut<[u8]>, B: BitOrder, Endian: ByteOrder> BitSlice<S, B, Endian> {
    /// Sets the value of a bit at a specified index.
    ///
    /// # Arguments
    /// * `n` - The index of the bit to set.
    /// * `value` - The value to set the bit to (`true` for set, `false` for clear).
    ///
    /// # Panics
    /// Panics if `n` is out of bounds.
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

// Implementation of `TryFrom<BitSlice>` for all unsigned integer types
macro_rules! impl_try_from_bitslice {
    ($($t:ty),*) => {
        $(
            impl<S: AsRef<[u8]>, B: BitOrder, Endian: ByteOrder> TryFrom<BitSlice<S, B, Endian>> for $t {
                type Error = ConversionError;
                #[inline(always)]
                fn try_from(value: BitSlice<S, B, Endian>) -> Result<Self, Self::Error> {
                    value
                        .to_uint(Self::BITS as usize)
                        .and_then(|v| v.try_into().map_err(|_| ConversionError))
                }
            }
        )*
    };
}
impl_try_from_bitslice!(u8, u16, u32, u64, u128, usize);

// Implementation of `Into<BitSlice>` for all unsigned integer types
macro_rules! impl_into_bitslice {
    ($($t:ty),*) => {
        $(
            impl<B: BitOrder + Default, Endian: ByteOrder + Default> From<$t>
                for BitSlice<[u8; (<$t>::BITS / 8) as usize], B, Endian>
            {
                fn from(mut value: $t) -> Self {
                    const BITS: usize = <$t>::BITS as _;
                    let arr = [0; (BITS / 8) as usize];
                    let mut slice: BitSlice<_, B, Endian> = BitSlice::new(arr, BITS);
                    for idx in 0..BITS {
                        if value == 0 {
                            break;
                        }
                        slice.set_bit(idx, (value & 1) == 1);
                        value >>= 1;
                    }
                    slice
                }
            }
        )*
    };
}
impl_into_bitslice!(u8, u16, u32, u64, u128, usize);

impl<S: AsRef<[u8]>, S2: AsRef<[u8]>, B: BitOrder, Endian: ByteOrder>
    PartialEq<BitSlice<S2, B, Endian>> for BitSlice<S, B, Endian>
{
    fn eq(&self, other: &BitSlice<S2, B, Endian>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            if self.get_bit(i) != other.get_bit(i) {
                return false;
            }
        }
        true
    }
}
impl<S: AsRef<[u8]>, T: AsRef<[bool]>, B: BitOrder, Endian: ByteOrder> PartialEq<T>
    for BitSlice<S, B, Endian>
{
    fn eq(&self, other: &T) -> bool {
        if self.len() != other.as_ref().len() {
            return false;
        }
        for i in 0..self.len() {
            if self.get_bit(i) != other.as_ref()[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(feature = "alloc")]
impl<S, B, Endian> core::fmt::Debug for BitSlice<S, B, Endian>
where
    S: AsRef<[u8]>,
    B: BitOrder + core::fmt::Debug,
    Endian: ByteOrder + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Start the debug struct.
        let mut name = alloc::string::String::new();
        let mut bits_field = alloc::string::String::new();
        core::fmt::write(
            &mut name,
            core::format_args!("BitSlice<_, {:?}, {:?}>", self.bit_order, self.byte_order,),
        )?;
        core::fmt::write(
            &mut bits_field,
            core::format_args!(
                "bits[{}..{}]",
                self.start_bit,
                self.start_bit + self.num_bits,
            ),
        )?;
        f.debug_struct(&name)
            // Add the bytes field.
            .field("bytes", &self.bytes.as_ref())
            // Add the bits range and their representation.
            .field(&bits_field, &self.bits_to_string())
            .finish()
    }
}

/// A macro to conveniently create a [BitSlice] from a list of boolean values.
///
/// # Examples
/// ```
/// use bitslicer::{BitSlice, LittleEndian, Lsb0, bits};
/// let my_bits: BitSlice<_, Lsb0, LittleEndian> = bits![1, 0, 1];
/// ```
#[macro_export]
macro_rules! bits {
    ($($bit:expr),*) => {{
        // Create a temporary array to store the bits.
        let array = [0u8; ($crate::count_expr!($($bit),*) + 7) /8];

        let mut slice = $crate::BitSlice::new(array, $crate::count_expr!($($bit),*));

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
#[macro_export]
#[doc(hidden)]
macro_rules! count_expr {
    () => (0usize);
    ($head:expr $(, $tail:expr)*) => (1usize + $crate::count_expr!($($tail),*));
}

/// An iterator over the bits of a [BitSlice].
///
/// This struct allows iterating over the bits in a [BitSlice], providing read (and optionally write) access to each bit.
///
/// # Type Parameters
/// - `S`: The underlying storage type, typically a byte slice.
/// - `B`: The bit order.
/// - `Endian`: The byte order.
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
    #[inline(always)]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.len() > 0 {
            Some(self.slice.get_bit(self.slice.len() - 1))
        } else {
            None
        }
    }
    #[inline(always)]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.idx += n;
        self.next()
    }
    #[inline(always)]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.slice.len() - self.idx
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.slice.len() - self.idx;
        (len, Some(len))
    }
}
impl<S: AsRef<[u8]>, B: BitOrder, Endian: ByteOrder> ExactSizeIterator for BitIter<S, B, Endian> {
    fn len(&self) -> usize {
        self.slice.len() - self.idx
    }
}

/// A utility function for converting range bounds to start and end indices.
///
/// This function is used internally by [BitSlice] to handle slicing operations.
///
/// # Arguments
/// - `start_bound`: The start bound of the range.
/// - `end_bound`: The end bound of the range.
/// - `len`: The length of the underlying byte sequence.
///
/// # Returns
/// A tuple containing the start index and exclusive end index.
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

#[cfg(test)]
mod tests;
