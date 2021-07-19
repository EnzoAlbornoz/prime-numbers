use std::{ops::DivAssign, time::{SystemTime, UNIX_EPOCH}};

use num::{traits::Pow, BigUint, FromPrimitive, Integer};

use crate::random::{gen_blum_blum_shub, gen_linear_congruential_generator};

pub fn is_prime_miller_rabin(maybe_prime: BigUint, rounds: usize) -> bool {
    // Create Big Numbers
    let big_one: BigUint = BigUint::from(1u32);
    let big_two: BigUint = BigUint::from(2u32);
    let big_three: BigUint = BigUint::from(3u32);
    // Check 2 (Even Prime)
    if maybe_prime == big_two || maybe_prime == big_three {
        return true;
    }
    if maybe_prime.is_even() {
        return false;
    }
    // Find s = max{r in N / (2^r) % (n-1) == 0}
    let mut max_exp_that_divides = 1;
    // "d" value in algorithm (The odd part in n-1)
    let maybe_prime_minus_one = &maybe_prime - &big_one;
    let mut maybe_odd_part = &maybe_prime - &big_one;
    while maybe_odd_part.is_even() {
        max_exp_that_divides += 1;
        maybe_odd_part.div_assign(&big_two);
    }
    // Repeat a arbitrary number of rounds
    let seed = BigUint::from_i64(1726378162783618261i64).unwrap();
    for _ in 0..rounds {
        // Generate a Random Number
        let random = gen_linear_congruential_generator(
            BigUint::from_i32(2).unwrap().pow(64u32),
            BigUint::from_i64(6364136223846793005i64).unwrap(),
            BigUint::from(1u32),
            seed.clone(),
        )
        .unwrap();
        // Compute x ← a^d mod n
        let x_value = random.modpow(&maybe_odd_part, &maybe_prime);
        // Check Inconclusive
        if x_value == big_one || x_value == maybe_prime_minus_one {
            continue;
        }
        // Check With 2^r
        let mut inconclusive = false;
        for _ in 0..(max_exp_that_divides - 1) {
            let x_value = random.modpow(&big_two, &maybe_prime);
            if x_value == maybe_prime_minus_one {
                inconclusive = true;
                break;
            }
        }
        // If is inconclusive Continue, else return composed
        if !inconclusive {
            return false;
        }
    }
    // Probably Prime
    return true;
}

pub fn gen_prime_number_lcg(bit_length: usize, strongness: usize, seed: BigUint) -> BigUint {
    // Try Generate a Value
    let mut current_random = seed.clone();
    loop {
        // Generate a Random Number (With POSIX parameters)
        let random = gen_linear_congruential_generator(
            BigUint::from_i32(2).unwrap().pow(bit_length),
            BigUint::from(25214903917u64),
            BigUint::from(11u64),
            current_random.clone(),
        );
        // Continue if Error
        if random.is_ok() {
            // Check Prime
            let maybe_prime = random.unwrap();
            if is_prime_miller_rabin(maybe_prime.clone(), strongness) {
                // Return Value
                return maybe_prime;
            }
            current_random = maybe_prime;
        }
    }
}

#[test]
fn test_is_prime_miller_rabin() {
    // Test Prime for 9973
    let value = BigUint::from_i32(9973).unwrap();
    let is_prime = is_prime_miller_rabin(value.clone(), 5);
    println!("Primality Test for {}: {}", value, is_prime)
}

#[test]
fn test_gen_prime_number_lcg_64bits() {
    // Generate Prime
    let now = BigUint::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    let prime = gen_prime_number_lcg(64, 10, now);
    println!("Generated Prime: {}", prime);
}

