use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use num::{BigUint, FromPrimitive};

use crate::prime::gen_prime_number_lcg;
pub mod prime;
pub mod random;

fn main() {
    const BENCH_BBS: bool = false;
    const BENCH_LCG: bool = false;
    const BENCH_PRIME_LCG: bool = true;


    // // Generate Prime
    // let now = BigUint::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    // let prime = gen_prime_number_lcg(2048, 1000, now);
    // println!("Generated Prime: {}", prime);

    if BENCH_BBS {
        println!("BlumBlum Shub Benchmark:");
        let sizes: Vec<usize> = vec![40, 56, 80, 128, 168, 224, 256, 512, 1024, 2048, 4096];
        for size in sizes {
            let initial_time = Instant::now();
            let mut size_time = Duration::new(0, 0);
            for _ in 0..500 {
                let time_loop_init = Instant::now();
                random::gen_blum_blum_shub(
                    BigUint::from(30000000091u64),
                    BigUint::from(40000000003u64),
                    BigUint::from(4882516701u64),
                    size,
                )
                .unwrap();
                size_time += time_loop_init.elapsed();
            }
            let elapsed = initial_time.elapsed();
            let elapsed_avg = size_time.checked_div(500).unwrap();
            println!("[BBS] [{} B] \tElapsed: {}.{}s \t- Avg: {}.{}µs", size, elapsed.as_secs(), elapsed.subsec_millis(), elapsed_avg.as_micros(), elapsed_avg.subsec_nanos());
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
                    BigUint::from_i32(2).unwrap().pow(u32::from_usize(size).unwrap()),
                    BigUint::from(30000000091u64),
                    BigUint::from(40000000003u64),
                    seed_value.clone()
                )
                .unwrap();
                seed_value = random;
                size_time += time_loop_init.elapsed();
            }
            let elapsed = initial_time.elapsed();
            let elapsed_avg = size_time.checked_div(500).unwrap();
            println!("[LCGB] [{} B]\tElapsed: {}.{}s \t- Avg: {}.{}µs", size, elapsed.as_secs(), elapsed.subsec_millis(), elapsed_avg.as_micros(), elapsed_avg.subsec_nanos());
        }
    }
    if BENCH_PRIME_LCG {
        println!("Prime Generator With Linear Congruent Generator Benchmark:");
        let sizes: Vec<usize> = vec![40, 56, 80, 128, 168, 224, 256, 512, 1024, 2048];
        for size in sizes {
            let initial_time = Instant::now();
            let mut size_time = Duration::new(0, 0);
            for _ in 0..20 {
                let time_loop_init = Instant::now();
                let now = BigUint::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
                gen_prime_number_lcg(size, 10, now);
                size_time += time_loop_init.elapsed();
            }
            let elapsed = initial_time.elapsed();
            let elapsed_avg = size_time.checked_div(500).unwrap();
            println!("[PG] [LCG] [{} B]\tElapsed: {}.{}s\t- Avg: {}.{}µs", size, elapsed.as_secs(), elapsed.subsec_millis(), elapsed_avg.as_micros(), elapsed_avg.subsec_nanos());
        }
    }
}
