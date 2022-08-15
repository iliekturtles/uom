//! Tests summarizing which binary operations are implemented.
//!
//! This includes:
//!
//! + the operations implemented using the `impl_ops!` macro.
//! + the comparison operators: `==`, `<`, `>`, `<=` and `>=`.
//!
//! The purpose is not to test the implementations exhaustively, but rather to
//! give a clear overview of which ones are present or absent, depending on
//! factors like:
//!
//! + `autoconvert` being enabled or disabled
//! + whether the operands are values or refs

// Helper functions for constructing a variety of Quantities without excessive
// syntactic noise.
#[allow(unused_macros)]
mod concise {

    // Bring the units used by the constructors into a limited scope
    mod units {
        pub use crate::si::{
            area::square_meter,
            frequency::hertz,
            length::meter,
            ratio::ratio,
            time::second,
            velocity::meter_per_second,
        };
    }

    // Define a set of quantities with different base units, for testing of `autoconvert`
    pub mod cgs {
        ISQ!(crate::si, f32, (centimeter, gram, second, ampere, kelvin, mole, candela));
    }

    /// Generate a module containing concise constructors for Quantities based
    /// on a given storage type, in order to allow writing pithy tests
    macro_rules! constructors {
        ($storage_type:ident $($function:item)*) => {
            #[allow(unused)]
            pub mod $storage_type {
                use crate::si::$storage_type as si;
                use crate::tests::impl_ops::concise::cgs;
                use crate::tests::impl_ops::concise::units::*;
                type T = $storage_type;
                $($function)*
            }
        };
    }

    /// Generate a function called NAME which returns QUANTITY by interpreting
    /// its argument as UNIT
    ///
    /// wrap!(NAME(QUANTITY) UNIT);
    macro_rules! wrap {
        ($name:ident ($quantity:ty) $unit:ident ) => {
            pub fn $name(x: T) -> $quantity {
                <$quantity>::new::<$unit>(x)
            }
        };
    }

    // Use the above macros to generate terse Quantity constructors.
    #[cfg(feature = "f32")]
    constructors! { f32
        wrap!(      m( si::Length)   meter);
        wrap!(  cgs_m(cgs::Length)   meter);
        wrap!(      s( si::Time)     second);
        wrap!(    one( si::Ratio)    ratio);
        wrap!(cgs_one(cgs::Ratio)    ratio);
        wrap!(     m2( si::Area)     square_meter);
        wrap!(    mps( si::Velocity) meter_per_second);
        wrap!(     hz(si::Frequency) hertz);
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
/// Syntactically these look like
///
/// 1. test!(TEST_NAME   LHS   OPERATOR RHS, EXPECTED);
/// 2. test!(TEST_NAME \[ LHS \] OPERATOR RHS, EXPECTED);
///
/// In other words, the side-effect test is distinguished from the expression
/// test by wrapping the left operand (the one which will be mutated by the
/// operator) in square brackets.
///
/// Additionally, some tests should only be run when the `autoconvert` features
/// is enabled. These are identified by using the `ac` token at the start of the
/// input to `test!(...)`:
///
/// 1. Run test always:                        `test!(   ...)`
/// 2. Run test only when `autoconvert` enabled: `test!(ac ...)`
///
/// Furthermore, the `ac` variant of `test!` appends `_autoconvert` to
/// TEST_NAME, in order to distinguish automatically between tests which require
/// autoconvert and those that don't.
#[allow(unused_macros)]
macro_rules! test {
    // The first two arms (with `ac` as the first input token) forward the
    // request to the last two arms, after injecting two changes:
    // 1. add `#[cfg(feature = "autoconvert")]`
    // 2. append `_autoconvert` to the test name
    (ac $test_name:ident [$lhs:expr] $op:tt $rhs:expr, $expected:expr) => {
        paste::paste! {
            #[cfg(feature = "autoconvert")]
            test!([<$test_name _ autoconvert>] [$lhs] $op $rhs, $expected);
        }
    };
    (ac $test_name:ident $lhs:expr, $rhs:expr) => {
        paste::paste! {
            #[cfg(feature = "autoconvert")]
            test!([<$test_name _ autoconvert>] $lhs, $rhs);
        }
    };
    // ----------------------------------------------------------------
    // test assignment operations:  let mut l; l += r
    ($test_name:ident [$lhs:expr] $op:tt $rhs:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let mut    x =   $lhs;
                       x $op $rhs;
            assert_eq!(x, $expected);
        }
    };
    // test binary operator expressions: l + r
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
    use super::concise::f32::*;
    //#[cfg(feature = "autoconvert")]

    // NOTE: tests labelled with `ac` just inside `test!(` require the
    // `autoconvert` feature to be enabled.

    test!(ac add                   m(1.)   +    cgs_m(2.),   m(3.)); // 1
    test!(ac sub                   m(8.)   -    cgs_m(2.),   m(6.)); // 1
    test!(   add                   m(1.)   +        m(2.),   m(3.)); //  2
    test!(   sub                   m(8.)   -        m(2.),   m(6.)); //  2
    test!(ac add_assign          [ m(1.) ] +=   cgs_m(2.),   m(3.)); //   3
    test!(ac sub_assign          [ m(8.) ] -=   cgs_m(2.),   m(6.)); //   3
    test!(   add_assign          [ m(1.) ] +=       m(2.),   m(3.)); //    4
    test!(   sub_assign          [ m(8.) ] -=       m(2.),   m(6.)); //    4
    test!(ac mul                   m(4.)   *    cgs_m(2.),  m2(8.)); //     5
    test!(ac div                   m(6.)   /    cgs_m(2.), one(3.)); //     5
    test!(   mul                   m(4.)   *        m(2.),  m2(8.)); //      6
    test!(   div                   m(6.)   /        s(2.), mps(3.)); //      6
    test!(   mul_ratio_left      one(4.)   *        m(2.),   m(8.)); //      6
    test!(   div_ratio_left      one(6.)   /        s(2.),  hz(3.)); //      6
    test!(   mul_ratio_right       m(4.)   *      one(2.),   m(8.)); //      6    c.f. AAA below
    test!(   div_ratio_right       m(6.)   /      one(2.),   m(3.)); //      6    c.f. BBB below
    test!(ac mul_ratio_right       m(4.)   *  cgs_one(2.),   m(8.)); //      6    c.f. CCC below
    test!(ac div_ratio_right       m(6.)   /  cgs_one(2.),   m(3.)); //      6    c.f. DDD below
 // test!(   mul_assign_ratio    [ m(4.) ] *=     one(2.),   m(8.)); // ERR       c.f. AAA above
 // test!(   div_assign_ratio    [ m(6.) ] /=     one(2.),   m(3.)); // ERR       c.f. BBB above
 // test!(ac mul_assign_ratio    [ m(4.) ] *= cgs_one(2.),   m(8.)); // ERR       c.f. CCC above
 // test!(ac div_assign_ratio    [ m(6.) ] /= cgs_one(2.),   m(3.)); // ERR       c.f. DDD above
    test!(   mul_bare_right        m(4.)   *          2. ,   m(8.)); //       7
    test!(   div_bare_right        m(6.)   /          2. ,   m(3.)); //       7
    test!(   mul_assign_bare     [ m(2.) ] *=         3. ,   m(6.)); //        8
    test!(   div_assign_bare     [ m(6.) ] /=         3. ,   m(2.)); //        8
    test!(   mul_bare_left           4.    *        m(2.),   m(8.)); //         9
    test!(   div_bare_left           6.    /        s(2.),  hz(3.)); //         9

    test!(   eq                    m(1.)   ==       m(2.),   false);
    test!(ac eq                    m(1.)   ==   cgs_m(2.),   false);
    test!(   lt                    m(1.)   <        m(2.),   true );
    test!(ac lt                    m(1.)   <    cgs_m(2.),   true );
    test!(   gt                    m(1.)   >        m(2.),   false);
    test!(ac gt                    m(1.)   >    cgs_m(2.),   false);
    test!(   le                    m(1.)   <=       m(2.),   true );
    test!(ac le                    m(1.)   <=   cgs_m(2.),   true );
    test!(   ge                    m(1.)   >=       m(2.),   false);
    test!(ac ge                    m(1.)   >=   cgs_m(2.),   false);
}
