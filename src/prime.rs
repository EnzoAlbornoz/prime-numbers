use std::{ops::{DivAssign, Shl}, str::FromStr, thread, time::{SystemTime, UNIX_EPOCH}};

use num::{traits::Pow, BigUint, FromPrimitive, Integer};

use crate::random::gen_linear_congruential_generator;

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
    let mut max_exp_that_divides = 0;
    // "d" value in algorithm (The odd part in n-1)
    let mut maybe_odd_part = &maybe_prime - &big_one;
    while maybe_odd_part.is_even() {
        max_exp_that_divides += 1;
        maybe_odd_part.div_assign(&big_two);
    }
    // Repeat a arbitrary number of rounds
    let seed = BigUint::from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    );
    for _ in 0..rounds {
        // Generate a Random Number
        let random = gen_linear_congruential_generator(
            BigUint::from_i32(2)
                .unwrap()
                .pow(maybe_prime.bits().next_power_of_two()),
            BigUint::from_i64(6364136223846793005i64).unwrap(),
            BigUint::from(1u32),
            seed.clone(),
        )
        .unwrap()
        .clamp(big_two.clone(), &maybe_prime - &big_two);
        // Compute x ← a^d mod n
        let mut x_value = random.modpow(&maybe_odd_part, &maybe_prime);
        // Check Inconclusive
        if x_value == big_one || x_value == (&maybe_prime - &big_one) {
            continue;
        }
        // Check With 2^r
        let mut inconclusive = false;
        for _ in 0..(max_exp_that_divides - 1) {
            x_value = x_value.modpow(&big_two, &maybe_prime);
            if x_value == (&maybe_prime - &big_one) {
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

pub fn gen_prime_number_lcg_miller(bit_length: usize, strongness: usize, seed: BigUint) -> BigUint {
    // Try Generate a Value
    let mut current_random = seed.clone();
    loop {
        // Generate a Random Number (With POSIX parameters)
        let random = gen_linear_congruential_generator(
            BigUint::from_i32(2).unwrap().pow(bit_length - 1),
            BigUint::from(25214903917u64),
            BigUint::from(11u64),
            current_random.clone(),
        );
        // Continue if Error
        if random.is_ok() {
            // Check Prime
            let mut maybe_prime: BigUint = random.unwrap() >> 1;
            maybe_prime.set_bit(u64::from_usize(bit_length - 1).unwrap(), true);
            maybe_prime.set_bit(0, true);
            if is_prime_miller_rabin(maybe_prime.clone(), strongness) {
                // Return Value
                return maybe_prime;
            }
            current_random = maybe_prime;
        }
    }
}

pub fn is_prime_fermat(maybe_prime: BigUint, rounds: usize) -> bool {
    // Define Constant Values
    let big_one = BigUint::from_i32(1).unwrap();
    // Create a Initial Seed
    let mut seed = BigUint::from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    );
    // Execute an arbitrary number of tests
    for _ in 0..rounds {
        // Generate a Random number < P -> Rand mod P (Using MUSL parameters)
        let random = gen_linear_congruential_generator(
            BigUint::from_i32(2)
                .unwrap()
                .pow(maybe_prime.bits().next_power_of_two()),
            BigUint::from_i64(6364136223846793005i64).unwrap(),
            BigUint::from_i32(1).unwrap(),
            seed.clone(),
        )
        .unwrap()
        .mod_floor(&maybe_prime);
        // Update Seed
        seed = random.clone();
        // Check GDC
        if random.gcd(&maybe_prime) != big_one {
            return false;
        }
        // Check Mod
        if random.modpow(&(&maybe_prime - &big_one), &maybe_prime) != big_one {
            return false;
        }
    }
    // We don't proved that this number is not prime
    return true;
}

pub fn is_prime_fermat_parallel(maybe_prime: BigUint, rounds: usize, threads: usize) -> bool {
    // Create a Initial Seed
    let seed = BigUint::from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    );
    // Execute an arbitrary number of tests
    let threads = (0..threads).into_iter().map(|_thread_id| {
        let rounds_per_thread = rounds / threads;
        let maybe_prime = maybe_prime.clone();
        let mut seed = seed.clone();
        let big_one = BigUint::from_i32(1).unwrap();
        thread::spawn(move || {
            for _ in 0..rounds_per_thread {
                // Generate a Random number < P -> Rand mod P (Using MUSL parameters)
                let random = gen_linear_congruential_generator(
                    BigUint::from_i32(2)
                        .unwrap()
                        .pow(maybe_prime.bits().next_power_of_two()),
                    BigUint::from_i64(6364136223846793005i64).unwrap(),
                    BigUint::from_i32(1).unwrap(),
                    seed.clone(),
                )
                .unwrap()
                .mod_floor(&maybe_prime);
                // Update Seed
                seed = random.clone();
                // Check GDC
                if random.gcd(&maybe_prime) != big_one {
                    return false;
                }
                // Check Mod
                if random.modpow(&(&maybe_prime - &big_one), &maybe_prime) != big_one {
                    return false;
                }
            }
            // We don't proved that this number is not prime
            return true;
        })
    });
    // Wait Threads
    threads.map(|thread| {
        thread.join().unwrap()
    }).any(|v| v)
}

pub fn gen_prime_number_lcg_fermat(bit_length: usize, strongness: usize, seed: BigUint) -> BigUint {
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
            let mut maybe_prime: BigUint = random.unwrap() >> 1;
            maybe_prime.set_bit(u64::from_usize(bit_length - 1).unwrap(), true);
            maybe_prime.set_bit(0, true);
            if is_prime_fermat(maybe_prime.clone(), strongness) {
                // Return Value
                return maybe_prime;
            }
            current_random = maybe_prime;
        }
    }
}

pub fn gen_prime_number_lcg_fermat_parallel(bit_length: usize, strongness: usize, seed: BigUint, threads: usize) -> BigUint {
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
            if is_prime_fermat_parallel(maybe_prime.clone(), strongness, threads) {
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
fn test_gen_prime_number_lcg_miller_64bits() {
    // Generate Prime
    let now = BigUint::from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    let prime = gen_prime_number_lcg_miller(64, 10, now);
    println!("Generated Prime: {}", prime);
}

#[test]
fn test_is_prime_fermat_4096b() {
    let value = BigUint::from_str(
        &("5032108614105155921437439173060598007572804606272059045731565".to_owned()
            + "0693101422745814776681881943192812304344032158270870298941881"
            + "6285088506084110780952542129451826791391722798388151644430398"
            + "0827717928749084620399680065868086077959315957142137796590062"
            + "2480860091739142766118168466473207409136845917340718245823578"
            + "5211331076584902596668116189374109183543358706754339897233416"
            + "8855433337777974927243324153497478324925298158794414658941220"
            + "2258764718310578302531840997789158562203198976679360000244988"
            + "0251495448520816829432260316149211271847883211383038151488703"
            + "7935581711366407594986149619699304185556925012236276693571349"
            + "2909216756846033472391677268923650119990441599857011197624522"
            + "4403416160220064935525553798681545807605456355114781182993048"
            + "0300929981521347844212601094086282704002482719090858261171281"
            + "4740242883292954086344804011987706789189435928147029145454319"
            + "6151738778643556297999434751245948403910524333740247347784033"
            + "9114540844959154243819773903383326614738359417567143831561748"
            + "5020779690474010835789851621218933093421456285658900192831197"
            + "3111208769618231139156648679292489058721306888434442929011109"
            + "1269883106486781637133042440614058767377199942747257969666515"
            + "7443581950575419390533080958316659709310236517328719281051920"
            + "2026984277671"),
    )
    .unwrap();
    // Test With Fermat
    let is_prime = is_prime_fermat(value, 10);
    println!("[Fermat] Result: {}", is_prime)
}
