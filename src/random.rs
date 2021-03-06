use bitvec::prelude::*;
use num::{BigUint, Integer};
use std::{fmt, num::TryFromIntError, ops::{Mul}, time::{Instant, SystemTime, UNIX_EPOCH}};

#[derive(Debug, Clone)]
pub enum Error {
    NotCoPrimeError(BigUint, BigUint),
    TryFromIntError(TryFromIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotCoPrimeError(v, p) => {
                write!(f, "Value {} is not a co-prime number with {}", v, p)
            }
            Error::TryFromIntError(error) => write!(f, "{}", error),
        }
    }
}

/// Blum Blum Shub Random Number Generator
/// # Arguments
/// * `p_val` - A prime number that will form N
/// * `q_val` - A prime number that will form N
/// * `seed`  - A number that is coprime with N. It will be X0
/// * `size`  - The bit length of the generated number (MSB = 1 Not guaranteed)
pub fn gen_blum_blum_shub(
    p_val: BigUint,
    q_val: BigUint,
    seed: BigUint,
    size: usize,
) -> Result<BigUint, Error> {
    // Assumes That P and Q are Primes
    // Check Seed - MDC (seed, p*q) = 1
    if seed.mod_floor(&p_val) == BigUint::from(0u32) {
        // Is Divisible - Not Compatible
        return Err(Error::NotCoPrimeError(seed, p_val));
    }
    if seed.mod_floor(&q_val) == BigUint::from(0u32) {
        // Is Divisible - Not Compatible
        return Err(Error::NotCoPrimeError(seed, q_val));
    }
    // Alloc Space for Generated Value
    let two_val = BigUint::from(2u32);
    // Defines N = p * q
    let n_val = p_val.mul(q_val);
    // Alloc Space for Generated Random Value
    let mut generated_number: BitVec<Lsb0, u8> = bitvec![Lsb0, u8; 0; size];
    // Initialize the Current Iteration with the Given Seed
    let mut current_iteration = seed.clone();
    // Generate First Bit
    generated_number.set(0, current_iteration.bit(0));
    // Generate Sequence
    for idx in 1..size {
        // Gen Next Step (Xn = (Xn-1)^2 mod N)
        current_iteration = current_iteration.modpow(&two_val, &n_val);
        // Gen Bit for This Number (Parity Test)
        generated_number.set(idx, current_iteration.bit(0))
    }
    // Return Generated Value
    Ok(BigUint::from_bytes_le(generated_number.as_raw_slice()))
}

/// # Linear Congruential Generator
/// For an initial value X0, computes the next value of this sequence using the
/// operation: Xn = a * (Xn-1) + C mod N
/// 
/// ## Arguments
/// * `modulus` - The `N` value
/// * `multiplier` - The `a` value
/// * `increment` - The `C` value
/// * `seed` - The `Xn-1` value
pub fn gen_linear_congruential_generator(
    modulus: BigUint,
    multiplier: BigUint,
    increment: BigUint,
    seed: BigUint
) -> Result<BigUint, Error> {

    // Alloc Final Number Data
    let mut current_iteration = seed.clone();
    // Generate Next
    current_iteration *= &multiplier;
    current_iteration += &increment;
    current_iteration = current_iteration.mod_floor(&modulus);
    // Return Generated Value
    Ok(current_iteration)
}

#[test]
fn bbs_test_100b() {
    let value = gen_blum_blum_shub(
        BigUint::from(30000000091u64),
        BigUint::from(40000000003u64),
        BigUint::from(4882516701u64),
        100,
    )
    .unwrap();
    println!("Generated Value: {}", value);
    println!("Generated Binary: {:b}", value);
}

#[test]
fn bbs_test_100b_x100() {
    for _ in 0..100 {
        gen_blum_blum_shub(
            BigUint::from(30000000091u64),
            BigUint::from(40000000003u64),
            BigUint::from(4882516701u64),
            100,
        )
        .unwrap();
    }
}

#[test]
fn bbs_test_100b_x1000() {
    for _ in 0..1000 {
        gen_blum_blum_shub(
            BigUint::from(30000000091u64),
            BigUint::from(40000000003u64),
            BigUint::from(4882516701u64),
            100,
        )
        .unwrap();
    }
}

#[test]
fn lcg_test_100b_x100() {
    let curr_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut value: BigUint = BigUint::from(curr_time.as_millis());
    for _ in 0..100 {
        value = gen_linear_congruential_generator(
            BigUint::from(2u32).pow(64),
            BigUint::from(6364136223846793005u64),
            BigUint::from(1u32),
            value
        )
        .unwrap();
        println!("{}", value)
    }
}