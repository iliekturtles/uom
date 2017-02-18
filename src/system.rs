/// Macro to implement a system of quantities.
///
/// * http://jcgm.bipm.org/vim/en/1.3.html
#[macro_export]
macro_rules! system {
    (
        $(#[$quantities_attr:meta])* quantities: $quantities:ident {
            $($name:ident: $unit:ident, $symbol:ident;)+
        }
        $(#[$units_attr:meta])* units: $units:ident {
            $($module:ident::$quantity:ident,)+
        }) =>
    {
        mod system { pub use super::*; }

        $(#[$quantities_attr])*
        #[derive(Clone, Copy, Debug)]
        pub struct $quantities<$($symbol),+>
            where $($symbol: $crate::typenum::Integer,)+ {
            $($name: $crate::stdlib::marker::PhantomData<$symbol>),+
        }

        impl<$($symbol),+> $crate::Dimension for $quantities<$($symbol),+>
            where $($symbol: $crate::typenum::Integer,)+ {
        }

        /// Marker struct to identify the base units of the system of quantities to be used in the
        /// internal representation of a quantity value.
        #[derive(Clone, Copy, Debug)]
        pub struct Units<$($symbol,)+ V>
            where $($symbol: system::$name::Unit<V>,)+ {
            $($name: $crate::stdlib::marker::PhantomData<$symbol>,)+
            value: $crate::stdlib::marker::PhantomData<V>,
        }

        $(#[$units_attr])*
        pub type $units<V> = Units<$(system::$name::$unit),+, V>;

        #[doc(hidden)]
        macro_rules! impl_units {
            ($V:ty) => {
                #[allow(non_camel_case_types)]
                impl<$($name,)+ $($symbol),+> $crate::Units<$quantities<$($name),+>, $V> for Units<$($symbol,)+ $V>
                    where $($name: $crate::typenum::Integer,)+
                          $($symbol: system::$name::Unit<$V>,)+ {
                    #[inline(always)]
                    fn conversion() -> $V {
                        #[cfg(not(feature = "std"))]
                        use $crate::stdlib::num::*;

                        (1.0 as $V)
                            $(* <$symbol as $crate::Unit<$V>>::conversion().powi($name::to_i32()))+
                    }
                }
            };
        }
        impl_units!(f32);
        impl_units!(f64);

        /// Macro to implement quantity type aliases for a specific system of units and value
        /// storage type.
        #[macro_export]
        macro_rules! $quantities {
            ($U:ty, $V:ty, $P:path) => {
                use $P as system;

                $($quantity!($U, $V, $module);)+
            };
        }
    };
}

/// Macro to implement a quantity and associated measurement units.
///
/// * http://jcgm.bipm.org/vim/en/1.9.html
#[macro_export]
macro_rules! quantity {
    ($(#[$quantity_attr:meta])* quantity: $quantity:ident;
        $(#[$dim_attr:meta])* dimension: $dimension:ty;
        units { $($unit:ident: $conversion:expr;)+ }) =>
    {
        $(#[$dim_attr])*
        pub type Dimension = $dimension;

        $(#[$quantity_attr])*
        pub type $quantity<U, V> = $crate::Quantity<Dimension, U, V>;

        /// Marker trait to identify measurement units for the quantity. See
        /// [Unit](../trait.Unit.html).
        pub trait Unit<V>: $crate::Unit<V> {}

        $(/// Measurement unit.
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug)]
        pub struct $unit;)+

        #[doc(hidden)]
        #[macro_export]
        macro_rules! $quantity {
            ($U:ty, $V:ty, $module:ident) => {
                $(#[$quantity_attr])*
                pub type $quantity = $crate::Quantity<$dimension, $U, $V>;

                $(impl system::$module::Unit<$V> for system::$module::$unit {}

                impl $crate::Unit<$V> for system::$module::$unit {
                    #[inline(always)]
                    fn conversion() -> $V {
                        $conversion as $V
                    }
                })+
            }
        }
    };
}
