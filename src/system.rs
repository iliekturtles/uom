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
