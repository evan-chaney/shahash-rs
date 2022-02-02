use bitvec::prelude::*;
use num_bigint::BigUint;
use pretty_assertions::{assert_eq, assert_ne};
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

// SHA-160 parameters
//const R: u16 = 576; // rate
const R: u16 = 512; // rate
const D: u16 = 512; // output size
const C: u16 = 1024; // capacity
const l: u16 = 6; // used to calculate word size and number of rounds
const W: u16 = 64; //pow(2, l); // word size
const ROUNDS: u16 = 12 + (2 * l); // Number of times the permutation is run
const FINAL_PAD: [u8; 4] = 0x0000000000000028u32.to_ne_bytes();
// Offsets
// Rotation offsets
const ROTATION_OFFSETS: [[u8; 5]; 5] = [
    // x = 0
    [0, 36, 3, 41, 18],
    // x = 1
    [1, 44, 10, 45, 2],
    // x = 2
    [62, 6, 43, 15, 61],
    // x = 3
    [28, 55, 25, 21, 56],
    // x = 4
    [27, 20, 39, 8, 14],
];

const H_C: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];

#[cfg(test)]
mod tests {

    // Pull all the imports from the rest of this file
    use super::*;
    #[cfg(test)]
    use pretty_assertions::{assert_eq, assert_ne};
    #[test]
    fn verify_pad() {
        let mut v = bitvec![Lsb0, usize; 1, 1, 1, 1];
        v.resize(40, true);
        assert_eq!(pad(&mut v).len() % 512 as usize, 0 as usize);

        // Use example from IETF docs https://tools.ietf.org/html/rfc3174
        //let mut v2 = bitvec![Lsb0, usize; 0b0110000101100010011000110110010001100101u64];

        //let mut v2 =
        //    BitSlice::<Lsb0, _>::from_element(&0b0110000101100010011000110110010001100101usize)
        //        .to_bitvec();
        let mut v2 = 0b0110000101100010011000110110010001100101usize.to_ne_bytes();
        // This is what the padding function will be checked against
        let target_pad: Vec<u8> = vec![
            0x6, 0x1, 0x6, 0x2, 0x6, 0x3, 0x6, 0x4, 0x6, 0x5, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x8,
        ];
        //assert_eq!(pad_input(&mut v2), BigUint::new(target_pad).to_bytes_be());
        assert_eq!(pad_input(&mut v2), target_pad);
    }
}

// Padding output function
fn pad(n: &mut BitVec) -> &BitVec {
    // Add bit string 1{1}0* with as many zeros as it takes to become cleanly divisible by 448
    // and then a 64-bit int (2 words) to make it divisible by 512

    let mut trailing_words: BitVec<Lsb0, usize> = BitVec::new();
    // start with the first 1 bit
    if n.len() < 32 {
        trace!("n len triggered as less than 2**32");
        trace!("trailing len: {}", trailing_words.len());
        //while trailing_words.len() < (32 - n.len()) {
        //    n.push(false);
        //}
        //trailing_words.extend(&vec![false; (32 as usize - n.len()) - trailing_words.len()]);
        trailing_words.resize(32 - n.len(), false);
        trace!("Trailing words resized, new len: {}", trailing_words.len());
    }

    trailing_words.resize(
        trailing_words.len() + (512 - ((trailing_words.len() + n.len()) % 512)),
        true,
    );

    trace!("Trailing words resized, new len: {}", trailing_words.len());
    trace!(
        "Trailing words resized, new mod 512: {}",
        trailing_words.len() % 512
    );
    trace!("About to extend n, current len: {}", n.len());
    n.extend(trailing_words);
    trace!("n extended, new len: {}", n.len());
    return n;
}

fn pad_input2(n: &mut BitVec) -> &BitVec {
    let rate = 256;
    // Add bit string 10*1 with as many zeros as it takes to become cleanly divisible by rate
    // but add at least 11 if it is cleanly divisible to start

    println!("pad_input called with n length: {}", n.len());

    // start with the first 1 bit
    n.push(true);
    while (n.len() % rate as usize) < (rate as usize - 65) {
        n.push(false);
    }
    n.push(true);
    n.extend(bitvec![
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x8
    ]);
    return n;
}

fn pad_input(n: &mut [u8]) -> Vec<u8> {
    let rate = 256;
    // Add bit string 10*1 with as many zeros as it takes to become cleanly divisible by rate
    // but add at least 11 if it is cleanly divisible to start

    println!("pad_input called with n length: {}", n.len());

    let mut padding: Vec<u8> = Vec::new();
    // start with the first 1 bit
    padding.push(0x1);
    while ((n.len() + padding.len()) % rate as usize) < (rate as usize - 65) {
        padding.push(0x0);
    }
    padding.push(0x1);

    padding.extend([
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x8,
    ]);
    let mut output_vec: Vec<u8> = Vec::new();
    output_vec.extend(n.to_vec());
    output_vec.extend(padding);

    return output_vec;
}

// Permutation/state transformation function
fn permutate(a: &BitVec, word_size: u16, rate: u16, output_length: u16) {
    let mut state: Vec<Vec<BitVec>> = Vec::with_capacity(5);
    let mut r: usize = 1;
    let mut memoffset: usize = 0;
    // Do some wild seeding of the state array
    for row in 0..5 {
        state.insert(row, Vec::with_capacity(5));
        for col in 0..5 {
            state[row].insert(col, BitVec::with_capacity(word_size as usize));

            state[row][col] = a[memoffset..memoffset + word_size as usize].to_bitvec();
            memoffset += word_size as usize;
        }

        assert_eq!(state[row].len(), 5);
    }
    assert_eq!(state.len(), 5);

    // Initialize some intermediate variables

    let mut b: Vec<BitVec> = Vec::with_capacity(5);
    let mut c: Vec<BitVec> = Vec::with_capacity(5);
    let mut d: Vec<BitVec> = Vec::with_capacity(5);

    // theta
    for x in 0..5 {
        c.insert(
            x,
            state[x][0].clone()
                ^ state[x][1].clone()
                ^ state[x][2].clone()
                ^ state[x][3].clone()
                ^ state[x][4].clone(),
        );
    }
    for x in 0..5 {
        let mut rotated_vec = c[(x + 1) % 5].clone();
        rotated_vec.rotate_right(1);
        d.insert(x, c[(x + 1) % 5].clone() ^ rotated_vec)
    }
    for x in 0..5 {
        for y in 0..5 {
            state[x][y] = state[x][y].clone() ^ d[x].clone();
        }
    }
    // end theta
    // ρ (rho) & π (pi)

    // manually seed b
    for x in 0..5 {
        b.insert(x, BitVec::with_capacity(5));
    }
    //for x in 0..5 {
    //    for y in 0..5 {
    //        let mut rotated_vec = state[x][y].clone();
    //        rotated_vec.rotate_right(Rotation_offsets[x][y] as usize);
    //        //b[y].insert((3 * x + 3 * y) % 5, rotated_vec);
    //    }
    //}

    // Redo this by adapting version from official Python implementation
    let mut x: usize = 1;
    let mut y: usize = 0;
    let mut temp_x: usize;
    let mut temp_y: usize;

    let mut current: BitVec = state[x][y].clone();
    for _ in 0..24 {
        temp_x = x.clone();
        temp_y = y.clone();
        x = y;
        y = (2 * temp_x + 3 * temp_y) % 5;
        let temp_current = current.clone();
        current = state[x][y].clone();
        let mut rotated_vec = temp_current.clone();
        rotated_vec.rotate_right(ROTATION_OFFSETS[x][y] as usize);
        state[x][y] = rotated_vec;
    }
    // end p and pi
    // χ (chi)
    for y in 0..5 {
        let mut t: Vec<BitVec> = Vec::with_capacity(5);
        // Load t with data from state
        for x in 0..5 {
            t.insert(x, state[x][y].clone());
        }

        for x in 0..5 {
            state[x][y] = t[x].clone() ^ ((!t[(x + 1) % 5].clone()) & t[(x + 2) % 5].clone());
        }
    }
    // end chi
    // ι (iota) step
    for j in 0..7 {
        r = 3; //fix me
               //    state[0][0] = state[0][0] ^ r;
    }
    // end iota
}

fn main() {
    pretty_env_logger::init();
    // N is our input bit string
    let mut n = bitvec![1, 1, 0, 1, 0, 0, 1, 1];
    for _ in 0..33 {
        n.insert(0, true);
    }
    // TODO use padded_input here?
    let padded_array = pad(&mut n);
    //permutate(padded_array, W, R, D);

    //   println!("Test bitvec 32: {:?}", bitvec![32]);
    //    println!("AND: {}", &n & bitvec![0, 1, 1, 0]);
}
