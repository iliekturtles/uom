/// Macro to implement a [system of quantities](http://jcgm.bipm.org/vim/en/1.3.html).
#[macro_export]
macro_rules! system {
    ($(#[$attr:meta])* quantities: $quantities:ident { $($name:ident: $symbol:ident,)+ }) => {
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

        $(#[$attr])*
        #[derive(Clone, Copy, Debug)]
        pub struct $quantities<$($symbol),+>
            where $($symbol: $crate::typenum::Integer,)+ {
            $($name: $crate::stdlib::marker::PhantomData<$symbol>),+
        }

        impl<$($symbol),+> Dimension for $quantities<$($symbol),+>
            where $($symbol: $crate::typenum::Integer,)+ {
        }
    };
}
