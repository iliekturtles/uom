#![feature(test)] 
extern crate test;

use test::{Bencher};

#[bench]
fn add(b: &mut Bencher) {
    let i = test::black_box(1000);
    let s = test::black_box(0.0_f32);
    let v = test::black_box(53.5_f32);

    b.iter(|| {
        (0..i).fold(s, |a, _| test::black_box(a + v))
    });
}
