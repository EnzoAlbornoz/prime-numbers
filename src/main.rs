use std::{
    str::FromStr,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use num::{BigUint, FromPrimitive};

use crate::prime::{
    gen_prime_number_lcg_fermat, gen_prime_number_lcg_fermat_parallel, gen_prime_number_lcg_miller,
    is_prime_fermat,
};
pub mod prime;
pub mod random;

fn main() {
    const BENCH_BBS: bool = false;
    const BENCH_LCG: bool = false;
    const BENCH_PRIME_LCG_MILLER: bool = true;
    const BENCH_PRIME_LCG_FERMAT: bool = true;

    // Generate Prime
    // let child = thread::spawn(move || {
    // let time = Instant::now();
    // let now = BigUint::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    // let prime = gen_prime_number_lcg_fermat(4096, 200, now);
    // let time = time.elapsed();
    // println!("[Fermat] Generated Prime: {} \nin {}.{}s", prime, time.as_secs(), time.subsec_millis());
    // });
    // let child2 = thread::spawn(move || {
    // let time = Instant::now();
    // let now = BigUint::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    // let prime = gen_prime_number_lcg_miller(4096, 200, now);
    // let time = time.elapsed();
    // println!("[MILLER] Generated Prime: {} in \n{}.{}s - Minimum Bits: {}", prime, time.as_secs(), time.subsec_millis(), prime.bits());
    // });
    // let child3 = thread::spawn(move || {
    // let time = Instant::now();
    // let now = BigUint::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    // let prime = gen_prime_number_lcg_fermat_parallel(4096, 200, now, 4);
    // let time = time.elapsed();
    // println!("[FermatP] Generated Prime: {} in \n{}.{}s", prime, time.as_secs(), time.subsec_millis());
    // });

    // child.join().unwrap();
    // // child2.join().unwrap();
    // child3.join().unwrap();

    if BENCH_BBS {
        println!("BlumBlum Shub Benchmark:");
        let sizes: Vec<usize> = vec![40, 56, 80, 128, 168, 224, 256, 512, 1024, 2048, 4096];
        for size in sizes {
            let initial_time = Instant::now();
            let mut size_time = Duration::new(0, 0);
            let mut seed_value = BigUint::from(4882516701u64);
            for _ in 0..500 {
                let time_loop_init = Instant::now();
                let random = random::gen_blum_blum_shub(
                    BigUint::from(30000000091u64),
                    BigUint::from(40000000003u64),
                    seed_value.clone(),
                    size,
                )
                .unwrap();
                seed_value = random;
                size_time += time_loop_init.elapsed();
            }
            let elapsed = initial_time.elapsed();
            let elapsed_avg = size_time.checked_div(500).unwrap();
            println!(
                "[BBS] [{} B] \tElapsed: {}.{}s \t- Avg: {}.{}µs",
                size,
                elapsed.as_secs(),
                elapsed.subsec_millis(),
                elapsed_avg.as_micros(),
                elapsed_avg.subsec_nanos()
            );
        }
    }
    if BENCH_LCG {
        println!("Linear Congruent Generator Benchmark:");
        let sizes: Vec<usize> = vec![40, 56, 80, 128, 168, 224, 256, 512, 1024, 2048, 4096];
        for size in sizes {
            let initial_time = Instant::now();
            let mut size_time = Duration::new(0, 0);
            let mut seed_value = BigUint::from(4882516701u64);
            for _ in 0..500 {
                let time_loop_init = Instant::now();
                let random = random::gen_linear_congruential_generator(
                    BigUint::from_i32(2)
                        .unwrap()
                        .pow(u32::from_usize(size).unwrap()),
                    BigUint::from(30000000091u64),
                    BigUint::from(40000000003u64),
                    seed_value.clone(),
                )
                .unwrap();
                seed_value = random;
                size_time += time_loop_init.elapsed();
            }
            let elapsed = initial_time.elapsed();
            let elapsed_avg = size_time.checked_div(500).unwrap();
            println!(
                "[LCGB] [{} B]\tElapsed: {}.{}s \t- Avg: {}.{}µs",
                size,
                elapsed.as_secs(),
                elapsed.subsec_millis(),
                elapsed_avg.as_micros(),
                elapsed_avg.subsec_nanos()
            );
        }
    }
    if BENCH_PRIME_LCG_MILLER {
        println!("Prime Generator With Linear Congruent Generator Benchmark:");
        let sizes: Vec<usize> = vec![40, 56, 80, 128, 168, 224, 256, 512, 1024, 2048, 4096];
        for size in sizes {
            let initial_time = Instant::now();
            let mut size_time = Duration::new(0, 0);
            for _ in 0..1 {
                let time_loop_init = Instant::now();
                let now = BigUint::from(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                );
                gen_prime_number_lcg_miller(size, 200, now);
                size_time += time_loop_init.elapsed();
            }
            let elapsed = initial_time.elapsed();
            let elapsed_avg = size_time.checked_div(1).unwrap();
            println!(
                "[PG] [LCG] [MILLER] [{} B]\tElapsed: {}.{}s   \t- Avg: {}.{}ms",
                size,
                elapsed.as_secs(),
                elapsed.subsec_millis(),
                elapsed_avg.as_millis(),
                elapsed_avg.subsec_micros()
            );
        }
    }
    if BENCH_PRIME_LCG_FERMAT {
        println!("Prime Generator With Linear Congruent Generator Benchmark:");
        let sizes: Vec<usize> = vec![40, 56, 80, 128, 168, 224, 256, 512, 1024, 2048, 4096];
        for size in sizes {
            let initial_time = Instant::now();
            let mut size_time = Duration::new(0, 0);
            for _ in 0..1 {
                let time_loop_init = Instant::now();
                let now = BigUint::from(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                );
                gen_prime_number_lcg_fermat(size, 200, now);
                size_time += time_loop_init.elapsed();
            }
            let elapsed = initial_time.elapsed();
            let elapsed_avg = size_time.checked_div(1).unwrap();
            println!(
                "[PG] [LCG] [FERMAT] [{} B]\tElapsed: {}.{}s   \t- Avg: {}.{}ms",
                size,
                elapsed.as_secs(),
                elapsed.subsec_millis(),
                elapsed_avg.as_millis(),
                elapsed_avg.subsec_micros()
            );
        }
    }
}
