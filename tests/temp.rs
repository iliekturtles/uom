#[cfg(feature = "notnanf32")]
mod notnan {
    use uom::si::notnanf32::*;
    use uom::num::ordered_float::NotNan;
    use uom::si::length::{kilometer, meter};

    #[test]
    fn test_not_nan32() {
        let x = Length::new::<kilometer>(NotNan(5.0));

        x.is_nan();
        x.is_finite();
        x.is_infinite();
        //x.sqrt();
        //x.cbrt();
    }

}

#[cfg(feature = "orderedf32")]
mod order32 {
use uom::si::orderedf32::*;
use uom::num::ordered_float::OrderedFloat;
use uom::si::length::{kilometer, meter};

#[test]
fn test_of32() {
    let x = Length::new::<kilometer>(OrderedFloat(5.0));
    assert_eq!(x.get::<meter>(), OrderedFloat(5000.0));

    let x2 = x * x;
    let x3 = x * x * x;

    x.is_nan();
    x.is_finite();
    x.is_infinite();
    //x2.sqrt();
    //x3.cbrt();
}

#[test]
fn foo() {
    let x = &30.0 % & 10.0;

    let a = ordered_float::OrderedFloat(30.0);
    let b = ordered_float::OrderedFloat(10.0);
    let xx = &a %  &b;

}
}