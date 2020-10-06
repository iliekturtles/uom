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
/// * `$kind`: [Kind][kind] of the quantity. Optional. This variable should only be specified when
///   defining a quantity that has the same dimensions as another quantity but isn't comparable.
///   When not specified [`uom::Kind`](trait.Kind.html) is used.
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
///         /// Length (base unit meter, m).
///         quantity: Length; "length";
///         /// Length dimension, m.
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
/// # }
/// ```
///
/// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
/// [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
/// [kind]: https://jcgm.bipm.org/vim/en/1.2.html
#[macro_export]
macro_rules! quantity {
    (
        $(#[$quantity_attr:meta])* quantity: $quantity:ident; $description:expr;
        $(#[$dim_attr:meta])* dimension: $system:ident<$($dimension:ident),+>;
        $(kind: $kind:ty;)?
        units {
            $($(#[$unit_attr:meta])* @$unit:ident: $($conversion:expr),+; $abbreviation:expr,
                $singular:expr, $plural:expr;)+
        }
    ) => {
        mod __system {
            pub use super::super::*;
        }

        $(#[$dim_attr])*
        pub type Dimension = __system::$system<$($crate::typenum::$dimension),+,
            quantity!(@kind $($kind)?)>;

        $(#[$quantity_attr])*
        ///
        /// ## Generic Parameters
        /// * `U`: Base units.
        /// * `V`: Underlying storage type.
        pub type $quantity<U, V> = __system::Quantity<Dimension, U, V>;

        /// Marker trait to identify measurement units for the quantity. See
        /// [`Unit`](../trait.Unit.html).
        pub trait Unit: __system::Unit {}

        /// Trait to identify [units][units] which have a [conversion factor][factor] for the
        /// `Quantity`. See [`Conversion<V>`](../../trait.Conversion.html).
        ///
        /// ## Generic Parameters
        /// * `V`: Underlying storage type trait is implemented for.
        ///
        /// [units]: http://jcgm.bipm.org/vim/en/1.13.html
        /// [factor]: https://jcgm.bipm.org/vim/en/1.24.html
        pub trait Conversion<V>: Unit + $crate::Conversion<V, T = <V as $crate::Conversion<V>>::T>
        where
            V: $crate::Conversion<V>,
        {
        }

        unit! {
            @units $($(#[$unit_attr])* @$unit: $($conversion),+;
                $abbreviation, $singular, $plural;)+
        }

        /// Quantity description.
        #[allow(dead_code)]
        #[inline(always)]
        pub fn description() -> &'static str {
            $description
        }

        /// Unit enum.
        #[allow(non_camel_case_types)]
        //#[non_exhaustive] // Requires rustc 1.40.0
        #[allow(clippy::manual_non_exhaustive)]
        #[derive(Debug, Clone, Copy)]
        pub enum Units {
            $(#[doc=$plural]
            $unit($unit),)+

            #[doc(hidden)]
            __nonexhaustive,
        }

        impl Units {
            /// Unit abbreviation.
            #[allow(dead_code)]
            pub fn abbreviation(&self) -> &'static str {
                match self {
                    $(Units::$unit(_) => <$unit as __system::Unit>::abbreviation(),)+

                    Units::__nonexhaustive => "unknown",
                }
            }

            /// Unit singular description.
            #[allow(dead_code)]
            pub fn singular(&self) -> &'static str {
                match self {
                    $(Units::$unit(_) => <$unit as __system::Unit>::singular(),)+

                    Units::__nonexhaustive => "unknown",
                }
            }

            /// Unit plural description.
            #[allow(dead_code)]
            pub fn plural(&self) -> &'static str {
                match self {
                    $(Units::$unit(_) => <$unit as __system::Unit>::plural(),)+

                    Units::__nonexhaustive => "unknown",
                }
            }
        }

        static ALL_UNITS: &[Units] = &[
            $(Units::$unit($unit),)+
        ];

        /// Iterate over all defined units for this quantity.
        #[allow(dead_code)]
        pub fn units() -> impl Iterator<Item = Units> {
            ALL_UNITS.iter().copied()
        }

        impl<U, V> $quantity<U, V>
        where
            U: __system::Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            /// Create a new quantity from the given value and measurement unit.
            ///
            /// ## Generic Parameters
            /// * `N`: Unit.
            #[inline(always)]
            pub fn new<N>(v: V) -> Self
            where
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                $quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: __system::to_base::<Dimension, U, V, N>(&v),
                }
            }

            /// Retrieve the value of the quantity in the given measurement unit.
            ///
            /// ## Generic Parameters
            /// * `N`: Unit.
            #[inline(always)]
            pub fn get<N>(&self) -> V
            where
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                __system::from_base::<Dimension, U, V, N>(&self.value)
            }

            /// Returns the largest integer less than or equal to a number in the given
            /// measurement unit.
            ///
            /// ## Generic Parameters
            /// * `N`: Unit.
            #[inline(always)]
            pub fn floor<N>(self) -> Self
            where
                V: $crate::num::Float,
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                Self::new::<N>(self.get::<N>().floor())
            }

            /// Returns the smallest integer less than or equal to a number in the given
            /// measurement unit.
            ///
            /// ## Generic Parameters
            /// * `N`: Unit.
            #[inline(always)]
            pub fn ceil<N>(self) -> Self
            where
                V: $crate::num::Float,
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                Self::new::<N>(self.get::<N>().ceil())
            }

            /// Returns the nearest integer to a number in the in given measurement unit.
            /// Round half-way cases away from 0.0.
            ///
            /// ## Generic Parameters
            /// * `N`: Unit.
            #[inline(always)]
            pub fn round<N>(self) -> Self
            where
                V: $crate::num::Float,
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                Self::new::<N>(self.get::<N>().round())
            }

            /// Returns the integer part of a number in the given measurement unit.
            ///
            /// ## Generic Parameters
            /// * `N`: Unit.
            #[inline(always)]
            pub fn trunc<N>(self) -> Self
            where
                V: $crate::num::Float,
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                Self::new::<N>(self.get::<N>().trunc())
            }

            /// Returns the fractional part of a number in the given measurement unit.
            ///
            /// ## Generic Parameters
            /// * `N`: Unit.
            #[inline(always)]
            pub fn fract<N>(self) -> Self
            where
                V: $crate::num::Float,
                N: Unit + $crate::Conversion<V, T = V::T>,
            {
                Self::new::<N>(self.get::<N>().fract())
            }

            /// Creates a struct that can be used to format a compatible quantity for display.
            ///
            /// # Notes
            /// The return value of this method cannot be used to print directly, but is instead
            /// used to format quantities and can be reused; see
            /// [Arguments::with](../fmt/struct.Arguments.html#method.with) and the examples below.
            ///
            /// If you do not need to format multiple quantities, consider using
            /// [`into_format_args`](#method.into_format_args) instead.
            ///
            /// # Examples
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::time::{femtosecond, picosecond};
            /// # use uom::si::fmt::Arguments;
            /// # use uom::fmt::DisplayStyle::*;
            /// let t1 = Time::new::<femtosecond>(1.0_E-1);
            /// let t2 = Time::new::<picosecond>(1.0_E-1);
            /// let a = Time::format_args(femtosecond, Description);
            ///
            /// assert_eq!("0.1 femtoseconds", format!("{}", a.with(t1)));
            /// assert_eq!("100 femtoseconds", format!("{}", a.with(t2)));
            /// ```
            ///
            /// ## Generic Parameters
            /// * `N`: Unit.
            pub fn format_args<N>(
                unit: N,
                style: $crate::fmt::DisplayStyle
            ) -> __system::fmt::Arguments<Dimension, N>
            where
                N: Unit
            {
                __system::fmt::Arguments {
                    dimension: $crate::lib::marker::PhantomData,
                    unit,
                    style,
                }
            }

            /// Creates a struct that formats `self` for display.
            ///
            /// # Notes
            /// Unlike [`format_args`](#method.format_args), the return value of this method can be
            /// used directly for display. It will format the value of `self` for the quantity on
            /// which it is called and nothing else.
            ///
            /// If you wish to reuse the return value to format multiple quantities, use
            /// [`format_args`](#method.format_args) instead.
            ///
            /// # Examples
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::time::{femtosecond, picosecond};
            /// # use uom::si::fmt::Arguments;
            /// # use uom::fmt::DisplayStyle::*;
            /// let t = Time::new::<picosecond>(1.0_E-1);
            /// let a = t.into_format_args(femtosecond, Description);
            ///
            /// assert_eq!("100 femtoseconds", format!("{}", a));
            /// ```
            ///
            /// ## Generic Parameters
            /// * `N`: Unit.
            pub fn into_format_args<N>(
                self,
                unit: N,
                style: $crate::fmt::DisplayStyle
            ) -> __system::fmt::QuantityArguments<Dimension, U, V, N>
            where
                N: Unit
            {
                __system::fmt::QuantityArguments {
                    arguments: __system::fmt::Arguments {
                        dimension: $crate::lib::marker::PhantomData,
                        unit,
                        style,
                    },
                    quantity: self,
                }
            }
        }

        impl<N> __system::fmt::Arguments<Dimension, N>
        where
            N: __system::Unit + Unit,
        {
            /// Specifies a quantity to display.
            ///
            /// ## Generic Parameters
            /// * `U`: Base units.
            /// * `V`: Underlying storage type trait is implemented for.
            pub fn with<U, V>(
                self,
                quantity: $quantity<U, V>
            ) -> __system::fmt::QuantityArguments<Dimension, U, V, N>
            where
                U: __system::Units<V> + ?Sized,
                V: $crate::num::Num + $crate::Conversion<V>,
            {
                __system::fmt::QuantityArguments {
                    arguments: self,
                    quantity,
                }
            }
        }

        mod str {
            storage_types! {
                use $crate::lib::str::FromStr;
                use $crate::str::ParseQuantityError::*;

                impl<U> FromStr for super::super::$quantity<U, V>
                where
                    U: super::super::__system::Units<V> + ?Sized,
                {
                    type Err = $crate::str::ParseQuantityError;

                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        let mut parts = s.splitn(2, ' ');
                        let value = parts.next().unwrap();
                        let unit = parts.next().ok_or(NoSeparator)?;
                        let value = value.parse::<V>().map_err(|_| ValueParseError)?;

                        #[allow(unreachable_patterns)]
                        match unit.trim() {
                            $($abbreviation | $singular | $plural => Ok(Self::new::<super::super::$unit>(value)),)+
                            _ => Err(UnknownUnit),
                        }
                    }
                }
            }
        }
    };
    (@kind $kind:ty) => { $kind };
    (@kind) => { dyn $crate::Kind };
}
