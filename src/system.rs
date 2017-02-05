/// Macro to implement a system of quantities.
///
/// * http://jcgm.bipm.org/vim/en/1.3.html
#[macro_export]
macro_rules! system {
    ($(#[$attr:meta])* quantities: $quantities:ident { $($name:ident: $symbol:ident,)+ }) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug)]
        pub struct $quantities<$($symbol),+>
            where $($symbol: $crate::typenum::Integer,)+ {
            $($name: $crate::stdlib::marker::PhantomData<$symbol>),+
        }

        impl<$($symbol),+> $crate::Dimension for $quantities<$($symbol),+>
            where $($symbol: $crate::typenum::Integer,)+ {
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
    };
}
