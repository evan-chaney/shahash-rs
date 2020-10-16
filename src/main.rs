use bitvec::prelude::*;

// SHA-512 parameters
const R: u16 = 576; // rate
const D: u16 = 512; // output size
const C: u16 = 1024; // capacity
const l: u16 = 6; // used to calculate word size and number of rounds
const W: u16 = 64; //pow(2, l); // word size
const ROUNDS: u16 = 12 + (2 * l); // Number of times the permutation is run

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
fn permutate(a: &BitVec, word_size: u16, rate: u16, output_length: u16) {
    // Endian here is little-endian
    // State is a 5 x 5 x W (row, column, bit) array
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
    for t in 0..24 {
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
    // N is our input bit string
    let mut n = bitvec![1, 1, 0, 1, 0, 0, 1, 1];
    for x in 0..4 {
        //        println!("My num: {}", x);
    }
    for x in 0..2000 {
        n.insert(0, true);
    }
    let mut padded_array = pad(&mut n, R);
    permutate(padded_array, W, R, D);

    //   println!("Test bitvec 32: {:?}", bitvec![32]);
    //    println!("AND: {}", &n & bitvec![0, 1, 1, 0]);
}
