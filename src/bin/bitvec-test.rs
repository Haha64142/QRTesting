use bitvec::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let text: &str = "Hello";
    // let capacity = 13 * 8;
    // assert!(capacity % 8 == 0);
    //
    // let mut bits: BitVec<u8, Msb0> = BitVec::new();
    // bits.reserve_exact(capacity);
    // println!("{bits:?}");
    //
    // let mode: u8 = 0b0100;
    // bits.extend_from_bitslice(&mode.view_bits::<Msb0>()[4..8]);
    // println!("{bits:?}");
    //
    // let length = text.len() as u8;
    // bits.extend_from_bitslice(&length.view_bits::<Msb0>());
    // println!("{bits:?}");
    //
    // bits.extend_from_bitslice(&text.as_bytes().view_bits::<Msb0>());
    // println!("{bits:?}");
    //
    // assert!(capacity >= bits.len());
    // let amount = std::cmp::min(4, capacity - bits.len());
    // bits.resize(bits.len() + amount, false);
    // println!("{bits:?}");
    //
    // let amount = (8 - bits.len() % 8) % 8;
    // bits.resize(bits.len() + amount, false);
    // println!("{bits:?}");
    //
    // assert!(bits.len() % 8 == 0);
    // let amount = (capacity - bits.len()) / 8;
    // bits.extend([0xECu8, 0x11].into_iter().cycle().take(amount));
    // println!("{bits:?}");
    //
    // println!("{:?}", bits.into_vec());

    let text = "Hello";
    println!("{:?}", data_from_bytes(text.as_bytes(), 13, 8));

    Ok(())
}

fn data_from_bytes(
    bytes: &[u8],
    data_codewords: u16,
    count_length: u8,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let capacity = data_codewords as usize * 8;

    let mut bits: BitVec<u8, Msb0> = BitVec::new();
    bits.reserve_exact(capacity);

    let mode: u8 = 0b0100;
    bits.extend_from_bitslice(&mode.view_bits::<Msb0>()[4..8]);

    let len = bytes.len();
    let count = len.view_bits::<Msb0>();
    bits.extend_from_bitslice(&count[count.len().saturating_sub(count_length as usize)..]);

    bits.extend_from_bitslice(&bytes.view_bits::<Msb0>());

    assert!(capacity >= bits.len());
    let amount = std::cmp::min(4, capacity - bits.len());
    bits.resize(bits.len() + amount, false);

    let amount = (8 - bits.len() % 8) % 8;
    bits.resize(bits.len() + amount, false);

    assert!(bits.len() % 8 == 0);
    let amount = (capacity - bits.len()) / 8;
    bits.extend([0xECu8, 0x11].into_iter().cycle().take(amount));

    Ok(bits.into_vec())
}
