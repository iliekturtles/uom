/// Macro to implement a [quantity][quantity] and associated [measurement units][measurement]. Note
/// that this macro must be executed in direct submodules of the module where the
/// [`system!`](macro.system.html) macro was executed. `@...` match arms are considered private.
///
/// * `$quantity_attr`: Quantity attributes. Generally used to set documentation comments for the
///   quantity.
/// * `$quantity`: Quantity name (e.g. `Length`).
/// * `$description`: Quantity description (e.g. `"length"`).
/// * `$dim_attr`: Dimension attributes. Generally used to set documentation comments for the
///   quantity's dimension type alias.
/// * `$system`: System of quantities type (e.g. `ISQ`).
/// * `$dimension`: Power of a factor for each base quantity in the system. Power should be
///   represented as a `typenum` type-level integer (e.g. `N1`, `Z0`, `P1`, `P2`, ...).
/// * `$unit`: Unit name (e.g. `meter`, `foot`).
/// * `$conversion`: Conversion from the unit to the base unit of the quantity (e.g. `3.048E-1` to
///   convert `foot` to `meter`).
/// * `$abbreviation`: Unit abbreviation (e.g. `"m"`).
/// * `$singular`: Singular unit description (e.g. `"meter"`).
/// * `$plural`: Plural unit description (e.g. `"meters"`).
///
/// An example invocation is given below for the quantity of length in a meter-kilogram-second
/// system. The `#[macro_use]` attribute must be used when including the `uom` crate to make the
/// `quantity!` macro available.
///
/// ```ignore
/// #[macro_use]
/// extern crate uom;
///
/// # fn main() { }
/// # mod mks {
/// #[macro_use]
/// mod length {
///     quantity! {
///         /// Length (base unit meter, m<sup>1</sup>).
///         quantity: Length; "length";
///         /// Length dimension, m<sup>1</sup>.
///         dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
///         units {
///             @meter: 1.0E0; "m", "meter", "meters";
///             @foot: 3.048E-1; "ft", "foot", "feet";
///         }
///         impl {
///             pub fn custom_description() -> &'static str { "LENGTH" }
///         }
///     }
/// }
/// #     #[macro_use]
/// #     mod mass {
/// #         quantity! {
/// #             /// Mass (base unit kilogram, kg<sup>1</sup>).
/// #             quantity: Mass; "mass";
/// #             /// Mass dimension, kg<sup>1</sup>.
/// #             dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @kilogram: 1.0; "kg", "kilogram", "kilograms";
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod time {
/// #         quantity! {
/// #             /// Time (base unit second, s<sup>1</sup>).
/// #             quantity: Time; "time";
/// #             /// Time dimension, s<sup>1</sup>.
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
/// #         Q!(mks, f32/*, (centimeter, gram, second)*/);
/// #     }
/// # }
/// ```
///
/// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
/// [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
#[macro_export]
macro_rules! quantity {
    (
        $(#[$quantity_attr:meta])* quantity: $quantity:ident; $description:expr;
        $(#[$dim_attr:meta])* dimension: $system:ident<$($dimension:ident),+>;
        units {
            $($(#[$unit_attr:meta])* @$unit:ident: $conversion:expr;
                $abbreviation:expr, $singular:expr, $plural:expr;)+
        }
    ) => {
        $(#[$dim_attr])*
        pub type Dimension = super::$system<$($crate::typenum::$dimension),+>;

        $(#[$quantity_attr])*
        pub type $quantity<U, V> = super::Quantity<Dimension, U, V>;

        /// Marker trait to identify measurement units for the quantity. See
        /// [`Unit`](../trait.Unit.html).
        pub trait Unit<V>: super::Conversion<V> {}

        $(quantity!(@unit $(#[$unit_attr])* @$unit);

        impl super::Unit for $unit {
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
        })+

        /// Quantity description.
        #[allow(dead_code)]
        #[inline(always)]
        pub fn description() -> &'static str {
            $description
        }

        impl<U, V> $quantity<U, V>
        where
            U: super::Units<Dimension, V>,
            V: $crate::num::Num,
        {
            /// Create a new quantity from the given value and measurement unit.
            #[inline(always)]
            pub fn new<N>(v: V) -> Self
            where
                N: Unit<V>,
            {
                $quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
                    value: v * <N as super::Conversion<V>>::conversion()
                        / <U as super::Units<Dimension, V>>::conversion(),
                }
            }

            /// Retrieve the value of the quantity in the given measurement unit.
            #[inline(always)]
            pub fn get<N>(self, _unit: N) -> V
            where
                N: Unit<V>,
            {
                self.value * <U as super::Units<Dimension, V>>::conversion()
                    / <N as super::Conversion<V>>::conversion()
            }

            /// Returns the largest integer less than or equal to a number in the given
            /// measurement unit.
            #[inline(always)]
            pub fn floor<N>(self, _unit: N) -> Self
            where
                V: $crate::num::Float,
                N: Unit<V>,
            {
                Self::new::<N>(self.get(_unit).floor())
            }

            /// Returns the smallest integer less than or equal to a number in the given
            /// measurement unit.
            #[inline(always)]
            pub fn ceil<N>(self, _unit: N) -> Self
            where
                V: $crate::num::Float,
                N: Unit<V>,
            {
                Self::new::<N>(self.get(_unit).ceil())
            }

            /// Returns the nearest integer to a number in the in given measurement unit.
            /// Round half-way cases away from 0.0.
            #[inline(always)]
            pub fn round<N>(self, _unit: N) -> Self
            where
                V: $crate::num::Float,
                N: Unit<V>,
            {
                Self::new::<N>(self.get(_unit).round())
            }

            /// Returns the integer part of a number in the given measurement unit.
            #[inline(always)]
            pub fn trunc<N>(self, _unit: N) -> Self
            where
                V: $crate::num::Float,
                N: Unit<V>,
            {
                Self::new::<N>(self.get(_unit).trunc())
            }

            /// Returns the fractional part of a number in the given measurement unit.
            #[inline(always)]
            pub fn fract<N>(self, _unit: N) -> Self
            where
                V: $crate::num::Float,
                N: Unit<V>,
            {
                Self::new::<N>(self.get(_unit).fract())
            }
        }

        $(impl<V> Unit<V> for $unit
        where
            V: $crate::num::Num + $crate::num::FromPrimitive,
        {
        }

        impl<V> super::Conversion<V> for $unit
        where
            V: $crate::num::Num + $crate::num::FromPrimitive,
        {
            #[inline(always)]
            fn conversion() -> V {
                V::from_f64($conversion).unwrap()
            }
        })+
    };
    (@unit $(#[$unit_attr:meta])+ @$unit:ident) => {
        $(#[$unit_attr])*
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Hash)]
        pub struct $unit;
    };
    (@unit @$unit:ident) => {
        /// Measurement unit.
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Hash)]
        pub struct $unit;
    };
}
