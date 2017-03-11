/// Macro to implement a [system of quantities](http://jcgm.bipm.org/vim/en/1.3.html).
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
/// * `$module`: Optional. Module name of the quantity. A `#[macro_use] pub mod $module;` statement
///   is generated if this variable is given.
/// * `$quantity`: Quantity name (e.g. `Length`, `Mass`, ...).
///
/// An example invocation is given below for a meter-kilogram-second system.
///
/// ```
/// # #[macro_use] extern crate uom;
/// # fn main() { }
/// # mod mks {
/// #     #[macro_use]
/// #     mod length {
/// #         quantity! {
/// #             /// Length (base unit meter, m^(1)).
/// #             quantity: mks_Length;
/// #             /// Length dimension, m^(1).
/// #             dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @meter: 1.0E0;
/// #                 @foot: 3.048E-1;
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod mass {
/// #         quantity! {
/// #             /// Mass (base unit kilogram, kg^(1)).
/// #             quantity: mks_Mass;
/// #             /// Mass dimension, kg^(1).
/// #             dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @kilogram: 1.0;
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod time {
/// #         quantity! {
/// #             /// Time (base unit second, s^(1)).
/// #             quantity: mks_Time;
/// #             /// Time dimension, s^(1).
/// #             dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
/// #             units {
/// #                 @second: 1.0;
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
///         mks_Length,
///         mks_Mass,
///         mks_Time,
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
    ) =>
    {
        $(#[macro_use]
        pub mod $module;)+

        system! {
            $(#[$quantities_attr])*
            quantities: $quantities {
                $($name: $unit, $symbol;)+
            }
            $(#[$units_attr])*
            units: $units {
                $($quantity,)+
            }
        }
    };
    (
        $(#[$quantities_attr:meta])* quantities: $quantities:ident {
            $($name:ident: $unit:ident, $symbol:ident;)+
        }
        $(#[$units_attr:meta])* units: $units:ident {
            $($quantity:ident,)+
        }
    ) =>
    {
        mod system { pub use super::*; }

        /// Property of a phenomenon, body or substance, where the property has a magnitude that
        /// can be expressed as a number and a reference.
        ///
        /// * http://jcgm.bipm.org/vim/en/1.1.html
        #[derive(Copy, Clone, Debug)]
        pub struct Quantity<D, U, V>
            where D: Dimension,
                U: Units<D, V>,
        {
            dimension: $crate::stdlib::marker::PhantomData<D>,
            units: $crate::stdlib::marker::PhantomData<U>,
            value: V,
        }

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
            where D: Dimension,
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
        pub trait Unit<V> {
            /// Conversion factor for the given unit to the base unit for the quantity.
            fn conversion() -> V;
        }

        $(#[$quantities_attr])*
        #[derive(Clone, Copy, Debug)]
        pub struct $quantities<$($symbol),+>
            where $($symbol: $crate::typenum::Integer,)+ {
            $($name: $crate::stdlib::marker::PhantomData<$symbol>),+
        }

        /// Type alias for [dimension one][one] for which all the exponents of the factors
        /// corresponding to the [base quantities][base] are zero.
        ///
        /// [one]: http://jcgm.bipm.org/vim/en/1.8.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.4.html
        pub type One = $quantities<$(replace_ty!($symbol $crate::typenum::Z0)),+>;

        impl<$($symbol),+> Dimension for $quantities<$($symbol),+>
            where $($symbol: $crate::typenum::Integer,)+ {
        }

        /// Marker struct to identify the [base units][base] of the
        /// [system of quantities][quantities] to be used in the internal representation of a
        /// [quantity][quantity] value.
        ///
        /// [base]: http://jcgm.bipm.org/vim/en/1.10.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        #[derive(Clone, Copy, Debug)]
        pub struct BaseUnits<$($symbol,)+ V>
            where $($symbol: system::$name::Unit<V>,)+ {
            $($name: $crate::stdlib::marker::PhantomData<$symbol>,)+
            value: $crate::stdlib::marker::PhantomData<V>,
        }

        $(#[$units_attr])*
        pub type $units<V> = BaseUnits<$(system::$name::$unit),+, V>;

        #[doc(hidden)]
        macro_rules! impl_marker_ops {
            ($Trait:ident, $fun:ident) => {
                #[allow(non_camel_case_types)]
                impl<$($name,)+ $($symbol),+> $crate::stdlib::ops::$Trait<$quantities<$($name),+>>
                    for $quantities<$($symbol),+>
                    where $($name: $crate::typenum::Integer,)+
                          $($symbol: $crate::typenum::Integer
                            + $crate::stdlib::ops::$Trait<$name>,)+
                          $(<$symbol as $crate::stdlib::ops::$Trait<$name>>::Output:
                            $crate::typenum::Integer,)+
                {
                    type Output = $quantities<
                        $(<$symbol as $crate::stdlib::ops::$Trait<$name>>::Output),+>;

                    fn $fun(self, _rhs: $quantities<$($name),+>) -> Self::Output {
                        unreachable!()
                    }
                }

                #[allow(non_camel_case_types)]
                impl<$($name,)+ $($symbol,)+ V> $crate::stdlib::ops::$Trait<BaseUnits<$($name,)+ V>>
                    for BaseUnits<$($symbol,)+ V>
                    where $($name: system::$name::Unit<V>,)+
                        $($symbol: system::$name::Unit<V>,)+
                {
                    type Output = BaseUnits<$($symbol,)+ V>;

                    fn $fun(self, _rhs: BaseUnits<$($name,)+ V>) -> Self::Output {
                        unreachable!();
                    }
                }
            };
        }
        impl_marker_ops!(Sub, sub);
        impl_marker_ops!(Add, add);

        #[doc(hidden)]
        macro_rules! impl_ops {
            (
                $AddSubTrait:ident, $addsub_fun:ident, $addsub_op:tt,
                $AddSubAssignTrait:ident, $addsubassign_fun:ident, $addsubassign_op:tt,
                $MulDivTrait:ident, $muldiv_fun:ident, $muldiv_op:tt,
                $MulDivAssignTrait:ident, $muldivassign_fun:ident, $muldivassign_op:tt,
                $V:ty
            ) =>
            {
                impl<D, Ul, Ur> $crate::stdlib::ops::$AddSubTrait<Quantity<D, Ur, $V>>
                    for Quantity<D, Ul, $V>
                    where D: Dimension,
                          Ul: Units<D, $V>,
                          Ur: Units<D, $V>,
                {
                    type Output = Quantity<D, Ul, $V>;

                    #[inline(always)]
                    fn $addsub_fun(self, rhs: Quantity<D, Ur, $V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value
                                $addsub_op ((<Ur as Units<D, $V>>::conversion()
                                        / <Ul as Units<D, $V>>::conversion())
                                    * rhs.value),
                        }
                    }
                }

                impl<D, Ul, Ur> $crate::stdlib::ops::$AddSubAssignTrait<Quantity<D, Ur, $V>>
                    for Quantity<D, Ul, $V>
                    where D: Dimension,
                          Ul: Units<D, $V>,
                          Ur: Units<D, $V>,
                {
                    #[inline(always)]
                    fn $addsubassign_fun(&mut self, rhs: Quantity<D, Ur, $V>) {
                        self.value $addsubassign_op (<Ur as Units<D, $V>>::conversion()
                                / <Ul as Units<D, $V>>::conversion())
                            * rhs.value;
                    }
                }

                impl<Dl, Dr, Ul, Ur> $crate::stdlib::ops::$MulDivTrait<Quantity<Dr, Ur, $V>>
                    for Quantity<Dl, Ul, $V>
                    where Dl: Dimension + $crate::stdlib::ops::$AddSubTrait<Dr>,
                          Dr: Dimension,
                          Ul: Units<Dl, $V> + $crate::stdlib::ops::$AddSubTrait<Ur>,
                          Ur: Units<Dr, $V>,
                          <Dl as $crate::stdlib::ops::$AddSubTrait<Dr>>::Output: Dimension,
                          <Ul as $crate::stdlib::ops::$AddSubTrait<Ur>>::Output:
                            Units<<Dl as $crate::stdlib::ops::$AddSubTrait<Dr>>::Output, $V>,
                {
                    type Output = Quantity<<Dl as $crate::stdlib::ops::$AddSubTrait<Dr>>::Output,
                        <Ul as $crate::stdlib::ops::$AddSubTrait<Ur>>::Output,
                        $V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: Quantity<Dr, Ur, $V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value
                                $muldiv_op ((<Ur as Units<Dr, $V>>::conversion()
                                        / <Ul as Units<Dl, $V>>::conversion())
                                    * rhs.value),
                        }
                    }
                }

                impl<D, U> $crate::stdlib::ops::$MulDivTrait<$V> for Quantity<D, U, $V>
                    where D: Dimension + $crate::stdlib::ops::$AddSubTrait<One>,
                          U: Units<D, $V> + $crate::stdlib::ops::$AddSubTrait<U>,
                          <D as $crate::stdlib::ops::$AddSubTrait<One>>::Output: Dimension,
                          <U as $crate::stdlib::ops::$AddSubTrait<U>>::Output:
                            Units<<D as $crate::stdlib::ops::$AddSubTrait<One>>::Output, $V>,
                {
                    type Output = Quantity<<D as $crate::stdlib::ops::$AddSubTrait<One>>::Output,
                        <U as $crate::stdlib::ops::$AddSubTrait<U>>::Output,
                        $V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: $V) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value $muldiv_op rhs,
                        }
                    }
                }

                impl<D, U> $crate::stdlib::ops::$MulDivTrait<Quantity<D, U, $V>> for $V
                    where D: Dimension,
                          U: Units<D, $V> + $crate::stdlib::ops::$AddSubTrait<U>,
                          One: $crate::stdlib::ops::$AddSubTrait<D>,
                          <One as $crate::stdlib::ops::$AddSubTrait<D>>::Output: Dimension,
                          <U as $crate::stdlib::ops::$AddSubTrait<U>>::Output:
                            Units<<One as $crate::stdlib::ops::$AddSubTrait<D>>::Output, $V>,
                {
                    type Output = Quantity<<One as $crate::stdlib::ops::$AddSubTrait<D>>::Output,
                        <U as $crate::stdlib::ops::$AddSubTrait<U>>::Output,
                        $V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: Quantity<D, U, $V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self $muldiv_op rhs.value,
                        }
                    }
                }

                impl<D, U> $crate::stdlib::ops::$MulDivAssignTrait<$V> for Quantity<D, U, $V>
                    where D: Dimension,
                          U: Units<D, $V>,
                {
                    #[inline(always)]
                    fn $muldivassign_fun(&mut self, rhs: $V) {
                        self.value $muldivassign_op rhs;
                    }
                }
            };
        }

        #[doc(hidden)]
        macro_rules! impl_units {
            ($V:ty) => {
                #[allow(non_camel_case_types)]
                impl<$($name,)+ $($symbol),+> Units<$quantities<$($name),+>, $V>
                    for BaseUnits<$($symbol,)+ $V>
                    where $($name: $crate::typenum::Integer,)+
                          $($symbol: system::$name::Unit<$V>,)+
                {
                    #[inline(always)]
                    fn conversion() -> $V {
                        #[cfg(not(feature = "std"))]
                        #[allow(unused_imports)]
                        use $crate::stdlib::num::*;

                        1.0 $(* <$symbol as Unit<$V>>::conversion().powi($name::to_i32()))+
                    }
                }

                impl_ops!(Add, add, +, AddAssign, add_assign, +=,
                    Mul, mul, *, MulAssign, mul_assign, *=,
                    $V);
                impl_ops!(Sub, sub, +, SubAssign, sub_assign, -=,
                    Div, div, /, DivAssign, div_assign, /=,
                    $V);

                impl<D, U> $crate::stdlib::ops::Neg for Quantity<D, U, $V>
                    where D: Dimension,
                          U: Units<D, $V>
                {
                    type Output = Quantity<D, U, $V>;

                    #[inline(always)]
                    fn neg(self) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: -self.value,
                        }
                    }
                }

                impl<D, Ul, Ur> $crate::stdlib::ops::Rem<Quantity<D, Ur, $V>>
                    for Quantity<D, Ul, $V>
                    where D: Dimension,
                          Ul: Units<D, $V>,
                          Ur: Units<D, $V>,
                {
                    type Output = Quantity<D, Ul, $V>;

                    #[inline(always)]
                    fn rem(self, rhs: Quantity<D, Ur, $V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value
                                % ((<Ur as Units<D, $V>>::conversion()
                                        / <Ul as Units<D, $V>>::conversion())
                                    * rhs.value),
                        }
                    }
                }

                impl<D, Ul, Ur> $crate::stdlib::ops::RemAssign<Quantity<D, Ur, $V>>
                    for Quantity<D, Ul, $V>
                    where D: Dimension,
                          Ul: Units<D, $V>,
                          Ur: Units<D, $V>,
                {
                    #[inline(always)]
                    fn rem_assign(&mut self, rhs: Quantity<D, Ur, $V>) {
                        self.value %= (<Ur as Units<D, $V>>::conversion()
                                / <Ul as Units<D, $V>>::conversion())
                            * rhs.value
                    }
                }
            };
        }
        #[cfg(feature = "f32")]
        impl_units!(f32);
        #[cfg(feature = "f64")]
        impl_units!(f64);

        /// Macro to implement [quantity](si/struct.Quantity.html) type aliases for a specific
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
        /// module `mks` with a system of quantities name `Q`. The optional units parameter to
        /// change the base units is included commented out.
        ///
        /// ```
        /// # #[macro_use] extern crate uom;
        /// # fn main() { }
        /// # mod mks {
        /// #     #[macro_use]
        /// #     mod length {
        /// #         quantity! {
        /// #             /// Length (base unit meter, m^(1)).
        /// #             quantity: mks_Length;
        /// #             /// Length dimension, m^(1).
        /// #             dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
        /// #             units {
        /// #                 @meter: 1.0E0;
        /// #                 @foot: 3.048E-1;
        /// #             }
        /// #         }
        /// #     }
        /// #     #[macro_use]
        /// #     mod mass {
        /// #         quantity! {
        /// #             /// Mass (base unit kilogram, kg^(1)).
        /// #             quantity: mks_Mass;
        /// #             /// Mass dimension, kg^(1).
        /// #             dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
        /// #             units {
        /// #                 @kilogram: 1.0;
        /// #             }
        /// #         }
        /// #     }
        /// #     #[macro_use]
        /// #     mod time {
        /// #         quantity! {
        /// #             /// Time (base unit second, s^(1)).
        /// #             quantity: mks_Time;
        /// #             /// Time dimension, s^(1).
        /// #             dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
        /// #             units {
        /// #                 @second: 1.0;
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
        /// #             mks_Length,
        /// #             mks_Mass,
        /// #             mks_Time,
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
            ($path:path, $V:ty) => {
                quantities!($path, $V, $units; $($quantity),+);
            };
            ($path:path, $V:ty, $U:tt) => {
                quantities!($path, $V; $($name),+; $U; $($quantity),+);
            };
        }
    };
}

/// Macro to implement a [quantity][quantity] and associated [measurement units][measurement]. Note
/// that this macro must be executed in direct submodules of the module where the
/// [`system!`](macro.system.html) macro was executed.
///
/// * `$quantity_attr`: Quantity attributes. Generally used to set documentation comments for the
///   quantity.
/// * `$quantity`: Quantity name (e.g. `Length`).
/// * `$dim_attr`: Dimension attributes. Generally used to set documentation comments for the
///   quantity's dimension type alias.
/// * `$system`: System of quantities type (e.g. `ISQ`).
/// * `$dimension`: Power of a factor for each base quantity in the system. Power should be
///   represented as a `typenum` type-level integer (e.g. `N1`, `Z0`, `P1`, `P2`, ...).
/// * `$unit`: Unit name (e.g. `meter`, `foot`).
/// * `$conversion`: Conversion from the unit to the base unit of the quantity (e.g. `3.048E-1` to
///   convert `foot` to `meter`).
///
/// An example invocation is given below for the quantity of length in a meter-kilogram-second
/// system.
///
/// ```
/// # #[macro_use] extern crate uom;
/// # fn main() { }
/// # mod mks {
/// #[macro_use]
/// mod length {
///     quantity! {
///         /// Length (base unit meter, m^(1)).
///         quantity: mks_Length;
///         /// Length dimension, m^(1).
///         dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
///         units {
///             @meter: 1.0E0;
///             @foot: 3.048E-1;
///         }
///     }
/// }
/// #     #[macro_use]
/// #     mod mass {
/// #         quantity! {
/// #             /// Mass (base unit kilogram, kg^(1)).
/// #             quantity: mks_Mass;
/// #             /// Mass dimension, kg^(1).
/// #             dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @kilogram: 1.0;
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod time {
/// #         quantity! {
/// #             /// Time (base unit second, s^(1)).
/// #             quantity: mks_Time;
/// #             /// Time dimension, s^(1).
/// #             dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
/// #             units {
/// #                 @second: 1.0;
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
/// #             mks_Length,
/// #             mks_Mass,
/// #             mks_Time,
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
        $(#[$quantity_attr:meta])* quantity: $quantity:ident;
        $(#[$dim_attr:meta])* dimension: $system:ident<$($dimension:ident),+>;
        units {
            $($(#[$unit_attr:meta])* @$unit:ident: $conversion:expr;)+
        }
    ) =>
    {
        $(#[$dim_attr])*
        #[allow(dead_code)]
        pub type Dimension = super::$system<$($crate::typenum::$dimension),+>;

        $(#[$quantity_attr])*
        #[allow(dead_code)]
        pub type $quantity<U, V> = super::Quantity<Dimension, U, V>;

        /// Marker trait to identify measurement units for the quantity. See
        /// [Unit](../trait.Unit.html).
        pub trait Unit<V>: super::Unit<V> {}

        $(unit!($(#[$unit_attr])* @$unit: $conversion);)+

        #[doc(hidden)]
        macro_rules! impl_quantity {
            ($V:ty) => {
                impl<U> $quantity<U, $V>
                    where U: super::Units<Dimension, $V>,
                {
                    /// Create a new quantity from the given value and measurement unit.
                    #[inline(always)]
                    pub fn new<N>(v: $V, _unit: N) -> Self
                        where N: Unit<$V>,
                    {
                        $quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: v * (<N as super::Unit<$V>>::conversion()
                                    / <U as super::Units<Dimension, $V>>::conversion()),
                        }
                    }

                    /// Retrieve the value of the quantity in the given measurement unit.
                    #[inline(always)]
                    pub fn get<N>(self, _unit: N) -> $V
                        where N: Unit<$V>,
                    {
                        self.value * <U as super::Units<Dimension, $V>>::conversion()
                            / <N as super::Unit<$V>>::conversion()
                    }
                }

                $(impl Unit<$V> for $unit {}

                impl super::Unit<$V> for $unit {
                    #[inline(always)]
                    fn conversion() -> $V {
                        $conversion
                    }
                })+
            };
        }
        #[cfg(feature = "f32")]
        impl_quantity!(f32);
        #[cfg(feature = "f64")]
        impl_quantity!(f64);

        #[doc(hidden)]
        #[macro_export]
        macro_rules! $quantity {
            ($U:ty, $V:ty) => {
                $(#[$quantity_attr])*
                #[allow(dead_code)]
                pub type $quantity =
                    system::Quantity<system::$system<$($crate::typenum::$dimension),+>, $U, $V>;
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! quantities {
    ($path:path, $V:ty, $U:ident; $($quantity:ident),+) => {
        use $path as system;

        $($quantity!(system::$U<$V>, $V);)+
    };
    ($path:path, $V:ty; $($name:ident),+; ($($U:ident),+); $($quantity:ident),+) => {
        use $path as system;

        type Units = system::BaseUnits<$(system::$name::$U,)+ $V>;

        $($quantity!(Units, $V);)+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! unit {
    ($(#[$unit_attr:meta])+ @$unit:ident: $conversion:expr) => {
        $(#[$unit_attr])*
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug)]
        pub struct $unit;
    };
    (@$unit:ident: $conversion:expr) => {
        /// Measurement unit.
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug)]
        pub struct $unit;
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! replace_ty {
    ($_t:tt $sub:ty) => { $sub };
}
