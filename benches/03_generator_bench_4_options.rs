//! To run this test, you need to have nightly rust installed:
//! ```
//! rustup install nightly
//! ```
//!
//! Then run
//! ```
//! rustup run nightly cargo bench --bench 03_generator_bench_4_options
//! ```
//!
//! # Results
//!
//! test bench_generate_uciv_10000_voters_4_options ... bench: 2,732,260,250 ns/iter (+/- 467,276,283)
//! test bench_generate_uciv_1000_voters_4_options ...  bench:   264,435,553 ns/iter (+/- 16,377,516)
//! test bench_generate_uciv_100_voters_4_options  ...  bench:    26,280,414 ns/iter (+/- 1,002,463)
//! test bench_generate_uciv_10_voters_4_options   ...  bench:     2,626,337 ns/iter (+/- 186,031)

#![feature(test)]

extern crate test;
extern crate generator_rs;
extern crate crypto_rs;

use generator_rs::generator::Generator;
use crypto_rs::el_gamal::encryption::{PublicKey, PrivateKey};
use test::Bencher;

// See Benchmarking: https://doc.rust-lang.org/1.8.0/book/benchmark-tests.html

/// Benchmark generating UCIV info for 10 voters and 4 voting options
//#[bench]
//fn bench_generate_uciv_10_voters_4_options(b: &mut Bencher) {
//    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();
//
//    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
//    b.iter(|| return Generator::generate_uciv(10, 4, pub_key.clone()));
//}
//
///// Benchmark generating UCIV info for 100 voters and 4 voting options
//#[bench]
//fn bench_generate_uciv_100_voters_4_options(b: &mut Bencher) {
//    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();
//
//    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
//    b.iter(|| return Generator::generate_uciv(100, 4, pub_key.clone()));
//}
//
///// Benchmark generating UCIV info for 1000 voters and 4 voting options
//#[bench]
//fn bench_generate_uciv_1000_voters_4_options(b: &mut Bencher) {
//    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();
//
//    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
//    b.iter(|| return Generator::generate_uciv(1000, 4, pub_key.clone()));
//}

/// Benchmark generating UCIV info for 10000 voters and 4 voting options
#[bench]
fn bench_generate_uciv_10000_voters_4_options(b: &mut Bencher) {
    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();

    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
    b.iter(|| return Generator::generate_uciv(10000, 4, pub_key.clone()));
}