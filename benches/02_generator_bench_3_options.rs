//! To run this test, you need to have nightly rust installed:
//! ```
//! rustup install nightly
//! ```
//!
//! Then run
//! ```
//! rustup run nightly cargo bench --bench 02_generator_bench_3_options
//! ```
//!
//! # Results
//!
//! test bench_generate_uciv_10000_voters_3_options ... bench: 1,995,460,854 ns/iter (+/- 111,283,644)
//! test bench_generate_uciv_1000_voters_3_options  ... bench: 197,636,889 ns/iter (+/- 9,724,723)
//! test bench_generate_uciv_100_voters_3_options   ... bench:  19,734,545 ns/iter (+/- 1,026,245)
//! test bench_generate_uciv_10_voters_3_options    ... bench:   1,956,130 ns/iter (+/- 126,083)

#![feature(test)]

extern crate test;
extern crate generator_rs;
extern crate crypto_rs;

use generator_rs::generator::Generator;
use crypto_rs::el_gamal::encryption::{PublicKey, PrivateKey};
use test::Bencher;

// See Benchmarking: https://doc.rust-lang.org/1.8.0/book/benchmark-tests.html

/// Benchmark generating UCIV info for 10 voters and 3 voting options
#[bench]
fn bench_generate_uciv_10_voters_3_options(b: &mut Bencher) {
    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();

    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
    b.iter(|| return Generator::generate_uciv(10, 3, pub_key.clone()));
}

/// Benchmark generating UCIV info for 100 voters and 3 voting options
#[bench]
fn bench_generate_uciv_100_voters_3_options(b: &mut Bencher) {
    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();

    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
    b.iter(|| return Generator::generate_uciv(100, 3, pub_key.clone()));
}

/// Benchmark generating UCIV info for 1000 voters and 3 voting options
#[bench]
fn bench_generate_uciv_1000_voters_3_options(b: &mut Bencher) {
    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();

    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
    b.iter(|| return Generator::generate_uciv(1000, 3, pub_key.clone()));
}

/// Benchmark generating UCIV info for 10000 voters and 3 voting options
#[bench]
fn bench_generate_uciv_10000_voters_3_options(b: &mut Bencher) {
    let (_, pub_key): (PrivateKey, PublicKey) = Generator::generate_keys();

    // Note: Return must be here so that the compiler will not optimize by removing the statement entirely!
    b.iter(|| return Generator::generate_uciv(10000, 3, pub_key.clone()));
}