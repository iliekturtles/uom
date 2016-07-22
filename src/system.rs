macro_rules! system {
    ($system:ident: dimensions { $($name:ident: $symbol:ident, $unit:ident, $abbrev:ident;)* }) => {
        use core::marker::{PhantomData};
        use typenum::{Integer};

        pub struct $system<$($symbol, concat_idents!($symbol, d): Integer),*> {
            $($name: PhantomData<($symbol, concat_idents!($symbol, d))>),*
        }
    };
}
