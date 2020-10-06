//! Example showing how to use the `unit!` macro to add new units to existing quantities.
//!
//! [Pull requests](https://github.com/iliekturtles/uom/pulls) for new units are always greatly
//! appreciated.

#[macro_use]
extern crate uom;

use uom::fmt::DisplayStyle::*;
use uom::si::f32::*;
use uom::si::length::meter;

unit! {
    system: uom::si;
    quantity: uom::si::length;

    @smoot: 1.702; "smoot", "smoot", "smoots";
}

fn main() {
    let l1 = Length::new::<meter>(15.0);
    let l2 = Length::new::<smoot>(1.0);

    println!(
        "{} = {}",
        l1.into_format_args(meter, Abbreviation),
        l1.into_format_args(smoot, Abbreviation)
    );
    println!(
        "{} = {}",
        l2.into_format_args(smoot, Abbreviation),
        l2.into_format_args(meter, Abbreviation)
    );
}
