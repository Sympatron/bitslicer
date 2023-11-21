# bitslicer

This crate provides utilities for bit-level operations on data. It is designed to offer flexible and efficient ways to manipulate bits in various storage formats. The primary focus of this crate is to allow detailed control over bit-level data and to cater to scenarios where such control is crucial.

## Features

- **Bit Order Handling**: Support for different bit ordering (e.g., MSB-first, LSB-first), allowing users to specify how bits are read from and written to the underlying storage.
- **Byte Order Handling**: Support for different byte endianness (e.g., little endian, big endian), enabling interpretation of byte sequences according to the specified byte order.
- **`BitSlice` Structure**: The primary feature of this crate, `BitSlice` provides a view into a sequence of bits, supporting operations like reading a bit at a specific index, slicing a range of bits, and setting the value of a bit. `BitSlice` is flexible in terms of the underlying storage and can be parameterized with different bit and byte orders.
- **[`BitIter`] Iterator**: An iterator over the bits in a `BitSlice`, offering both read and write capabilities for individual bits.
- **Macros for Convenience**: Macros like [`bits!`] to facilitate easy and concise creation of `BitSlice` instances from literal sequences of bits.

### Optional `alloc` Feature

Enabling the `alloc` feature adds:
- Conversion of `BitSlice` to a bit string (e.g., "1010110").
- Implementation of the `Debug` trait for `BitSlice`.

## Example

```rust
use bitslicer::{BitSlice, LittleEndian, Lsb0, bits};

let mut data = [0b01010101u8, 0b11110010];
let mut bit_slice: BitSlice<_, Lsb0, LittleEndian> = data.as_mut().into();

// Iterate over bits
for bit in bit_slice.iter() {
    println!("{}", bit);
}

// Use bit_slice to access and manipulate bits...
assert_eq!(bit_slice.get_bit(2), true);
bit_slice.set_bit(2, false);
assert_eq!(bit_slice.get_bit(2), false);

let sub_slice = bit_slice.slice(1..10);
assert!(sub_slice == bits![0, 0, 0, 1, 0, 1, 0, 0, 1]);
```

Installation
------------
Add `bitslicer` to your `Cargo.toml`:

```toml
[dependencies]
bitslicer =  {version: "0.1.0", git = "https://github.com/Sympatron/bitslicer.git"}
```

Contribution
------------

Contributions to `bitslicer` are welcome! Please feel free to submit issues and pull requests to the repository.

License
-------

This crate is licensed under [BSD-3-Clause](LICENSE).