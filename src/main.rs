use bitvec::prelude::*;

// SHA-512 parameters
const R: u16 = 576; // rate
const D: u16 = 512; // output size
const C: u16 = 1024; // capacity
const l: u16 = 6; // used to calculate word size and number of rounds
const W: u16 = 64; //pow(2, l); // word size
const Rounds: u16 = 12 + (2 * l); // Number of times the permutation is run
#[cfg(test)]
mod tests {

    // Pull all the imports from the rest of this file
    use super::*;

    #[test]
    fn verify_pad() {
        for r in 1..1000 {
            let mut v = bitvec![1, 0, 0, 1];
            assert_eq!(pad(&mut v, r).len() % r as usize, 0 as usize);
        }
    }
}

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
fn permutate(A: BitVec, block_width: u8, rate: u16, output_length: u16) {
    // Endian here is little-endian
    // State is a 5 x 5 x W (row, column, bit) array

    // Initialize some intermediate variables

    let b: Vec<BitVec> = Vec::with_capacity(5);
    let c: Vec<BitVec> = Vec::with_capacity(5);
    let d: Vec<BitVec> = Vec::with_capacity(5);
    //let mut b: BitVec<LocalBits, usize> = BitVec::with_capacity(64); // is 64 the right number here???
    //let mut c: BitVec<LocalBits, usize> = BitVec::with_capacity(64);
    //let mut d: BitVec<LocalBits, usize> = BitVec::with_capacity(64);

    for x in 0..4 {
        c[x] = A[x][0]
    }
}

fn main() {
    // N is our input bit string
    let mut n = bitvec![1, 1, 0, 1, 0, 0, 1, 1];

    println!("Length: {}", n.len());
    println!("Length: {}", pad(&mut n, R).len());
    //    println!("AND: {}", &n & bitvec![0, 1, 1, 0]);
}
