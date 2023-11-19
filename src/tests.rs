use super::*;
extern crate alloc;
use alloc::vec;

#[test]
#[allow(unused_assignments)]
fn test_macro() {
    let bits: BitSlice<_, Lsb0, LittleEndian> = bits![0, 1, 1, 0, 1];
    assert!(bits == bits![0, 1, 1, 0, 1]);
    let mut bits = bits.into_iter();
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), None);
}
#[test]
fn test_slice() {
    let mut x = [1u8, 2, 3, 4];
    let bits: BitSlice<_, Lsb0, LittleEndian> = x.as_mut().into();
    let bits = bits.slice(14..19);
    let mut bits = bits.into_iter();
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), None);
}

#[test]
fn test_iter() {
    let mut x = [1u8, 2, 3, 4];
    let bits: BitSlice<_, Lsb0, LittleEndian> = x.as_mut().into();
    let mut bits = bits.into_iter();
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    for _ in 2..8 {
        assert_eq!(bits.next(), Some(false));
    }
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    for _ in 2..8 {
        assert_eq!(bits.next(), Some(false));
    }
    for _ in 0..16 {
        assert!(bits.next().is_some());
    }
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next(), None);
}

#[test]
fn test_lsb0_litle_endian() {
    let mut x = [1u8, 2, 3, 4];
    let mut bits: BitSlice<_, Lsb0, LittleEndian> = x.as_mut().into();
    assert_eq!(bits.get_bit(0), true);
    assert_eq!(bits.get_bit(1), false);
    assert_eq!(bits.get_bit(8), false);
    assert_eq!(bits.get_bit(9), true);
    bits.set_bit(1, true);
    assert_eq!(bits.get_bit(1), true);
    assert_eq!(
        bits.slice(0..8).into_iter().collect::<alloc::vec::Vec<_>>(),
        vec![true, true, false, false, false, false, false, false]
    );
    assert_eq!(x[0], 3);
}

#[test]
fn test_msb0_litle_endian() {
    let mut x = [1u8, 2, 3, 4];
    let mut bits: BitSlice<_, Msb0, LittleEndian> = x.as_mut().into();
    assert_eq!(bits.get_bit(7), true);
    assert_eq!(bits.get_bit(6), false);
    assert_eq!(bits.get_bit(15), false);
    assert_eq!(bits.get_bit(14), true);
    bits.set_bit(6, true);
    assert_eq!(bits.get_bit(6), true);
    assert_eq!(x[0], 3);
}

#[test]
fn test_lsb0_big_endian() {
    let mut x = [1u8, 2, 3, 4];
    let mut bits: BitSlice<_, Lsb0, BigEndian> = x.as_mut().into();
    assert_eq!(bits.get_bit(0), false);
    assert_eq!(bits.get_bit(1), false);
    assert_eq!(bits.get_bit(2), true);
    assert_eq!(bits.get_bit(8), true);
    assert_eq!(bits.get_bit(9), true);
    assert_eq!(bits.get_bit(10), false);
    bits.set_bit(6, true);
    assert_eq!(bits.get_bit(6), true);
    assert_eq!(x[3], 4 | (1 << 6));
}

#[test]
fn test_msb0_big_endian() {
    let mut x = [1u8, 2, 3, 4];
    let mut bits: BitSlice<_, Msb0, BigEndian> = x.as_mut().into();
    assert_eq!(bits.get_bit(7), false);
    assert_eq!(bits.get_bit(6), false);
    assert_eq!(bits.get_bit(5), true);
    assert_eq!(bits.get_bit(15), true);
    assert_eq!(bits.get_bit(14), true);
    assert_eq!(bits.get_bit(13), false);
    bits.set_bit(6, true);
    assert_eq!(bits.get_bit(6), true);
    assert_eq!(x[3], 6);
}
