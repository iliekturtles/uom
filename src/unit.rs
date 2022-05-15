/// Macro to implement a set of [measurement units][measurement]. Note that units manually defined
/// using this macro will not be included in the quantity unit enum or associated functions, or in
/// the `FromStr` implementation. Using this macro will create submodules for the underlying storage
/// types that are enabled (e.g. `mod f32`). `@...` match arms are considered private.
///
/// When using the pre-built [SI][si] system included with `uom` this macro allows for new units to
/// quickly be defined without requiring a release. [Pull requests][pr] to add new units upstream
/// area always greatly appreciated.
///
/// * `$system`: Path to the module where the [`system!`](macro.system.html) macro was run (e.g.
///   `uom::si`).
/// * `quantity`: Path to the module where the [`quantity!`](macro.quantity.html) macro was run
///   (e.g. `uom::si::length`).
/// * `$unit`: Unit name (e.g. `meter`, `foot`).
/// * `$conversion`: Conversion (coefficient and constant factor) from the unit to the base unit of
///   the quantity (e.g. `3.048_E-1` to convert `foot` to `meter`. `1.0_E0, 273.15_E0` to convert
///   `celsius` to `kelvin`.). The coefficient is required and the constant factor is optional.
///   Note that using a unit with a non-zero constant factor is not currently supported as a base
///   unit.
/// * `$abbreviation`: Unit abbreviation (e.g. `"m"`).
/// * `$singular`: Singular unit description (e.g. `"meter"`).
/// * `$plural`: Plural unit description (e.g. `"meters"`).
///
/// An example invocation is given below to add kilometers to length in a meter-kilogram-second
/// system. The `#[macro_use]` attribute must be used when including the `uom` crate to make the
/// `unit!` macro available.
///
/// ```
/// #[macro_use]
/// extern crate uom;
///
/// # fn main() { }
/// # mod mks {
/// #     #[macro_use]
/// #     mod length {
/// #         quantity! {
/// #             /// Length (base unit meter, m).
/// #             quantity: Length; "length";
/// #             /// Length dimension, m.
/// #             dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @meter: 1.0E0; "m", "meter", "meters";
/// #                 @foot: 3.048E-1; "ft", "foot", "feet";
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod mass {
/// #         quantity! {
/// #             /// Mass (base unit kilogram, kg).
/// #             quantity: Mass; "mass";
/// #             /// Mass dimension, kg.
/// #             dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @kilogram: 1.0; "kg", "kilogram", "kilograms";
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod time {
/// #         quantity! {
/// #             /// Time (base unit second, s).
/// #             quantity: Time; "time";
/// #             /// Time dimension, s.
/// #             dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
/// #             units {
/// #                 @second: 1.0; "s", "second", "seconds";
/// #             }
/// #         }
/// #     }
/// #     system! {
/// #         /// System of quantities, Q.
/// #         quantities: Q {
/// #             length: meter, L;
/// #             mass: kilogram, M;
/// #             time: second, T;
/// #         }
/// #         /// System of units, U.
/// #         units: U {
/// #             mod length::Length,
/// #             mod mass::Mass,
/// #             mod time::Time,
/// #         }
/// #     }
/// #     mod f32 {
/// #         Q!(crate::mks, f32/*, (centimeter, gram, second)*/);
/// #     }
/// #     mod unit {
/// unit! {
///     system: crate::mks;
///     quantity: crate::mks::length;
///
///     @kilometer: 1.0E-03; "km", "kilometer", "kilometers";
/// }
/// #     }
/// # }
/// ```
///
/// [si]: https://jcgm.bipm.org/vim/en/1.16.html
/// [measurement]: https://jcgm.bipm.org/vim/en/1.9.html
/// [pr]: https://github.com/iliekturtles/uom/pulls
#[macro_export]
macro_rules! unit {
    (
        system: $system:path;
        quantity: $quantity:path;

        $($(#[$unit_attr:meta])* @$unit:ident: $($conversion:expr),+;
            $abbreviation:expr, $singular:expr, $plural:expr;)+
    ) => {
        use $system as __system;
        use $quantity as __quantity;
        use __quantity::{Conversion, Unit};

        unit!(@units $($(#[$unit_attr])* @$unit: $($conversion),+;
            $abbreviation, $singular, $plural;)+);
    };
    (
        @units $($(#[$unit_attr:meta])* @$unit:ident: $($conversion:expr),+;
            $abbreviation:expr, $singular:expr, $plural:expr;)+
    ) => {
        $(unit!(@unit $(#[$unit_attr])* @$unit $plural);

        impl __system::Unit for $unit {
            #[inline(always)]
            fn abbreviation() -> &'static str {
                $abbreviation
            }

            #[inline(always)]
            fn singular() -> &'static str {
                $singular
            }

            #[inline(always)]
            fn plural() -> &'static str {
                $plural
            }
        }

        impl Unit for $unit {})+

        storage_types! {
            types: Float;

            $(impl $crate::Conversion<V> for super::$unit {
                type T = V;

                #[inline(always)]
                #[allow(clippy::inconsistent_digit_grouping)]
                fn coefficient() -> Self::T {
                    unit!(@coefficient $($conversion),+)
                }

                #[inline(always)]
                #[allow(unused_variables)]
                #[allow(clippy::inconsistent_digit_grouping)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    unit!(@constant op $($conversion),+)
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        storage_types! {
            types: PrimInt, BigInt;
            pub type T = $crate::num::rational::Ratio<V>;

            #[inline(always)]
            fn from_f64(value: f64) -> T {
                <T as $crate::num::FromPrimitive>::from_f64(value).unwrap()
            }

            $(impl $crate::Conversion<V> for super::$unit {
                type T = T;

                #[inline(always)]
                fn coefficient() -> Self::T {
                    from_f64(unit!(@coefficient $($conversion),+))
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    from_f64(unit!(@constant op $($conversion),+))
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        storage_types! {
            types: BigUint;
            pub type T = $crate::num::rational::Ratio<V>;

            #[inline(always)]
            fn from_f64(value: f64) -> T {
                use $crate::num::FromPrimitive;

                let c = $crate::num::rational::Ratio::<$crate::num::BigInt>::from_f64(value)
                    .unwrap();

                T::new(c.numer().to_biguint().unwrap(), c.denom().to_biguint().unwrap())
            }

            $(impl $crate::Conversion<V> for super::$unit {
                type T = T;

                #[inline(always)]
                fn coefficient() -> Self::T {
                    from_f64(unit!(@coefficient $($conversion),+))
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    from_f64(unit!(@constant op $($conversion),+))
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        storage_types! {
            types: Ratio;

            #[inline(always)]
            fn from_f64(value: f64) -> V {
                <V as $crate::num::FromPrimitive>::from_f64(value).unwrap()
            }

            $(impl $crate::Conversion<V> for super::$unit {
                type T = V;

                #[inline(always)]
                fn coefficient() -> Self::T {
                    from_f64(unit!(@coefficient $($conversion),+))
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    from_f64(unit!(@constant op $($conversion),+))
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        storage_types! {
            types: Complex;

            $(impl $crate::Conversion<V> for super::$unit {
                type T = VV;

                #[inline(always)]
                #[allow(clippy::inconsistent_digit_grouping)]
                fn coefficient() -> Self::T {
                    unit!(@coefficient $($conversion),+)
                }

                #[inline(always)]
                #[allow(unused_variables)]
                #[allow(clippy::inconsistent_digit_grouping)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    unit!(@constant op $($conversion),+)
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }
    };
    (@unit $(#[$unit_attr:meta])+ @$unit:ident $plural:expr) => {
        $(#[$unit_attr])*
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Hash)]
        pub struct $unit;
    };
    (@unit @$unit:ident $plural:expr) => {
        #[doc = $plural]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Hash)]
        pub struct $unit;
    };
    (@coefficient $factor:expr, $const:expr) => { $factor };
    (@coefficient $factor:expr) => { $factor };
    (@constant $op:ident $factor:expr, $const:expr) => { $const };
    (@constant $op:ident $factor:expr) => {
        match $op {
            $crate::ConstantOp::Add => -0.0,
            $crate::ConstantOp::Sub => 0.0,
        }
    };
}
