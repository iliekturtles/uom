#[macro_export]
macro_rules! system {
    ($system:ident: dimensions {$($name:ident: $symbol:ident),+}) => {
        use core::marker::{PhantomData};
        use typenum::{Integer, Z0, P1};

        pub struct $system<$($symbol: Integer = Z0),+> {
            $($name: PhantomData<$symbol>),+
        }
    };
}
