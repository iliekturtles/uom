/// Macro to implement a [system of quantities](http://jcgm.bipm.org/vim/en/1.3.html).
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
            };
        }
        #[cfg(feature = "f32")]
        impl_units!(f32);
        #[cfg(feature = "f64")]
        impl_units!(f64);

        /// Macro to implement [quantity](si/struct.Quantity.html) type aliases for a specific
        /// [system of units][units] and value storage type.
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

/// Macro to implement a [quantity][quantity] and associated [measurement units][measurement].
///
/// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
/// [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
#[macro_export]
macro_rules! quantity {
    (
        $(#[$quantity_attr:meta])* quantity: $quantity:ident;
        $(#[$dim_attr:meta])* dimension: $system:ident<$($dimension:ident),+>;
        units {
            $($unit:ident: $conversion:expr;)+
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

        $(/// Measurement unit.
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug)]
        pub struct $unit;)+

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
