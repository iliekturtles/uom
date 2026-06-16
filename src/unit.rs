/// Macro to implement a set of [measurement units][measurement]. Note that units manually defined
/// using this macro will not be included in the quantity unit enum or associated functions, or in
/// the `FromStr` implementation. Using this macro will create submodules for the underlying storage
/// types that are enabled (e.g. `mod f32`).
///
/// When using the pre-built [SI][si] system included with `uom` this macro allows for new units to
/// quickly be defined without requiring a release. [Pull requests][pr] to add new units upstream
/// area always greatly appreciated.
///
/// * `$system`: Path to the module where the [`system!`] macro was run (e.g.
///   `uom::si`).
/// * `quantity`: Path to the module where the [`quantity!`] macro was run
///   (e.g. `uom::si::length`).
/// * `$unit`: Unit name (e.g. `meter`, `foot`).
/// * `$coefficient`: Conversion coefficient from the unit to the base unit of the quantity (e.g.
///   `3.048_E-1` to convert `foot` to `meter`, `1.0_E0` to convert `celsius` to `kelvin`).
/// * `$constant`: Optional conversion constant factor from the unit to the base unit of the
///   quantity (e.g. `273.15_E0` to convert `celsius` to `kelvin`). Note that using a unit with a
///   non-zero constant factor is not currently supported as a base unit.
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

        $($(#[$unit_attr:meta])* @$unit:ident: $coefficient:expr $(, $constant:expr)?;
            $abbreviation:expr, $singular:expr, $plural:expr;)+
    ) => {
        use $system as __system;
        use $quantity as __quantity;
        use __quantity::{Conversion, Unit};

        unit_units!($($(#[$unit_attr])* @$unit: $coefficient $(, $constant)?;
            $abbreviation, $singular, $plural;)+);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! unit_units {
    (
        $($(#[$unit_attr:meta])* @$unit:ident: $coefficient:expr $(, $constant:expr)?;
            $abbreviation:expr, $singular:expr, $plural:expr;)+
    ) => {
        $(unit_unit!($(#[$unit_attr])* @$unit $plural);

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
                #[allow(clippy::eq_op)]
                #[allow(clippy::approx_constant)]
                fn coefficient() -> Self::T {
                    $coefficient
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    unit_constant!(op $($constant)?)
                }
            }

            impl super::Conversion<V> for super::$unit {
                test! {
                #[inline(always)]
                #[allow(clippy::eq_op)]
                #[allow(clippy::approx_constant)]
                fn is_valid() -> bool {
                    use $crate::num::ToPrimitive;

                    let r = Some($coefficient);
                    let c = <Self as $crate::Conversion<V>>::coefficient().to_f64();

                    r == c
                }}
            })+
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
                #[allow(clippy::eq_op)]
                #[allow(clippy::approx_constant)]
                fn coefficient() -> Self::T {
                    from_f64($coefficient)
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    from_f64(unit_constant!(op $($constant)?))
                }
            }

            impl super::Conversion<V> for super::$unit {
                test! {
                #[inline(always)]
                #[allow(clippy::eq_op)]
                #[allow(clippy::approx_constant)]
                fn is_valid() -> bool {
                    use $crate::num::{FromPrimitive, ToPrimitive};

                    if let Some(conversion) = Self::T::from_f64($coefficient) {
                        // Fractional conversion factors will end up being truncated.
                        if conversion.numer() >= conversion.denom() {
                            if let Some(numer) = conversion.numer().to_f64() {
                                if let Some(denom) = conversion.denom().to_f64() {
                                    let r = $coefficient;
                                    let c = numer / denom;

                                    return r == c;
                                }
                            }
                        }
                    }

                    false
                }}
            })+
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
                #[allow(clippy::eq_op)]
                #[allow(clippy::approx_constant)]
                fn coefficient() -> Self::T {
                    from_f64($coefficient)
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    from_f64(unit_constant!(op $($constant)?))
                }
            }

            impl super::Conversion<V> for super::$unit {
                test! {
                #[inline(always)]
                #[allow(clippy::eq_op)]
                #[allow(clippy::approx_constant)]
                fn is_valid() -> bool {
                    use $crate::num::{FromPrimitive, ToPrimitive};

                    if let Some(conversion) = $crate::num::rational::Ratio::<$crate::num::BigInt>::from_f64($coefficient) {
                        if conversion.numer() >= conversion.denom() {
                            if let Some(numer) = conversion.numer().to_f64() {
                                if let Some(denom) = conversion.denom().to_f64() {
                                    let r = $coefficient;
                                    let c = numer / denom;

                                    return r == c;
                                }
                            }
                        }
                    }

                    false
                }}
            })+
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
                #[allow(clippy::eq_op)]
                #[allow(clippy::approx_constant)]
                fn coefficient() -> Self::T {
                    from_f64($coefficient)
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    from_f64(unit_constant!(op $($constant)?))
                }
            }

            impl super::Conversion<V> for super::$unit {
                test! {
                #[inline(always)]
                #[allow(clippy::eq_op)]
                #[allow(clippy::approx_constant)]
                fn is_valid() -> bool {
                    use $crate::num::{FromPrimitive, ToPrimitive};

                    if let Some(conversion) = Self::T::from_f64($coefficient) {
                        // Factional conversion factors will end up being truncated.
                        if conversion.numer() >= conversion.denom() {
                            if let Some(numer) = conversion.numer().to_f64() {
                                if let Some(denom) = conversion.denom().to_f64() {
                                    let r = $coefficient;
                                    let c = numer / denom;

                                    return r == c;
                                }
                            }
                        }
                    }

                    false
                }}
            })+
        }

        storage_types! {
            types: Complex;

            $(impl $crate::Conversion<V> for super::$unit {
                type T = VV;

                #[inline(always)]
                #[allow(clippy::eq_op)]
                #[allow(clippy::approx_constant)]
                fn coefficient() -> Self::T {
                    $coefficient
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    unit_constant!(op $($constant)?)
                }
            }

            impl super::Conversion<V> for super::$unit {
                test! {
                #[inline(always)]
                #[allow(clippy::eq_op)]
                #[allow(clippy::approx_constant)]
                fn is_valid() -> bool {
                    use $crate::num::ToPrimitive;

                    let r = Some($coefficient);
                    let c = <Self as $crate::Conversion<V>>::coefficient().to_f64();

                    r == c
                }}
            })+
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! unit_serde {
    ($quantity:ident; $($unit:ident),+) => {
        serde! {
            $(impl $unit {
                /// Deserialize a value in this unit from any deserializer.
                ///
                /// This function is generic over storage types (f32, f64, i32, etc.) and can be used
                /// with serde's `deserialize_with` attribute:
                ///
                /// ```ignore
                /// #[derive(Deserialize)]
                /// struct MyStruct {
                ///     #[serde(deserialize_with = "millimeter::deserialize")]
                ///     width: uom::si::f64::Length,
                /// }
                /// ```
                #[allow(dead_code)]
                pub fn deserialize<'de, De, U, V, T>(deserializer: De) -> Result<$quantity<U, V>, De::Error>
                where
                    De: $crate::serde::Deserializer<'de>,
                    U: __system::Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V, T = T> + $crate::serde::Deserialize<'de>,
                    T: $crate::ConversionFactor<V>,
                    $unit: $crate::Conversion<V, T = T>,
                {
                    let value: V = $crate::serde::Deserialize::deserialize(deserializer)?;
                    Ok($quantity::<U, V>::new::<$unit>(value))
                }

                /// Serialize a value in this unit to any serializer.
                ///
                /// This function is generic over storage types (f32, f64, i32, etc.) and can be used
                /// with serde's `serialize_with` attribute:
                ///
                /// ```ignore
                /// #[derive(Serialize)]
                /// struct MyStruct {
                ///     #[serde(serialize_with = "millimeter::serialize")]
                ///     width: uom::si::f64::Length,
                /// }
                /// ```
                #[allow(dead_code)]
                pub fn serialize<S, U, V, T>(quantity: &$quantity<U, V>, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: $crate::serde::Serializer,
                    U: __system::Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V, T = T> + $crate::serde::Serialize,
                    T: $crate::ConversionFactor<V>,
                    $unit: $crate::Conversion<V, T = T>,
                {
                    let value = quantity.get::<$unit>();
                    value.serialize(serializer)
                }
            })+
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! unit_unit {
    ($(#[$unit_attr:meta])+ @$unit:ident $plural:expr) => {
        $(#[$unit_attr])*
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Hash)]
        pub struct $unit;
    };
    (@$unit:ident $plural:expr) => {
        #[doc = $plural]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Hash)]
        pub struct $unit;
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! unit_constant {
    ($op:ident $const:expr) => {
        $const
    };
    ($op:ident) => {
        match $op {
            $crate::ConstantOp::Add => -0.0,
            $crate::ConstantOp::Sub => 0.0,
        }
    };
}
