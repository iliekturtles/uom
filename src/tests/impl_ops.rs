//! Tests for the operations implemented using the `impl_ops!` macro.

// Helper functions for constructing a variety of Quantities without excessive
// syntactic noise.
#[allow(unused_macros)]
mod concise {

    // Bring the units used by the constructors into a limited scope
    mod units {
        pub use crate::si::{
            area::{square_kilometer, square_meter},
            frequency::hertz,
            length::{kilometer, meter},
            mass::{gram, kilogram},
            ratio::{ratio, percent},
            time::second,
            velocity::meter_per_second,
        };
    }

    /// Generate a module containing concise constructors for Quantities based
    /// on a given storage type
    macro_rules! constructors {
        ($storage_type:ident $($function:item)*) => {
            #[allow(unused)]
            pub mod $storage_type {
                use crate::si::$storage_type::{
                    Area, Frequency, Length, Mass, Ratio, Time, Velocity
                };
                use super::units::*;
                type T = $storage_type;
                $($function)*
            }
        };
    }

    /// Generate a function called NAME which returns QUANTITY by interpreting
    /// its argument as UNIT
    ///
    /// wrap!(NAME QUANTITY UNIT);
    macro_rules! wrap {
        ($name:ident $quantity:ident $unit:ident ) => {
            pub fn $name(x: T) -> $quantity {
                $quantity::new::<$unit>(x)
            }
        };
    }

    #[cfg(feature = "f32")]
    constructors! { f32
        wrap!(      m Length              meter);
        wrap!(     km Length          kilometer);
        wrap!(      g Mass                 gram);
        wrap!(     kg Mass             kilogram);
        wrap!(      s Time               second);
        wrap!(  ratio Ratio               ratio);
        wrap!(    pct Ratio             percent);
        wrap!(     m2 Area         square_meter);
        wrap!(    km2 Area     square_kilometer);
        wrap!(m_per_s Velocity meter_per_second);
        wrap!(     hz Frequency           hertz);
    }
}

// The tests will come in 4 varieties: `vv`, `vr`, `rv` & `rr`. with `r` and `v`
// standing for 'value' and 'reference', describing the left and right operands
// of the operator being tested.

/// Generate a test that checks the result of a binary operation
///
/// There are two different kinds of tests:
///
/// 1. verify the result of a binary operation expression
/// 2. verify the side effect of an assignment operator on the left operand
///
/// 1. test!(TEST_NAME   LHS   OPERATOR RHS, EXPECTED);
/// 2. test!(TEST_NAME [ LHS ] OPERATOR RHS, EXPECTED);
///
/// In other words, the side-effect test is distinguished from the expression
/// test, by wrapping the left operand (the one which will be mutated by the
/// operator) in square brackets.
#[allow(unused_macros)]
macro_rules! test {
    ($test_name:ident [$lhs:expr] $op:tt $rhs:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let mut    x =   $lhs;
                       x $op $rhs;
            assert_eq!(x, $expected);
        }
    };
    ($test_name:ident $lhs:expr, $rhs:expr) => {
        #[test]
        fn $test_name() {
            assert_eq!($lhs, $rhs);
        }
    };
}

// The number in the comment after each test indicates which implementation
// provides the functionality being tested, according to the table shown here:
// https://github.com/iliekturtles/uom/pull/307#issuecomment-1186208970
#[rustfmt::skip]
#[cfg(feature = "f32")]
mod vv {
    mod autoconvert_no {
        use super::super::concise::f32::*;
        test!(add_conv              m(1000.)   +     km(2.),     m(3000.)); // 1
        test!(sub_conv              m(8000.)   -     km(2.),     m(6000.)); // 1
        test!(add                   m(   1.)   +      m(2.),        m(3.)); //  2
        test!(sub                   m(   8.)   -      m(2.),        m(6.)); //  2
        test!(add_assign_conv     [ m(1000.) ] +=    km(2.),       km(3.)); //   3
        test!(sub_assign_conv     [ m(8000.) ] -=    km(2.),       km(6.)); //   3
        test!(add_assign          [ m(   1.) ] +=     m(2.),        m(3.)); //    4
        test!(sub_assign          [ m(   8.) ] -=     m(2.),        m(6.)); //    4
        test!(mul_conv              m(4000.)   *     km(2.),      km2(8.)); //     5
        test!(div_conv              m(6000.)   /     km(2.),    ratio(3.)); //     5
        test!(mul                      m(4.)   *      m(2.),       m2(8.)); //      6
        test!(div                      m(6.)   /      s(2.),  m_per_s(3.)); //      6
        test!(mul_ratio_left       ratio(4.)   *      m(2.),        m(8.)); //      6
        test!(div_ratio_left       ratio(6.)   /      s(2.),       hz(3.)); //      6
        test!(mul_ratio_right          m(4.)   *  ratio(2.),        m(8.)); //      6    c.f. AAA below
        test!(div_ratio_right          m(6.)   /  ratio(2.),        m(3.)); //      6    c.f. BBB below
        test!(mul_ratio_right_p        m(4.)   *   pct(50.),        m(2.)); //      6    c.f. CCC below
        test!(div_ratio_right_p        m(4.)   /   pct(50.),        m(8.)); //      6    c.f. DDD below
        test!(mul_assign_ratio    [    m(4.) ] *= ratio(2.),        m(8.)); //  ERR      c.f. AAA above
        test!(div_assign_ratio    [    m(6.) ] /= ratio(2.),        m(3.)); //  ERR      c.f. BBB above
        test!(mul_assign_ratio_p  [    m(4.) ] *=  pct(50.),        m(4.)); //  ERR      c.f. CCC above
        test!(div_assign_ratio_p  [    m(4.) ] /=  pct(50.),        m(8.)); //  ERR      c.f. DDD above
        test!(mul_bare_right           m(4.)   *        2. ,        m(8.)); //       7
        test!(div_bare_right           m(6.)   /        2. ,        m(3.)); //       7
        test!(mul_assign_bare     [    m(2.) ] *=       3. ,        m(6.)); //        8
        test!(div_assign_bare     [    m(6.) ] /=       3. ,        m(2.)); //        8
        test!(mul_bare_left              4.    *      m(2.),        m(8.)); //         9
        test!(div_bare_left              6.    /      s(2.),       hz(3.)); //         9
    }
}
