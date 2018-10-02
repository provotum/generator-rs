//! To run this test, you need to have nightly rust installed:
//! ```
//! rustup install nightly
//! ```
//!
//! Then run
//! ```
//! rustup run nightly cargo bench --bench 01_generator_bench_2_options
//! ```
//!
//! # Results
//!
//! test bench_generate_uciv_10000_voters_2_options ... bench: 1,387,672,461 ns/iter (+/- 440,998,160)
//! test bench_generate_uciv_1000_voters_2_options  ... bench: 134,968,584 ns/iter (+/- 7,347,127)
//! test bench_generate_uciv_100_voters_2_options   ... bench:  13,329,362 ns/iter (+/- 734,511)
//! test bench_generate_uciv_10_voters_2_options    ... bench:   1,351,005 ns/iter (+/- 178,155)

#![feature(test)]

extern crate test;
extern crate generator_rs;
extern crate crypto_rs;

use generator_rs::generator::Generator;
use crypto_rs::el_gamal::encryption::{PublicKey, PrivateKey};
use test::Bencher;

// See Benchmarking: https://doc.rust-lang.org/1.8.0/book/benchmark-tests.html

/// Benchmark generating UCIV info for 10 voters and 2 voting options
#[bench]
fn bench_generate_uciv_10_voters_2_options(b: &mut Bencher) {
    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();

    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
    b.iter(|| return Generator::generate_uciv(10, 2, pub_key.clone()));
}

/// Benchmark generating UCIV info for 100 voters and 2 voting options
#[bench]
fn bench_generate_uciv_100_voters_2_options(b: &mut Bencher) {
    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();

    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
    b.iter(|| return Generator::generate_uciv(100, 2, pub_key.clone()));
}

/// Benchmark generating UCIV info for 1000 voters and 2 voting options
#[bench]
fn bench_generate_uciv_1000_voters_2_options(b: &mut Bencher) {
    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();

    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
    b.iter(|| return Generator::generate_uciv(1000, 2, pub_key.clone()));
}

/// Benchmark generating UCIV info for 10000 voters and 2 voting options
#[bench]
fn bench_generate_uciv_10000_voters_2_options(b: &mut Bencher) {
    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();

    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
    b.iter(|| return Generator::generate_uciv(10000, 2, pub_key.clone()));
}