use bitvec::prelude::*;

// SHA-512 parameters
const R: u16 = 576; // rate
const D: u16 = 512; // output size
const C: u16 = 1024; // capacity
const W: u16 = 64; // word size

// Padding function
fn pad(n: &mut BitVec, rate: u16) -> &BitVec {
    // Add bit string 10*1 with as many zeros as it takes to become cleanly divisible by rate
    // but add at least 11 if it is cleanly divisible to start

    // start with the first 1 bit
    n.push(true);
    while (n.len() % rate as usize) < (rate as usize - 1) {
        n.push(false);
    }
    n.push(true);
    return n;
}

// Permutation/state transformation function
fn permutate(block_width: u8, rate: u16, output_length: u16) {

    // Endian here is little-endian
    // State is a 5 x 5 x W (row, column, bit) array
}

fn main() {
    // N is our input bit string
    let mut n = bitvec![1, 1, 0, 1, 0, 0, 1, 1];
    println!("Length: {}", n.len());
    println!("Length: {}", pad(&mut n, R).len());
    //    println!("AND: {}", &n & bitvec![0, 1, 1, 0]);
}
