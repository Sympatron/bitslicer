use super::*;
extern crate alloc;
extern crate std;
use alloc::vec;
use std::println;

#[test]
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
    drop(bits);

    let bits: BitSlice<_, Msb0, BigEndian> = x.as_ref().into();
    let bits = bits.slice(14..19);
    let mut bits = bits.into_iter();
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(false));
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

    let x: BitSlice<_, Msb0, BigEndian> = 0xffffu16.into();
    for b in x {
        assert_eq!(b, true);
    }
    let x: BitSlice<_, Msb0, BigEndian> = [0xff, 0xff].as_ref().into();
    for b in &x {
        assert_eq!(b, true);
    }
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

#[test]
fn from_int() {
    let a: BitSlice<_> = 0b11101010u8.into();
    let b: BitSlice<_> = 0b11101010u16.into();
    let c: BitSlice<_> = 0b11101010u32.into();
    let d: BitSlice<_> = 0b11101010u64.into();
    assert_eq!(a, b.slice(0..8));
    assert_eq!(b, c.slice(0..16));
    assert_eq!(c, d.slice(0..32));

    let bits: BitSlice<_> = 0b101010101u64.into();
    assert_eq!(
        bits,
        bits![
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0
        ]
    );
}
#[test]
fn into_int() {
    let v = 0b101010101u64;
    let bits: BitSlice<_> = v.into();
    let x: Result<u64, _> = bits.try_into();
    assert!(x.is_ok());
    println!("{:b} {:b}", v, x.unwrap());
    assert_eq!(x.unwrap(), v);
}
fn push<S: AsMut<[u8]>, B: BitOrder, E: ByteOrder>(
    bits: &mut BitSlice<S, B, E>,
) -> Result<(), crate::Error> {
    bits.push(true)?;
    bits.push(false)?;
    bits.push(true)?;
    bits.push(true)?;
    bits.push(false)?;
    bits.push(true)?;
    bits.push(true)?;
    bits.push(false)?;
    bits.push(true)?;
    bits.push(true)?;
    bits.push(true)?;
    Ok(())
}
#[test]
fn test_push() -> Result<(), crate::Error> {
    let mut x = [0, 0, 0, 0];
    let mut bits: BitSlice<_, Msb0, BigEndian> = BitSlice::new(&mut x, 0);
    push(&mut bits)?;
    assert_eq!(&bits, &bits![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1]);
    assert_eq!(&x, &[0, 0, 0b11100000, 0b10110110]);

    let mut bits: BitSlice<_, Lsb0, BigEndian> = BitSlice::new(&mut x, 0);
    push(&mut bits)?;
    assert_eq!(&bits, &bits![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1]);
    assert_eq!(&x, &[0, 0, 0b00000111, 0b01101101]);

    let mut bits: BitSlice<_, Lsb0, LittleEndian> = BitSlice::new(&mut x, 0);
    push(&mut bits)?;
    assert_eq!(&bits, &bits![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1]);
    assert_eq!(&x, &[0b01101101, 0b00000111, 0, 0]);

    let mut bits: BitSlice<_, Msb0, LittleEndian> = BitSlice::new(&mut x, 0);
    push(&mut bits)?;
    assert_eq!(&bits, &bits![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1]);
    assert_eq!(&x, &[0b10110110, 0b11100000, 0, 0]);

    Ok(())
}
