/// Macro to implement a [system of quantities](http://jcgm.bipm.org/vim/en/1.3.html). `@...` match
/// arms are considered private.
///
/// * `$quantities_attr`: System of quantities attributes. Generally used to set documentation
///   comments for the system of quantities.
/// * `$quantities`: Name of the system of quantities (e.g. `ISQ`).
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
/// #             /// Length (base unit meter, m<sup>1</sup>).
/// #             quantity: Length; "length";
/// #             /// Length dimension, m<sup>1</sup>.
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
            $($name:ident: $unit:ident, $symbol:ident;)+
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
                $($name: $unit, $symbol;)+
            }
            $(#[$units_attr])*
            units: $units {
                $(mod $module::$quantity,)+
            }
        }
    };
    (
        $(#[$quantities_attr:meta])* quantities: $quantities:ident {
            $($name:ident: $unit:ident, $symbol:ident;)+
        }
        $(#[$units_attr:meta])* units: $units:ident {
            $(mod $module:ident::$quantity:ident,)+
        }
    ) => {
        /// Marker trait to express the dependence of a [quantity][quantity] on the
        /// [base quantities][base] of a [system of quantities][quantities] as a product of powers
        /// of factors corresponding to the base quantities, omitting any numerical factor.
        ///
        /// * http://jcgm.bipm.org/vim/en/1.7.html
        ///
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.4.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        pub trait Dimension {}

        /// Trait to identify a [system of units][units] based on a set of [base units][base] of a
        /// [system of quantities][quantities].
        ///
        /// [units]: http://jcgm.bipm.org/vim/en/1.13.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.10.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        pub trait Units<D, V>
        where
            D: Dimension,
        {
            /// Conversion factor for the given base units to the base units for the system of
            /// quantities.
            fn conversion() -> V;
        }

        /// Trait to identify [measurement units][measurement] of individual
        /// [quantities][quantity].
        ///
        /// [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        pub trait Unit {
            /// Unit abbreviation.
            fn abbreviation() -> &'static str;

            /// Unit singular description.
            fn singular() -> &'static str;

            /// Unit plural description.
            fn plural() -> &'static str;
        }

        /// Trait to identify conversion factors for measurement units. See
        /// [`Unit`](./trait.Unit.html).
        pub trait Conversion<V>: Unit {
            /// Conversion factor for the given unit to the base unit for the quantity.
            fn conversion() -> V;
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
        /// let L = Length::new::<meter>(1.0);
        /// ```
        ///
        /// `Quantity` fields are public to allow for the creation of `const` values and instances
        /// of non-named `Quantity`s. This functionality will be deprecated and subsequently removed
        /// once the `const fn` feature is stabilized.
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::{Quantity, ISQ, SI};
        /// # use uom::si::f32::*;
        /// # use uom::stdlib::marker::PhantomData;
        /// # use uom::typenum::{P2, Z0};
        /// // Create a `const` length of 1 meter.
        /// const L: Length = Length { dimension: PhantomData, units: PhantomData, value: 1.0, };
        /// // Create a `const` area of 1 square meter explicitly without using the `Area` alias.
        /// const A: Quantity<ISQ<P2, Z0, Z0, Z0, Z0, Z0, Z0>, SI<f32>, f32> =
        ///     Quantity { dimension: PhantomData, units: PhantomData, value: 1.0, };
        /// ```
        ///
        /// * http://jcgm.bipm.org/vim/en/1.1.html
        #[derive(Copy, Clone, Hash)]
        pub struct Quantity<D, U, V>
        where
            D: Dimension,
            U: Units<D, V>,
            V: $crate::num::Num,
        {
            /// Quantity dimension. See [`Dimension`](./trait.Dimension.html).
            pub dimension: $crate::stdlib::marker::PhantomData<D>,
            /// Quantity base units. See [`Units`](./trait.Units.html).
            pub units: $crate::stdlib::marker::PhantomData<U>,
            /// Quantity value stored in the base units for the quantity.
            pub value: V,
        }

        $(#[$quantities_attr])*
        #[allow(missing_debug_implementations)]
        #[derive(Clone, Copy, Hash)]
        pub struct $quantities<$($symbol),+>
        where
            $($symbol: $crate::typenum::Integer,)+
        {
            $($name: $crate::stdlib::marker::PhantomData<$symbol>),+
        }

        /// Marker struct to identify the [base units][base] of the
        /// [system of quantities][quantities] to be used in the internal representation of a
        /// [quantity][quantity] value.
        ///
        /// [base]: http://jcgm.bipm.org/vim/en/1.10.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        #[allow(missing_debug_implementations, non_camel_case_types)]
        #[derive(Clone, Copy, Hash)]
        pub struct BaseUnits<$($name,)+ V>
        where
            $($name: self::$name::Unit<V>,)+
        {
            $($name: $crate::stdlib::marker::PhantomData<$name>,)+
            value: $crate::stdlib::marker::PhantomData<V>,
        }

        // Type alias for dimensions where all exponents of the factors are the given value.
        type DN<N> = $quantities<$(system!(@replace $symbol N)),+>;

        /// Type alias for [dimension one][one] for which all the exponents of the factors
        /// corresponding to the [base quantities][base] are zero.
        ///
        /// [one]: http://jcgm.bipm.org/vim/en/1.8.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.4.html
        pub type One = DN<$crate::typenum::Z0>;

        $(#[$units_attr])*
        pub type $units<V> = BaseUnits<$($name::$unit),+, V>;

        #[allow(non_camel_case_types)]
        impl<$($symbol,)+ $($name,)+ V> $crate::stdlib::fmt::Debug
            for Quantity<$quantities<$($symbol),+>, BaseUnits<$($name,)+ V>, V>
        where
            $quantities<$($symbol),+>: Dimension,
            $($symbol: $crate::typenum::Integer,)+
            BaseUnits<$($name,)+ V>: Units<$quantities<$($symbol),+>, V>,
            $($name: self::$name::Unit<V>,)+
            V: $crate::num::Num + $crate::stdlib::fmt::Debug,
        {
            fn fmt(&self, f: &mut $crate::stdlib::fmt::Formatter) -> $crate::stdlib::fmt::Result {
                self.value.fmt(f)
                $(.and_then(|_| {
                    let d = $symbol::to_i32();

                    if 0 != d {
                        write!(f, " {}^{}", $name::abbreviation(), d)
                    }
                    else {
                        Ok(())
                    }
                }))+
            }
        }

        impl<$($symbol),+> Dimension for $quantities<$($symbol),+>
        where
            $($symbol: $crate::typenum::Integer,)+
        {
        }

        impl<$($symbol),+> $crate::stdlib::ops::Neg for $quantities<$($symbol),+>
        where
            $($symbol: $crate::typenum::Integer
                + $crate::stdlib::ops::Neg,)+
            $($crate::typenum::Negate<$symbol>: $crate::typenum::Integer,)+
        {
            type Output = $quantities<$($crate::typenum::Negate<$symbol>),+>;

            fn neg(self) -> Self::Output {
                unreachable!()
            }
        }

        #[doc(hidden)]
        macro_rules! impl_marker_ops {
            ($Trait:ident, $fun:ident, $alias:ident) => {
                #[allow(non_camel_case_types)]
                impl<$($symbol,)+ $($name),+> $crate::stdlib::ops::$Trait<$quantities<$($symbol),+>>
                    for $quantities<$($name),+>
                where
                    $($symbol: $crate::typenum::Integer,)+
                    $($name: $crate::typenum::Integer
                        + $crate::stdlib::ops::$Trait<$symbol>,)+
                    $($crate::typenum::$alias<$name, $symbol>: $crate::typenum::Integer,)+
                {
                    type Output = $quantities<$($crate::typenum::$alias<$name, $symbol>),+>;

                    fn $fun(self, _rhs: $quantities<$($symbol),+>) -> Self::Output {
                        unreachable!()
                    }
                }
            };
        }
        impl_marker_ops!(Add, add, Sum);
        impl_marker_ops!(Sub, sub, Diff);
        impl_marker_ops!(Mul, mul, Prod);
        impl_marker_ops!(PartialDiv, partial_div, PartialQuot);

        #[doc(hidden)]
        macro_rules! impl_ops {
            (
                $AddSubTrait:ident, $addsub_fun:ident, $addsub_op:tt,
                $AddSubAssignTrait:ident, $addsubassign_fun:ident, $addsubassign_op:tt,
                $AddSubAlias:ident,
                $MulDivTrait:ident, $muldiv_fun:ident, $muldiv_op:tt,
                $MulDivAssignTrait:ident, $muldivassign_fun:ident, $muldivassign_op:tt
            ) => {
                impl<D, Ul, Ur, V> $crate::stdlib::ops::$AddSubTrait<Quantity<D, Ur, V>>
                    for Quantity<D, Ul, V>
                where
                    D: Dimension,
                    Ul: Units<D, V>,
                    Ur: Units<D, V>,
                    V: $crate::num::Num,
                {
                    type Output = Quantity<D, Ul, V>;

                    #[inline(always)]
                    fn $addsub_fun(self, rhs: Quantity<D, Ur, V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value
                                $addsub_op (rhs.value * <Ur as Units<D, V>>::conversion()
                                    / <Ul as Units<D, V>>::conversion()),
                        }
                    }
                }

                impl<D, Ul, Ur, V> $crate::stdlib::ops::$AddSubAssignTrait<Quantity<D, Ur, V>>
                    for Quantity<D, Ul, V>
                where
                    D: Dimension,
                    Ul: Units<D, V>,
                    Ur: Units<D, V>,
                    V: $crate::num::Num + $crate::stdlib::ops::$AddSubAssignTrait<V>,
                {
                    #[inline(always)]
                    fn $addsubassign_fun(&mut self, rhs: Quantity<D, Ur, V>) {
                        self.value $addsubassign_op rhs.value * <Ur as Units<D, V>>::conversion()
                            / <Ul as Units<D, V>>::conversion();
                    }
                }

                impl<Dl, Dr, Ul, Ur, V> $crate::stdlib::ops::$MulDivTrait<Quantity<Dr, Ur, V>>
                    for Quantity<Dl, Ul, V>
                where
                    Dl: Dimension + $crate::stdlib::ops::$AddSubTrait<Dr>,
                    Dr: Dimension,
                    Ul: Units<Dl, V> + Units<Dr, V>
                        + Units<$crate::typenum::$AddSubAlias<Dl, Dr>, V>,
                    Ur: Units<Dr, V>,
                    $crate::typenum::$AddSubAlias<Dl, Dr>: Dimension,
                    V: $crate::num::Num + $crate::stdlib::ops::$MulDivTrait<V>,
                {
                    type Output = Quantity<$crate::typenum::$AddSubAlias<Dl, Dr>, Ul, V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: Quantity<Dr, Ur, V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value
                                $muldiv_op (rhs.value * <Ur as Units<Dr, V>>::conversion()
                                    / <Ul as Units<Dr, V>>::conversion()),
                        }
                    }
                }

                impl<D, U, V> $crate::stdlib::ops::$MulDivTrait<V> for Quantity<D, U, V>
                where
                    D: Dimension + $crate::stdlib::ops::$AddSubTrait<One>,
                    U: Units<D, V>
                        + Units<$crate::typenum::$AddSubAlias<D, One>, V>,
                    $crate::typenum::$AddSubAlias<D, One>: Dimension,
                    V: $crate::num::Num,
                {
                    type Output = Quantity<$crate::typenum::$AddSubAlias<D, One>, U, V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: V) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value $muldiv_op rhs,
                        }
                    }
                }

                // impl<D, U, V> $crate::stdlib::ops::$MulDivTrait<Quantity<D, U, V>> for V
                // where
                //     D: Dimension,
                //     U: Units<D, V>
                //         + Units<$crate::typenum::$AddSubAlias<One, D>, V>,
                //     One: $crate::stdlib::ops::$AddSubTrait<D>,
                //     $crate::typenum::$AddSubAlias<One, D>: Dimension,
                //     V: $crate::num::Num + $crate::stdlib::ops::$MulDivTrait<V>,
                // {
                //     type Output = Quantity<$crate::typenum::$AddSubAlias<One, D>, U, V>;

                //     #[inline(always)]
                //     fn $muldiv_fun(self, rhs: Quantity<D, U, V>) -> Self::Output {
                //         Quantity {
                //             dimension: $crate::stdlib::marker::PhantomData,
                //             units: $crate::stdlib::marker::PhantomData,
                //             value: self $muldiv_op rhs.value,
                //         }
                //     }
                // }

                impl<D, U, V> $crate::stdlib::ops::$MulDivAssignTrait<V> for Quantity<D, U, V>
                where
                    D: Dimension,
                    U: Units<D, V>,
                    V: $crate::num::Num + $crate::stdlib::ops::$MulDivAssignTrait<V>,
                {
                    #[inline(always)]
                    fn $muldivassign_fun(&mut self, rhs: V) {
                        self.value $muldivassign_op rhs;
                    }
                }
            };
        }

        impl_ops!(Add, add, +, AddAssign, add_assign, +=, Sum,
            Mul, mul, *, MulAssign, mul_assign, *=);
        impl_ops!(Sub, sub, -, SubAssign, sub_assign, -=, Diff,
            Div, div, /, DivAssign, div_assign, /=);

        impl<D, U, V> Quantity<D, U, V>
        where
            D: Dimension,
            U: Units<D, V>,
            V: $crate::num::Num,
        {
            /// Returns `true` if this value is `NAN` and `false` otherwise.
            #[cfg_attr(feature = "clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_nan(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_nan()
            }

            /// Returns `true` if this value is positive infinity or negative infinity and
            /// `false` otherwise.
            #[cfg_attr(feature = "clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_infinite(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_infinite()
            }

            /// Returns `true` if this number is neither infinite nor `NAN`.
            #[cfg_attr(feature = "clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_finite(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_finite()
            }

            /// Returns `true` if the number is neither zero, infinite, subnormal, or `NAN`.
            #[cfg_attr(feature = "clippy", allow(wrong_self_convention))]
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
            pub fn classify(self) -> $crate::stdlib::num::FpCategory
            where
                V: $crate::num::Float,
            {
                self.value.classify()
            }

            /// Takes the cubic root of a number.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::volume::cubic_meter;
            /// let l: Length = Volume::new::<cubic_meter>(8.0).cbrt();
            /// ```
            #[inline(always)]
            pub fn cbrt(
                self
            ) -> Quantity<$crate::typenum::PartialQuot<D, DN<$crate::typenum::P3>>, U, V>
            where
                D: $crate::stdlib::ops::PartialDiv<DN<$crate::typenum::P3>>,
                U: Units<$crate::typenum::PartialQuot<D, DN<$crate::typenum::P3>>, V>,
                V: $crate::num::Float,
                $crate::typenum::PartialQuot<D, DN<$crate::typenum::P3>>: Dimension,
            {
                Quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
                    value: self.value.cbrt(),
                }
            }

            /// Computes the absolute value of `self`. Returns `NAN` if the quantity is
            /// `NAN`.
            #[inline(always)]
            pub fn abs(self) -> Self
            where
                V: $crate::num::Signed,
            {
                Quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
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
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
                    value: self.value.signum(),
                }
            }

            /// Returns `true` if `self`'s sign bit is positive, including `+0.0` and
            /// `INFINITY`.
            #[cfg_attr(feature = "clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_sign_positive(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_sign_positive()
            }

            /// Returns `true` if `self`'s sign is negative, including `-0.0` and
            /// `NEG_INFINITY`.
            #[cfg_attr(feature = "clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_sign_negative(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_sign_negative()
            }

            /// Fused multiply-add. Computes `(self * a) + b` with only one rounding error.
            /// This produces a more accurate result with better performance than a separate
            /// multiplication operation followed by an add.
            #[inline(always)]
            pub fn mul_add<Da, Ua, Ub>(
                self,
                a: Quantity<Da, Ua, V>,
                b: Quantity<$crate::typenum::Sum<D, Da>, Ub, V>
            ) -> Quantity<$crate::typenum::Sum<D, Da>, U, V>
            where
                D: $crate::stdlib::ops::Add<Da>,
                U: Units<Da, V> + Units<$crate::typenum::Sum<D, Da>, V>,
                V: $crate::num::Float,
                Da: Dimension,
                Ua: Units<Da, V>,
                Ub: Units<$crate::typenum::Sum<D, Da>, V>,
                $crate::typenum::Sum<D, Da>: Dimension,
            {
                // (self * a) + b
                Quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
                    value: self.value.mul_add(a.value, b.value),
                }
            }

            /// Takes the reciprocal (inverse) of a number, `1/x`.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::time::second;
            /// let f: Frequency = Time::new::<second>(1.0).recip();
            /// ```
            #[inline(always)]
            pub fn recip(self) -> Quantity<$crate::typenum::Negate<D>, U, V>
            where
                D: $crate::stdlib::ops::Neg,
                U: Units<$crate::typenum::Negate<D>, V>,
                V: $crate::num::Float,
                $crate::typenum::Negate<D>: Dimension,
            {
                Quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
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
            #[inline(always)]
            pub fn powi<E>(
                self, e: E
            ) -> Quantity<$crate::typenum::Prod<D, DN<E>>, U, <V as $crate::typenum::Pow<E>>::Output>
            where
                D: $crate::stdlib::ops::Mul<DN<E>>,
                U: Units<$crate::typenum::Prod<D, DN<E>>, <V as $crate::typenum::Pow<E>>::Output>,
                E: $crate::typenum::Integer,
                $crate::typenum::Prod<D, DN<E>>: Dimension,
                V: $crate::typenum::Pow<E>,
                <V as $crate::typenum::Pow<E>>::Output: $crate::num::Num,
            {
                Quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
                    value: $crate::typenum::Pow::powi(self.value, e),
                }
            }

            /// Takes the square root of a number. Returns `NAN` if `self` is a negative
            /// number.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::area::square_meter;
            /// let l: Length = Area::new::<square_meter>(4.0).sqrt();
            /// ```
            #[inline(always)]
            pub fn sqrt(
                self
            ) -> Quantity<$crate::typenum::PartialQuot<D, DN<$crate::typenum::P2>>, U, V>
            where
                D: $crate::stdlib::ops::PartialDiv<DN<$crate::typenum::P2>>,
                U: Units<$crate::typenum::PartialQuot<D, DN<$crate::typenum::P2>>, V>,
                V: $crate::num::Float,
                $crate::typenum::PartialQuot<D, DN<$crate::typenum::P2>>: Dimension,
            {
                Quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
                    value: self.value.sqrt(),
                }
            }

            /// Returns the maximum of the two quantities.
            #[inline(always)]
            pub fn max(self, other: Self) -> Self
            where
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
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
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
                    value: self.value.min(other.value),
                }
            }
        }

        #[allow(non_camel_case_types)]
        impl<$($symbol,)+ $($name,)+ V> Units<$quantities<$($symbol),+>, V>
            for BaseUnits<$($name,)+ V>
        where
            $($symbol: $crate::typenum::Integer,)+
            $($name: self::$name::Unit<V>,)+
            V: $crate::num::Float,
        {
            #[inline(always)]
            fn conversion() -> V {
                V::one() $(* <$name as Conversion<V>>::conversion().powi($symbol::to_i32()))+
            }
        }

        impl<D, U, V> $crate::stdlib::ops::Neg for Quantity<D, U, V>
        where
            D: Dimension,
            U: Units<D, V>,
            V: $crate::num::Signed,
        {
            type Output = Quantity<D, U, V>;

            #[inline(always)]
            fn neg(self) -> Self::Output {
                Quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
                    value: -self.value,
                }
            }
        }

        impl<D, Ul, Ur, V> $crate::stdlib::ops::Rem<Quantity<D, Ur, V>>
            for Quantity<D, Ul, V>
        where
            D: Dimension,
            Ul: Units<D, V>,
            Ur: Units<D, V>,
            V: $crate::num::Num,
        {
            type Output = Quantity<D, Ul, V>;

            #[inline(always)]
            fn rem(self, rhs: Quantity<D, Ur, V>) -> Self::Output {
                Quantity {
                    dimension: $crate::stdlib::marker::PhantomData,
                    units: $crate::stdlib::marker::PhantomData,
                    value: self.value
                        % (rhs.value * <Ur as Units<D, V>>::conversion()
                            / <Ul as Units<D, V>>::conversion()),
                }
            }
        }

        impl<D, Ul, Ur, V> $crate::stdlib::ops::RemAssign<Quantity<D, Ur, V>>
            for Quantity<D, Ul, V>
        where
            D: Dimension,
            Ul: Units<D, V>,
            Ur: Units<D, V>,
            V: $crate::num::Num + $crate::stdlib::ops::RemAssign,
        {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: Quantity<D, Ur, V>) {
                self.value %= rhs.value * <Ur as Units<D, V>>::conversion()
                    / <Ul as Units<D, V>>::conversion()
            }
        }

        /// Macro to implement [`quantity`](si/struct.Quantity.html) type aliases for a specific
        /// [system of units][units] and value storage type.
        ///
        /// * `$path`: Path to the module where the [`system!`](macro.system.html) macro was run
        ///   (e.g. `::uom::si`).
        /// * `$V`: Underlying value storage type (e.g. `f32`).
        /// * `$U`: Optional. Base units. Pass as a tuple with the desired units: `(meter, kilogram,
        ///   second, ampere, kelvin, mole, candela)`. The system's base units will be used if no
        ///   value is provided.
        ///
        /// An example invocation is given below for a meter-kilogram-second system setup in the
        /// module `mks` with a system of quantities name `Q`. The `#[macro_use]` attribute must be
        /// used when including the `uom` crate to make macros for predefined systems available.
        /// The optional units parameter to change the base units is included commented out.
        ///
        /// ```ignore
        /// #[macro_use]
        /// extern crate uom;
        ///
        /// # fn main() { }
        /// # mod mks {
        /// #     #[macro_use]
        /// #     mod length {
        /// #         quantity! {
        /// #             /// Length (base unit meter, m<sup>1</sup>).
        /// #             quantity: Length; "length";
        /// #             /// Length dimension, m<sup>1</sup>.
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
        /// mod f32 {
        ///     Q!(mks, f32/*, (centimeter, gram, second)*/);
        /// }
        /// # }
        /// ```
        ///
        /// [units]: http://jcgm.bipm.org/vim/en/1.13.html
        #[macro_export]
        macro_rules! $quantities {
            ($path:path) => {
                use $path as system;

                $(/// [`Quantity`](struct.Quantity.html) type alias using the default base units
                /// parameterized on the underlying storage type.
                #[allow(dead_code)]
                pub type $quantity<V> = system::$module::$quantity<system::$units<V>, V>;)+
            };
            ($path:path, $V:ty) => {
                use $path as system;

                $(/// [`Quantity`](struct.Quantity.html) type alias using the default base units.
                #[allow(dead_code)]
                pub type $quantity = system::$module::$quantity<system::$units<$V>, $V>;)+
            };
            ($path:path, $V:ty, $U:tt) => {
                system!(@quantities $path, $V; $($name),+; $U; $($module::$quantity),+);
            };
        }
    };
    (@quantities $path:path, $V:ty; $($name:ident),+; ($($U:ident),+); $($module:ident::$quantity:ident),+) => {
        use $path as system;

        type Units = system::BaseUnits<$(system::$name::$U,)+ $V>;

        $(/// [`Quantity`](struct.Quantity.html) type alias using the given base units.
        #[allow(dead_code)]
        pub type $quantity = system::$module::$quantity<Units, $V>;)+
    };
    (@replace $_t:tt $sub:ty) => { $sub };
}
