/// Macro to implement a [system of quantities](http://jcgm.bipm.org/vim/en/1.3.html). `@...` match
/// arms are considered private.
///
/// * `$quantities_attr`: System of quantities attributes. Generally used to set documentation
///   comments for the system of quantities.
/// * `$quantities`: Name of the system of quantities (e.g. `ISQ`).
/// * `$name_attr`: Base quantity attributes. Generally used to set documentation comments for base
///   units.
/// * `$name`: Name of the base quantities for the system of quantities (e.g. `length`, `mass`,
///   ...). Note that this name must match the module name of the quantity.
/// * `$unit`: Base unit of the quantity (e.g. `meter`, `kilogram`).
/// * `$symbol`: Dimension symbol of the quantity.
/// * `$units_attr`: System of units attributes. Generally used to set documentation comments for
///   the system of units.
/// * `$units`: Name of the system of units (e.g. `SI`).
/// * `$module`: Module name of the quantity. When prefixed by the `mod` keyword the module must
///   already be defined with the `#[macro_use]` attribute. A `#[macro_use] pub mod $module;`
///   statement is generated if this variable is not prefixed by the `mod` keyword.
/// * `$quantity`: Quantity name (e.g. `Length`, `Mass`, ...).
///
/// An example invocation is given below for a meter-kilogram-second system. The `#[macro_use]`
/// attribute must be used when including the `uom` crate to make the `system!` macro available.
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
/// system! {
///     /// System of quantities, Q.
///     quantities: Q {
///         length: meter, L;
///         mass: kilogram, M;
///         time: second, T;
///     }
///     /// System of units, U.
///     units: U {
///         mod length::Length,
///         mod mass::Mass,
///         mod time::Time,
///     }
/// }
/// #     mod f32 {
/// #         Q!(mks, f32/*, (centimeter, gram, second)*/);
/// #     }
/// # }
/// ```
#[macro_export]
macro_rules! system {
    (
        $(#[$quantities_attr:meta])* quantities: $quantities:ident {
            $($(#[$name_attr:meta])* $name:ident: $unit:ident, $symbol:ident;)+
        }
        $(#[$units_attr:meta])* units: $units:ident {
            $($module:ident::$quantity:ident,)+
        }
    ) => {
        $(#[macro_use]
        pub mod $module;)+

        system! {
            $(#[$quantities_attr])*
            quantities: $quantities {
                $($(#[$name_attr])* $name: $unit, $symbol;)+
            }
            $(#[$units_attr])*
            units: $units {
                $(mod $module::$quantity,)+
            }
        }
    };
    (
        $(#[$quantities_attr:meta])* quantities: $quantities:ident {
            $($(#[$name_attr:meta])* $name:ident: $unit:ident, $symbol:ident;)+
        }
        $(#[$units_attr:meta])* units: $units:ident {
            $(mod $module:ident::$quantity:ident,)+
        }
    ) => {
        /// Marker trait to express the dependence of a [quantity][quantity] on the
        /// [base quantities][base] of a [system of quantities][quantities] as a product of powers
        /// of factors corresponding to the base quantities, omitting any numerical factor.
        ///
        /// * <http://jcgm.bipm.org/vim/en/1.7.html>
        ///
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.4.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        pub trait Dimension: Send + Sync {
            $($(#[$name_attr])*
            ///
            /// Quantity dimension.
            type $symbol: $crate::typenum::Integer;)+

            /// [Kind](https://jcgm.bipm.org/vim/en/1.2.html) of the quantity. Quantities of the
            /// same dimension but differing kinds are not comparable.
            type Kind: ?Sized;
        }

        /// Marker trait to identify a [system of units][units] based on a set of [base units][base]
        /// of a [system of quantities][quantities].
        ///
        /// ## Generic Parameters
        /// * `V`: Underlying storage type trait is implemented for.
        ///
        /// [units]: http://jcgm.bipm.org/vim/en/1.13.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.10.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        pub trait Units<V>: Send + Sync
        where
            V: $crate::Conversion<V>,
        {
            $($(#[$name_attr])*
            ///
            /// Base unit.
            #[allow(non_camel_case_types)]
            type $name: Unit + $crate::Conversion<V, T = V::T>;)+
        }

        /// Trait to identify [measurement units][measurement] of individual
        /// [quantities][quantity].
        ///
        /// [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        pub trait Unit: Copy {
            /// Unit abbreviation.
            fn abbreviation() -> &'static str;

            /// Unit singular description.
            fn singular() -> &'static str;

            /// Unit plural description.
            fn plural() -> &'static str;
        }

        /// Property of a phenomenon, body or substance, where the property has a magnitude that
        /// can be expressed as a number and a reference.
        ///
        /// The preferred method of creating a `Quantity` instance is to use the `new` constructor
        /// which is generic over the input unit and accepts the input value as it's only
        /// parameter.
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::f32::*;
        /// # use uom::si::length::meter;
        /// // Create a length of 1 meter.
        /// let l = Length::new::<meter>(1.0);
        /// ```
        ///
        /// `Quantity` fields are public to allow for the creation of `const` values and instances
        /// of non-named `Quantity`s. This functionality will be deprecated and subsequently removed
        /// once the [`const fn`](https://github.com/rust-lang/rust/issues/24111) feature is
        /// stabilized.
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::{Quantity, ISQ, SI};
        /// # use uom::si::f32::*;
        /// # use uom::lib::marker::PhantomData;
        /// # use uom::typenum::{P2, Z0};
        /// // Create a `const` length of 1 meter.
        /// const L: Length = Length { dimension: PhantomData, units: PhantomData, value: 1.0, };
        /// // Create a `const` area of 1 square meter explicitly without using the `Area` alias.
        /// const A: Quantity<ISQ<P2, Z0, Z0, Z0, Z0, Z0, Z0>, SI<f32>, f32> =
        ///     Quantity { dimension: PhantomData, units: PhantomData, value: 1.0, };
        /// ```
        ///
        /// Using units for the wrong quantity will cause a compile error:
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust,compile_fail")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::f32::*;
        /// # use uom::si::time::second;
        /// // error[E0277]: the trait bound `second: length::Unit` is not satisfied
        /// let l = Length::new::<second>(1.0);
        /// ```
        ///
        /// Mixing quantities will also cause a compile error:
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust,compile_fail")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::f32::*;
        /// # use uom::si::length::meter;
        /// # use uom::si::time::second;
        /// // error[E0308]: mismatched types
        /// let r = Length::new::<meter>(1.0) + Time::new::<second>(1.0);
        /// ```
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust,compile_fail")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::f32::*;
        /// # use uom::si::length::meter;
        /// # use uom::si::time::second;
        /// // error[E0308]: mismatched types
        /// let v: Velocity = Length::new::<meter>(1.0) * Time::new::<second>(1.0);
        /// ```
        ///
        /// * <http://jcgm.bipm.org/vim/en/1.1.html>
        ///
        /// ## Generic Parameters
        /// * `D`: Quantity dimension. See [`Dimension`](./trait.Dimension.html).
        /// * `U`: Quantity base units. See [`Units`](./trait.Units.html).
        /// * `V`: Quantity value underlying storage type.
        #[repr(transparent)]
        pub struct Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            /// Quantity dimension. See [`Dimension`](./trait.Dimension.html).
            pub dimension: $crate::lib::marker::PhantomData<D>,

            /// Quantity base units. See [`Units`](./trait.Units.html).
            pub units: $crate::lib::marker::PhantomData<U>,

            /// Quantity value stored in the base units for the quantity.
            pub value: V,
        }

        // Type alias for dimensions where all exponents of the factors are the given value.
        type DN<N> = dyn Dimension<$($symbol = system!(@replace $symbol N),)+
            Kind = dyn $crate::Kind>;

        /// Type alias for [dimension one][one] for which all the exponents of the factors
        /// corresponding to the [base quantities][base] are zero.
        ///
        /// [one]: http://jcgm.bipm.org/vim/en/1.8.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.4.html
        #[allow(dead_code)]
        pub type DimensionOne = DN<$crate::typenum::Z0>;

        $(#[$quantities_attr])*
        pub type $quantities<$($symbol,)+ K = dyn $crate::Kind> =
            dyn Dimension<$($symbol = $symbol,)+ Kind = K>;

        $(#[$units_attr])*
        ///
        /// ## Generic Parameters
        /// * `V`: Underlying storage type.
        #[allow(unused_qualifications)]
        pub type $units<V> = dyn Units<V, $($name = $name::$unit),+>;

        /// Convert a value from base units to the given unit.
        ///
        /// ## Generic Parameters
        /// * `D`: Dimension.
        /// * `U`: Base units.
        /// * `V`: Value underlying storage type.
        /// * `N`: Unit.
        #[inline(always)]
        fn from_base<D, U, V, N>(v: &V) -> V
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::Conversion<V> + $crate::lib::ops::Mul<V, Output = V>,
            N: $crate::Conversion<V, T = V::T>,
        {
            use $crate::typenum::Integer;
            use $crate::Conversion;
            use $crate::ConversionFactor;

            (v.into_conversion() $(* U::$name::coefficient().powi(D::$symbol::to_i32()))+
                    / N::coefficient() - N::constant($crate::ConstantOp::Sub))
                .value()
        }

        /// Convert a value from the given unit to base units.
        ///
        /// ## Generic Parameters
        /// * `D`: Dimension.
        /// * `U`: Base units.
        /// * `V`: Value underlying storage type.
        /// * `N`: Unit.
        #[inline(always)]
        fn to_base<D, U, V, N>(v: &V) -> V
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::Conversion<V> + $crate::lib::ops::Mul<V, Output = V>,
            N: $crate::Conversion<V, T = V::T>,
        {
            use $crate::typenum::Integer;
            use $crate::Conversion;
            use $crate::ConversionFactor;

            ((v.into_conversion() + N::constant($crate::ConstantOp::Add)) * N::coefficient()
                    / (V::coefficient() $(* U::$name::coefficient().powi(D::$symbol::to_i32()))+))
                .value()
        }

        autoconvert_test! {
        /// Convert a value from one set of base units to a second.
        ///
        /// ## Generic Parameters
        /// * `D`: Dimension.
        /// * `Ul`: Base units for left quantity.
        /// * `Ur`: Base units for right quantity.
        /// * `V`: Value underlying storage type.
        #[allow(dead_code)]
        #[inline(always)]
        fn change_base<D, Ul, Ur, V>(v: &V) -> V
        where
            D: Dimension + ?Sized,
            Ul: Units<V> + ?Sized,
            Ur: Units<V> + ?Sized,
            V: $crate::Conversion<V> + $crate::lib::ops::Mul<V, Output = V>,
        {
            use $crate::typenum::Integer;
            use $crate::Conversion;
            use $crate::ConversionFactor;

            (v.into_conversion() $(* Ur::$name::coefficient().powi(D::$symbol::to_i32())
                    / Ul::$name::coefficient().powi(D::$symbol::to_i32()))+)
                .value()
        }}

        #[doc(hidden)]
        macro_rules! impl_ops {
            (
                $AddSubTrait:ident, $addsub_fun:ident, $addsub_op:tt,
                $AddSubAssignTrait:ident, $addsubassign_fun:ident, $addsubassign_op:tt,
                $AddSubAlias:ident,
                $MulDivTrait:ident, $muldiv_fun:ident, $muldiv_op:tt,
                $MulDivAssignTrait:ident, $muldivassign_fun:ident, $muldivassign_op:tt,
                $Mod:ident
            ) => {
                autoconvert! {
                impl<D, Ul, Ur, V> $crate::lib::ops::$AddSubTrait<Quantity<D, Ur, V>>
                    for Quantity<D, Ul, V>
                where
                    D: Dimension + ?Sized,
                    D::Kind: $crate::marker::$AddSubTrait,
                    Ul: Units<V> + ?Sized,
                    Ur: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V>,
                {
                    type Output = Quantity<D, Ul, V>;

                    #[inline(always)]
                    fn $addsub_fun(self, rhs: Quantity<D, Ur, V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::lib::marker::PhantomData,
                            units: $crate::lib::marker::PhantomData,
                            value: self.value $addsub_op change_base::<D, Ul, Ur, V>(&rhs.value),
                        }
                    }
                }}

                not_autoconvert! {
                impl<D, U, V> $crate::lib::ops::$AddSubTrait for Quantity<D, U, V>
                where
                    D: Dimension + ?Sized,
                    D::Kind: $crate::marker::$AddSubTrait,
                    U: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V>,
                {
                    type Output = Self;

                    #[inline(always)]
                    fn $addsub_fun(self, rhs: Self) -> Self::Output {
                        Quantity {
                            dimension: $crate::lib::marker::PhantomData,
                            units: $crate::lib::marker::PhantomData,
                            value: self.value $addsub_op rhs.value,
                        }
                    }
                }}

                autoconvert! {
                impl<D, Ul, Ur, V> $crate::lib::ops::$AddSubAssignTrait<Quantity<D, Ur, V>>
                    for Quantity<D, Ul, V>
                where
                    D: Dimension + ?Sized,
                    D::Kind: $crate::marker::$AddSubAssignTrait,
                    Ul: Units<V> + ?Sized,
                    Ur: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V>
                        + $crate::lib::ops::$AddSubAssignTrait<V>,
                {
                    #[inline(always)]
                    fn $addsubassign_fun(&mut self, rhs: Quantity<D, Ur, V>) {
                        self.value $addsubassign_op change_base::<D, Ul, Ur, V>(&rhs.value);
                    }
                }}

                not_autoconvert! {
                impl<D, U, V> $crate::lib::ops::$AddSubAssignTrait for Quantity<D, U, V>
                where
                    D: Dimension + ?Sized,
                    D::Kind: $crate::marker::$AddSubAssignTrait,
                    U: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V>
                        + $crate::lib::ops::$AddSubAssignTrait<V>,
                {
                    #[inline(always)]
                    fn $addsubassign_fun(&mut self, rhs: Self) {
                        self.value $addsubassign_op rhs.value;
                    }
                }}

                autoconvert! {
                impl<Dl, Dr, Ul, Ur, V> $crate::lib::ops::$MulDivTrait<Quantity<Dr, Ur, V>>
                    for Quantity<Dl, Ul, V>
                where
                    Dl: Dimension + ?Sized,
                    $(Dl::$symbol: $crate::lib::ops::$AddSubTrait<Dr::$symbol>,)+
                    Dl::Kind: $crate::marker::$MulDivTrait,
                    Dr: Dimension + ?Sized,
                    Dr::Kind: $crate::marker::$MulDivTrait,
                    Ul: Units<V> + ?Sized,
                    Ur: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::ops::$MulDivTrait<V>,
                {
                    type Output = Quantity<
                        $quantities<$($crate::typenum::$AddSubAlias<Dl::$symbol, Dr::$symbol>,)+>,
                        Ul, V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: Quantity<Dr, Ur, V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::lib::marker::PhantomData,
                            units: $crate::lib::marker::PhantomData,
                            value: self.value $muldiv_op change_base::<Dr, Ul, Ur, V>(&rhs.value),
                        }
                    }
                }}

                not_autoconvert! {
                impl<Dl, Dr, U, V> $crate::lib::ops::$MulDivTrait<Quantity<Dr, U, V>>
                    for Quantity<Dl, U, V>
                where
                    Dl: Dimension + ?Sized,
                    $(Dl::$symbol: $crate::lib::ops::$AddSubTrait<Dr::$symbol>,)+
                    Dl::Kind: $crate::marker::$MulDivTrait,
                    Dr: Dimension + ?Sized,
                    Dr::Kind: $crate::marker::$MulDivTrait,
                    U: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::ops::$MulDivTrait<V>,
                {
                    type Output = Quantity<
                        $quantities<$($crate::typenum::$AddSubAlias<Dl::$symbol, Dr::$symbol>,)+>,
                        U, V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: Quantity<Dr, U, V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::lib::marker::PhantomData,
                            units: $crate::lib::marker::PhantomData,
                            value: self.value $muldiv_op rhs.value,
                        }
                    }
                }}

                impl<D, U, V> $crate::lib::ops::$MulDivTrait<V> for Quantity<D, U, V>
                where
                    D: Dimension + ?Sized,
                    D::Kind: $crate::marker::$MulDivTrait,
                    U: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V>,
                {
                    type Output = Quantity<D, U, V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: V) -> Self::Output {
                        Quantity {
                            dimension: $crate::lib::marker::PhantomData,
                            units: $crate::lib::marker::PhantomData,
                            value: self.value $muldiv_op rhs,
                        }
                    }
                }

                impl<D, U, V> $crate::lib::ops::$MulDivAssignTrait<V> for Quantity<D, U, V>
                where
                    D: Dimension + ?Sized,
                    D::Kind: $crate::marker::$MulDivAssignTrait,
                    U: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V>
                        + $crate::lib::ops::$MulDivAssignTrait<V>,
                {
                    #[inline(always)]
                    fn $muldivassign_fun(&mut self, rhs: V) {
                        self.value $muldivassign_op rhs;
                    }
                }

                #[doc(hidden)]
                mod $Mod {
                    storage_types! {
                        use super::super::*;

                        impl<D, U> $crate::lib::ops::$MulDivTrait<Quantity<D, U, V>> for V
                        where
                            D: Dimension + ?Sized,
                            D::Kind: $crate::marker::$MulDivTrait,
                            U: Units<V> + ?Sized,
                            $($crate::typenum::Z0: $crate::lib::ops::$AddSubTrait<D::$symbol>,)+
                        {
                            type Output = Quantity<
                                $quantities<
                                    $($crate::typenum::$AddSubAlias<
                                      $crate::typenum::Z0,
                                      D::$symbol>,)+
                                      D::Kind>,
                                U, V>;

                            #[inline(always)]
                            fn $muldiv_fun(self, rhs: Quantity<D, U, V>) -> Self::Output {
                                Quantity {
                                    dimension: $crate::lib::marker::PhantomData,
                                    units: $crate::lib::marker::PhantomData,
                                    value: self $muldiv_op rhs.value,
                                }
                            }
                        }
                    }
                }
            };
        }

        impl_ops!(Add, add, +, AddAssign, add_assign, +=, Sum,
            Mul, mul, *, MulAssign, mul_assign, *=, add_mul);
        impl_ops!(Sub, sub, -, SubAssign, sub_assign, -=, Diff,
            Div, div, /, DivAssign, div_assign, /=, sub_div);

        impl<D, U, V> Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            /// Returns `true` if this value is `NAN` and `false` otherwise.
            #[cfg_attr(feature = "cargo-clippy", allow(clippy::wrong_self_convention))]
            #[inline(always)]
            pub fn is_nan(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_nan()
            }

            /// Returns `true` if this value is positive infinity or negative infinity and
            /// `false` otherwise.
            #[cfg_attr(feature = "cargo-clippy", allow(clippy::wrong_self_convention))]
            #[inline(always)]
            pub fn is_infinite(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_infinite()
            }

            /// Returns `true` if this number is neither infinite nor `NAN`.
            #[cfg_attr(feature = "cargo-clippy", allow(clippy::wrong_self_convention))]
            #[inline(always)]
            pub fn is_finite(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_finite()
            }

            /// Returns `true` if the number is neither zero, infinite, subnormal, or `NAN`.
            #[cfg_attr(feature = "cargo-clippy", allow(clippy::wrong_self_convention))]
            #[inline(always)]
            pub fn is_normal(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_normal()
            }

            /// Returns the floating point category of the number. If only one property is
            /// going to be tested, it is generally faster to use the specific predicate
            /// instead.
            #[inline(always)]
            pub fn classify(self) -> $crate::lib::num::FpCategory
            where
                V: $crate::num::Float,
            {
                self.value.classify()
            }

            std! {
            /// Takes the cubic root of a number.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::volume::cubic_meter;
            /// let l: Length = Volume::new::<cubic_meter>(8.0).cbrt();
            /// ```
            ///
            /// The input type must have dimensions divisible by three:
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust,compile_fail")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::area::square_meter;
            /// // error[E0271]: type mismatch resolving ...
            /// let r = Area::new::<square_meter>(8.0).cbrt();
            /// ```
            #[inline(always)]
            pub fn cbrt(
                self
            ) -> Quantity<
                $quantities<$($crate::typenum::PartialQuot<D::$symbol, $crate::typenum::P3>),+>,
                U, V>
            where
                $(D::$symbol: $crate::lib::ops::PartialDiv<$crate::typenum::P3>,)+
                D::Kind: $crate::marker::Div,
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.cbrt(),
                }
            }

            autoconvert! {
            /// Calculates the length of the hypotenuse of a right-angle triangle given the legs.
            #[inline(always)]
            pub fn hypot<Ur>(self, other: Quantity<D, Ur, V>) -> Self
            where
                V: $crate::num::Float,
                Ur: Units<V> + ?Sized,
            {
                Self {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.hypot(change_base::<D, U, Ur, V>(&other.value)),
                }
            }}

            not_autoconvert! {
            /// Calculates the length of the hypotenuse of a right-angle triangle given the legs.
            #[inline(always)]
            pub fn hypot(self, other: Self) -> Self
            where
                V: $crate::num::Float,
            {
                Self {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.hypot(other.value),
                }
            }}}

            /// Computes the absolute value of `self`. Returns `NAN` if the quantity is
            /// `NAN`.
            #[inline(always)]
            pub fn abs(self) -> Self
            where
                V: $crate::num::Signed,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.abs(),
                }
            }

            /// Returns a quantity that represents the sign of `self`.
            ///
            /// * `1.0` of the base unit if the number is positive, `+0.0`, or `INFINITY`.
            /// * `-1.0` of the base unit if the number is negative, `-0.0`, or
            ///   `NEG_INFINITY`.
            /// * `NAN` if the number is `NAN`.
            #[inline(always)]
            pub fn signum(self) -> Self
            where
                V: $crate::num::Signed,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.signum(),
                }
            }

            /// Returns `true` if `self`'s sign bit is positive, including `+0.0` and
            /// `INFINITY`.
            #[cfg_attr(feature = "cargo-clippy", allow(clippy::wrong_self_convention))]
            #[inline(always)]
            pub fn is_sign_positive(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_sign_positive()
            }

            /// Returns `true` if `self`'s sign is negative, including `-0.0` and
            /// `NEG_INFINITY`.
            #[cfg_attr(feature = "cargo-clippy", allow(clippy::wrong_self_convention))]
            #[inline(always)]
            pub fn is_sign_negative(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_sign_negative()
            }

            std! {
            /// Fused multiply-add. Computes `(self * a) + b` with only one rounding error.
            /// This produces a more accurate result with better performance than a separate
            /// multiplication operation followed by an add.
            ///
            /// ## Generic Parameters
            /// * `Da`: Dimension for parameter `a`.
            /// * `Ua`: Base units for parameter `a`.
            /// * `Ub`: Base units for parameter `b`.
            #[inline(always)]
            pub fn mul_add<Da, Ua, Ub>(
                self,
                a: Quantity<Da, Ua, V>,
                b: Quantity<$quantities<$($crate::typenum::Sum<D::$symbol, Da::$symbol>),+>, Ub, V>,
            ) -> Quantity<$quantities<$($crate::typenum::Sum<D::$symbol, Da::$symbol>),+>, U, V>
            where
                $(D::$symbol: $crate::lib::ops::Add<Da::$symbol>,)+
                D::Kind: $crate::marker::Mul,
                V: $crate::num::Float,
                Da: Dimension + ?Sized,
                Da::Kind: $crate::marker::Mul,
                Ua: Units<V> + ?Sized,
                Ub: Units<V> + ?Sized,
            {
                // (self * a) + b
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.mul_add(a.value, b.value),
                }
            }}

            /// Takes the reciprocal (inverse) of a number, `1/x`.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::time::second;
            /// let f: Frequency = Time::new::<second>(1.0).recip();
            /// ```
            #[inline(always)]
            pub fn recip(
                self
            ) -> Quantity<$quantities<$($crate::typenum::Negate<D::$symbol>),+>, U, V>
            where
                $(D::$symbol: $crate::lib::ops::Neg,)+
                D::Kind: $crate::marker::Div,
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.recip(),
                }
            }

            /// Raises a quantity to an integer power.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::length::meter;
            /// let a: Area = Length::new::<meter>(3.0).powi(::uom::typenum::P2::new());
            /// ```
            ///
            /// ## Generic Parameters
            /// * `E`: `typenum::Integer` power.
            #[inline(always)]
            pub fn powi<E>(
                self, e: E
            ) -> Quantity<$quantities<$($crate::typenum::Prod<D::$symbol, E>),+>, U, V>
            where
                $(D::$symbol: $crate::lib::ops::Mul<E>,)+
                D::Kind: $crate::marker::Mul,
                E: $crate::typenum::Integer,
                V: $crate::typenum::Pow<E, Output = V> + $crate::Conversion<V>,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: $crate::typenum::Pow::powi(self.value, e),
                }
            }

            std! {
            /// Takes the square root of a number. Returns `NAN` if `self` is a negative
            /// number.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::area::square_meter;
            /// let l: Length = Area::new::<square_meter>(4.0).sqrt();
            /// ```
            ///
            /// The input type must have dimensions divisible by two:
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust,compile_fail")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::length::meter;
            /// // error[E0271]: type mismatch resolving ...
            /// let r = Length::new::<meter>(4.0).sqrt();
            /// ```
            #[inline(always)]
            pub fn sqrt(
                self
            ) -> Quantity<
                $quantities<$($crate::typenum::PartialQuot<D::$symbol, $crate::typenum::P2>),+>,
                U, V>
            where
                $(D::$symbol: $crate::typenum::PartialDiv<$crate::typenum::P2>,)+
                D::Kind: $crate::marker::Div,
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.sqrt(),
                }
            }}

            /// Returns the maximum of the two quantities.
            #[inline(always)]
            pub fn max(self, other: Self) -> Self
            where
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.max(other.value),
                }
            }

            /// Returns the minimum of the two quantities.
            #[inline(always)]
            pub fn min(self, other: Self) -> Self
            where
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.min(other.value),
                }
            }
        }

        impl<D, U, V> $crate::lib::clone::Clone for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::clone::Clone,
        {
            #[inline(always)]
            fn clone(&self) -> Self {
                match *self {
                    Quantity { ref value, .. } => {
                        Quantity {
                            dimension: $crate::lib::marker::PhantomData,
                            units: $crate::lib::marker::PhantomData,
                            value: $crate::lib::clone::Clone::clone(&(*value)),
                        }
                    }
                }
            }
        }

        impl<D, U, V> $crate::lib::marker::Copy for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::marker::Copy,
        {
        }

        #[allow(non_camel_case_types)]
        impl<D, U, V> $crate::lib::fmt::Debug for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::fmt::Debug,
        {
            fn fmt(&self, f: &mut $crate::lib::fmt::Formatter) -> $crate::lib::fmt::Result {
                self.value.fmt(f)
                $(.and_then(|_| {
                    let d = <D::$symbol as $crate::typenum::Integer>::to_i32();

                    if 0 != d {
                        write!(f, " {}^{}", U::$name::abbreviation(), d)
                    }
                    else {
                        Ok(())
                    }
                }))+
            }
        }

        impl<D, U, V> $crate::lib::default::Default for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::default::Default,
        {
            fn default() -> Self {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: V::default(),
                }
            }
        }

        impl<D, U, V> $crate::lib::cmp::Eq for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::cmp::Eq,
        {
        }

        impl<D, U, V> $crate::lib::hash::Hash for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::hash::Hash,
        {
            fn hash<H: $crate::lib::hash::Hasher>(&self, state: &mut H) {
                self.value.hash(state);
            }
        }

        impl<D, U, V> $crate::lib::ops::Neg for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            D::Kind: $crate::marker::Neg,
            U: Units<V> + ?Sized,
            V: $crate::num::Signed + $crate::Conversion<V>,
        {
            type Output = Quantity<D, U, V>;

            #[inline(always)]
            fn neg(self) -> Self::Output {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: -self.value,
                }
            }
        }

        impl<D, U, V> $crate::lib::cmp::Ord for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::cmp::Ord,
        {
            #[inline(always)]
            fn cmp(&self, other: &Self) -> $crate::lib::cmp::Ordering {
                self.value.cmp(&other.value)
            }

            #[inline(always)]
            fn max(self, other: Self) -> Self {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.max(other.value),
                }
            }

            #[inline(always)]
            fn min(self, other: Self) -> Self {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.min(other.value),
                }
            }
        }

        autoconvert! {
        impl<D, Ul, Ur, V> $crate::lib::cmp::PartialEq<Quantity<D, Ur, V>> for Quantity<D, Ul, V>
        where
            D: Dimension + ?Sized,
            Ul: Units<V> + ?Sized,
            Ur: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            #[inline(always)]
            fn eq(&self, other: &Quantity<D, Ur, V>) -> bool {
                self.value == change_base::<D, Ul, Ur, V>(&other.value)
            }
        }}

        not_autoconvert! {
        impl<D, U, V> $crate::lib::cmp::PartialEq for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }}

        autoconvert! {
        impl<D, Ul, Ur, V> $crate::lib::cmp::PartialOrd<Quantity<D, Ur, V>> for Quantity<D, Ul, V>
        where
            D: Dimension + ?Sized,
            Ul: Units<V> + ?Sized,
            Ur: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::cmp::PartialOrd,
        {
            #[inline(always)]
            fn partial_cmp(
                &self, other: &Quantity<D, Ur, V>
            ) -> Option<$crate::lib::cmp::Ordering>
            {
                self.value.partial_cmp(&change_base::<D, Ul, Ur, V>(&other.value))
            }

            #[inline(always)]
            fn lt(&self, other: &Quantity<D, Ur, V>) -> bool {
                self.value.lt(&change_base::<D, Ul, Ur, V>(&other.value))
            }

            #[inline(always)]
            fn le(&self, other: &Quantity<D, Ur, V>) -> bool {
                self.value.le(&change_base::<D, Ul, Ur, V>(&other.value))
            }

            #[inline(always)]
            fn gt(&self, other: &Quantity<D, Ur, V>) -> bool {
                self.value.gt(&change_base::<D, Ul, Ur, V>(&other.value))
            }

            #[inline(always)]
            fn ge(&self, other: &Quantity<D, Ur, V>) -> bool {
                self.value.ge(&change_base::<D, Ul, Ur, V>(&other.value))
            }
        }}

        not_autoconvert! {
        impl<D, U, V> $crate::lib::cmp::PartialOrd for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::cmp::PartialOrd,
        {
            #[inline(always)]
            fn partial_cmp(&self, other: &Self) -> Option<$crate::lib::cmp::Ordering> {
                self.value.partial_cmp(&other.value)
            }

            #[inline(always)]
            fn lt(&self, other: &Self) -> bool {
                self.value.lt(&other.value)
            }

            #[inline(always)]
            fn le(&self, other: &Self) -> bool {
                self.value.le(&other.value)
            }

            #[inline(always)]
            fn gt(&self, other: &Self) -> bool {
                self.value.gt(&other.value)
            }

            #[inline(always)]
            fn ge(&self, other: &Self) -> bool {
                self.value.ge(&other.value)
            }
        }}

        autoconvert! {
        impl<D, Ul, Ur, V> $crate::lib::ops::Rem<Quantity<D, Ur, V>> for Quantity<D, Ul, V>
        where
            D: Dimension + ?Sized,
            D::Kind: $crate::marker::Rem,
            Ul: Units<V> + ?Sized,
            Ur: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            type Output = Quantity<D, Ul, V>;

            #[inline(always)]
            fn rem(self, rhs: Quantity<D, Ur, V>) -> Self::Output {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value % change_base::<D, Ul, Ur, V>(&rhs.value)
                }
            }
        }}

        not_autoconvert! {
        impl<D, U, V> $crate::lib::ops::Rem for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            D::Kind: $crate::marker::Rem,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            type Output = Self;

            #[inline(always)]
            fn rem(self, rhs: Self) -> Self::Output {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value % rhs.value
                }
            }
        }}

        autoconvert! {
        impl<D, Ul, Ur, V> $crate::lib::ops::RemAssign<Quantity<D, Ur, V>> for Quantity<D, Ul, V>
        where
            D: Dimension + ?Sized,
            D::Kind: $crate::marker::RemAssign,
            Ul: Units<V> + ?Sized,
            Ur: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::ops::RemAssign,
        {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: Quantity<D, Ur, V>) {
                self.value %= change_base::<D, Ul, Ur, V>(&rhs.value)
            }
        }}

        not_autoconvert! {
        impl<D, U, V> $crate::lib::ops::RemAssign for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            D::Kind: $crate::marker::RemAssign,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::ops::RemAssign,
        {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: Self) {
                self.value %= rhs.value
            }
        }}

        impl<D, U, V> $crate::num::Saturating for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            D::Kind: $crate::marker::Saturating,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::num::Saturating,
        {
            fn saturating_add(self, v: Self) -> Self {
                Quantity { value: self.value.saturating_add(v.value), ..self }
            }

            fn saturating_sub(self, v: Self) -> Self {
                Quantity { value: self.value.saturating_sub(v.value), ..self }
            }
        }

        impl<D, U, V> $crate::lib::iter::Sum for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            D::Kind: $crate::marker::Add,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::iter::Sum,
        {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: iter.map(|v| { v.value }).sum(),
                }
            }
        }

        test! {
        impl<D, U, V> $crate::tests::Test for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::tests::Test,
        {
            fn assert_eq(lhs: &Self, rhs: &Self) {
                $crate::tests::Test::assert_eq(&lhs.value, &rhs.value);
            }

            fn assert_approx_eq(lhs: &Self, rhs: &Self) {
                $crate::tests::Test::assert_approx_eq(&lhs.value, &rhs.value);
            }

            fn eq(lhs: &Self, rhs: &Self) -> bool {
                $crate::tests::Test::eq(&lhs.value, &rhs.value)
            }

            fn approx_eq(lhs: &Self, rhs: &Self) -> bool {
                $crate::tests::Test::approx_eq(&lhs.value, &rhs.value)
            }
        }}

        impl<D, U, V> $crate::num::Zero for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            D::Kind: $crate::marker::Add,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            fn zero() -> Self {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: V::zero(),
                }
            }

            fn is_zero(&self) -> bool {
                self.value.is_zero()
            }
        }

        serde! {
        impl<D, U, V> $crate::serde::Serialize for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::serde::Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer
            {
                self.value.serialize(serializer)
            }
        }

        impl<'de, D, U, V> $crate::serde::Deserialize<'de> for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::serde::Deserialize<'de>,
        {
            fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
            where
                De: $crate::serde::Deserializer<'de>,
            {
                let value: V = $crate::serde::Deserialize::deserialize(deserializer)?;

                Ok(Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value,
                })
            }
        }}

        /// Utilities for formatting and printing quantities.
        pub mod fmt {
            use $crate::lib::fmt;
            use super::{Dimension, Quantity, Unit, Units, from_base};
            use $crate::num::Num;
            use $crate::Conversion;
            use $crate::fmt::DisplayStyle;

            /// A struct to specify a display style and unit.
            ///
            /// # Usage
            /// ## Indirect style
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::length::{centimeter, meter};
            /// # use uom::si::fmt::Arguments;
            /// # use uom::fmt::DisplayStyle::*;
            /// let l = Length::new::<meter>(1.0);
            /// let a = Length::format_args(centimeter, Description);
            ///
            /// assert_eq!("100 centimeters", format!("{}", a.with(l)));
            /// ```
            ///
            /// ## Direct style
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::length::{centimeter, meter};
            /// # use uom::si::fmt::Arguments;
            /// # use uom::fmt::DisplayStyle::*;
            /// let l = Length::new::<meter>(1.0);
            /// let a = l.into_format_args(centimeter, Description);
            ///
            /// assert_eq!("100 centimeters", format!("{}", a));
            /// ```
            ///
            /// ## Generic Parameters
            /// * `D`: Dimension.
            /// * `N`: Unit.
            #[allow(missing_debug_implementations)] // Prevent accidental direct use.
            pub struct Arguments<D, N>
            where
                D: Dimension + ?Sized,
                N: Unit,
            {
                pub(super) dimension: $crate::lib::marker::PhantomData<D>,
                pub(super) unit: N,
                pub(super) style: DisplayStyle,
            }

            /// A struct to specify a display style and unit for a given quantity.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::length::{centimeter, meter};
            /// # use uom::si::fmt::Arguments;
            /// # use uom::fmt::DisplayStyle::*;
            /// let l = Length::new::<meter>(1.0);
            /// let a = l.into_format_args(centimeter, Description);
            ///
            /// assert_eq!("100 centimeters", format!("{}", a));
            /// ```
            ///
            /// ## Generic Parameters
            /// * `D`: Dimension.
            /// * `U`: Base units.
            /// * `V`: Value underlying storage type.
            /// * `N`: Unit.
            pub struct QuantityArguments<D, U, V, N>
            where
                D: Dimension + ?Sized,
                U: Units<V> + ?Sized,
                V: Num + Conversion<V>,
                N: Unit,
            {
                pub(super) arguments: Arguments<D, N>,
                pub(super) quantity: Quantity<D, U, V>,
            }

            impl<D, N> $crate::lib::clone::Clone for Arguments<D, N>
            where
                D: Dimension + ?Sized,
                N: Unit,
            {
                fn clone(&self) -> Self {
                    Self {
                        dimension: $crate::lib::marker::PhantomData,
                        unit: self.unit.clone(),
                        style: self.style.clone(),
                    }
                }
            }

            impl<D, N> $crate::lib::marker::Copy for Arguments<D, N>
            where
                D: Dimension + ?Sized,
                N: Unit,
            {
            }

            impl<D, U, V, N> $crate::lib::clone::Clone for QuantityArguments<D, U, V, N>
            where
                D: Dimension + ?Sized,
                U: Units<V> + ?Sized,
                V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::clone::Clone,
                N: Unit,
            {
                fn clone(&self) -> Self {
                    Self {
                        arguments: self.arguments.clone(),
                        quantity: self.quantity.clone(),
                    }
                }
            }

            impl<D, U, V, N> $crate::lib::marker::Copy for QuantityArguments<D, U, V, N>
            where
                D: Dimension + ?Sized,
                U: Units<V> + ?Sized,
                V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::marker::Copy,
                N: Unit,
            {
            }

            macro_rules! format_arguments {
                ($style:ident) => {
                    impl<D, U, V, N> fmt::$style for QuantityArguments<D, U, V, N>
                    where
                        D: Dimension + ?Sized,
                        U: Units<V> + ?Sized,
                        V: Num + Conversion<V> + fmt::$style,
                        N: Unit + Conversion<V, T = V::T>,
                    {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            let value = from_base::<D, U, V, N>(&self.quantity.value);

                            value.fmt(f)?;
                            write!(f, " {}",
                                match self.arguments.style {
                                    DisplayStyle::Abbreviation => N::abbreviation(),
                                    DisplayStyle::Description => {
                                        if value.is_one() { N::singular() } else { N::plural() }
                                    },
                                })
                        }
                    }
                };
            }

            format_arguments!(Binary);
            format_arguments!(Debug);
            format_arguments!(Display);
            format_arguments!(LowerExp);
            format_arguments!(LowerHex);
            format_arguments!(Octal);
            format_arguments!(UpperExp);
            format_arguments!(UpperHex);
        }

        /// Macro to implement [`quantity`](si/struct.Quantity.html) type aliases for a specific
        /// [system of units][units] and value storage type.
        ///
        /// * `$path`: Path to the module where the [`system!`](macro.system.html) macro was run
        ///   (e.g. `::uom::si`).
        /// * `$V`: Underlying value storage type (e.g. `f32`).
        /// * `$U`: Optional. Base units. Pass as a tuple with the desired units: `(meter, kilogram,
        ///   second, ampere, kelvin, mole, candela)`. The system's base units will be used if no
        ///   value is provided. Note that a unit with a non-zero constant factor is not currently
        ///   supported as a base unit.
        ///
        /// An example invocation is given below for a meter-kilogram-second system setup in the
        /// module `mks` with a system of quantities name `Q`. The `#[macro_use]` attribute must be
        /// used when including the `uom` crate to make macros for predefined systems available.
        /// The optional units parameter to change the base units is included commented out.
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
        /// mod f32 {
        ///     mod mks {
        ///         pub use super::super::*;
        ///     }
        ///
        ///     // `crate::mks` works in Rust 1.30.0 or later. `mod mks {...}` workaround is needed
        ///     // to support older versions of Rust and the 2018 edition at the same time.
        ///     Q!(self::mks, f32/*, (centimeter, gram, second)*/);
        /// }
        /// # }
        /// ```
        ///
        /// [units]: http://jcgm.bipm.org/vim/en/1.13.html
        #[macro_export]
        macro_rules! $quantities {
            ($path:path) => {
                use $path as __system;

                $(/// [`Quantity`](struct.Quantity.html) type alias using the default base units
                /// parameterized on the underlying storage type.
                ///
                /// ## Generic Parameters
                /// * `V`: Underlying storage type.
                #[allow(dead_code)]
                #[allow(unused_qualifications)]
                pub type $quantity<V> = __system::$module::$quantity<__system::$units<V>, V>;)+
            };
            ($path:path, $V:ty) => {
                use $path as __system;

                $(/// [`Quantity`](struct.Quantity.html) type alias using the default base units.
                #[allow(dead_code)]
                #[allow(unused_qualifications)]
                pub type $quantity = __system::$module::$quantity<__system::$units<$V>, $V>;)+
            };
            ($path:path, $V:ty, $U:tt) => {
                system!(@quantities $path, $V; $($name),+; $U; $($module::$quantity),+);
            };
        }
    };
    (
        @quantities $path:path,
        $V:ty;
        $($name:ident),+;
        ($($U:ident),+);
        $($module:ident::$quantity:ident),+
    ) => {
        use $path as __system;

        type Units = dyn __system::Units<$V, $($name = __system::$name::$U,)+>;

        $(/// [`Quantity`](struct.Quantity.html) type alias using the given base units.
        #[allow(dead_code)]
        #[allow(unused_qualifications)]
        pub type $quantity = __system::$module::$quantity<Units, $V>;)+
    };
    (@replace $_t:tt $sub:ty) => { $sub };
}
