#![feature(test)]

extern crate test;

use std::process::Termination;

use test::Bencher;

#[bench]
fn setup(b: &mut Bencher) -> impl Termination {
    b.iter(|| {})
}
