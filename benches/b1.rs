#![feature(test)]
extern crate test;
extern crate uom;

use test::{Bencher};

#[bench]
fn add(b: &mut Bencher) {
    let i = test::black_box(1000);
    let s = test::black_box(uom::si::f32::Length::new(0.0, uom::si::length::meter));
    let v = test::black_box(uom::si::f32::Length::new(53.5, uom::si::length::meter));

    b.iter(|| {
        (0..i).fold(s, |a, _| test::black_box(a + v))
    });
}
