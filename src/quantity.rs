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
/// ```
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
        pub trait Unit: super::Unit {}

        /// Trait to identify [units][units] which have a [conversion factor][factor] for the
        /// `Quantity`. See [`Conversion<V>`](../../trait.Conversion.html).
        ///
        /// [units]: http://jcgm.bipm.org/vim/en/1.13.html
        /// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
        pub trait Conversion<V>: Unit + $crate::Conversion<V, T = <V as $crate::Conversion<V>>::T>
        where
            V: $crate::Conversion<V>,
        {
        }

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
        }

        impl Unit for $unit {})+

        storage_types! {
            types: Float;

            $(impl $crate::Conversion<V> for super::$unit {
                type T = V;

                #[inline(always)]
                fn conversion() -> Self::T {
                    $conversion
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        storage_types! {
            types: PrimInt, BigInt;

            $(impl $crate::Conversion<V> for super::$unit {
                type T = $crate::num::rational::Ratio<V>;

                #[inline(always)]
                fn conversion() -> Self::T {
                    use $crate::num::FromPrimitive;

                    Self::T::from_f64($conversion).unwrap()
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        storage_types! {
            types: BigUint;

            $(impl $crate::Conversion<V> for super::$unit {
                type T = $crate::num::rational::Ratio<V>;

                #[inline(always)]
                fn conversion() -> Self::T {
                    use $crate::num::FromPrimitive;

                    let c = $crate::num::rational::Ratio::<$crate::num::BigInt>::from_f64($conversion).unwrap();

                    Self::T::new(c.numer().to_biguint().unwrap(),
                        c.denom().to_biguint().unwrap())
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        storage_types! {
            types: Ratio;

            $(impl $crate::Conversion<V> for super::$unit {
                type T = V;

                #[inline(always)]
                fn conversion() -> Self::T {
                    use $crate::num::FromPrimitive;

                    Self::T::from_f64($conversion).unwrap()
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        /// Quantity description.
        #[allow(dead_code)]
        #[inline(always)]
        pub fn description() -> &'static str {
            $description
        }

        impl<U, V> $quantity<U, V>
        where
            U: super::Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            /// Create a new quantity from the given value and measurement unit.
            #[inline(always)]
            pub fn new<N>(v: V) -> Self
            where
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                $quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
                    value: super::to_base::<Dimension, U, V, N>(&v),
                }
            }

            /// Retrieve the value of the quantity in the given measurement unit.
            #[inline(always)]
            pub fn get<N>(&self, _unit: N) -> V
            where
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                super::from_base::<Dimension, U, V, N>(&self.value)
            }

            /// Returns the largest integer less than or equal to a number in the given
            /// measurement unit.
            #[inline(always)]
            pub fn floor<N>(self, _unit: N) -> Self
            where
                V: $crate::num::Float,
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                Self::new::<N>(self.get(_unit).floor())
            }

            /// Returns the smallest integer less than or equal to a number in the given
            /// measurement unit.
            #[inline(always)]
            pub fn ceil<N>(self, _unit: N) -> Self
            where
                V: $crate::num::Float,
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                Self::new::<N>(self.get(_unit).ceil())
            }

            /// Returns the nearest integer to a number in the in given measurement unit.
            /// Round half-way cases away from 0.0.
            #[inline(always)]
            pub fn round<N>(self, _unit: N) -> Self
            where
                V: $crate::num::Float,
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                Self::new::<N>(self.get(_unit).round())
            }

            /// Returns the integer part of a number in the given measurement unit.
            #[inline(always)]
            pub fn trunc<N>(self, _unit: N) -> Self
            where
                V: $crate::num::Float,
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                Self::new::<N>(self.get(_unit).trunc())
            }

            /// Returns the fractional part of a number in the given measurement unit.
            #[inline(always)]
            pub fn fract<N>(self, _unit: N) -> Self
            where
                V: $crate::num::Float,
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                Self::new::<N>(self.get(_unit).fract())
            }
        }
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
