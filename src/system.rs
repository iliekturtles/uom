#[macro_export]
macro_rules! system {
    ($system:ident; $units:ident: dimensions { $($name:ident: $symbol:ident, $unit:ident;)+ }) => {
        use core::marker::{PhantomData};
        use core::ops::{Sub};
        use typenum::{Integer, Z0};
        use $crate::{Dimensions, Unit, Units};

        $(pub trait $unit: Unit {})+

        #[derive(Clone, Copy, Debug)]
        pub struct $system<$($symbol: Integer = Z0),+> {
            $($name: PhantomData<$symbol>),+
        }

        #[derive(Debug)]
        pub struct $units<$($symbol: $unit),+> {
            $($name: PhantomData<$symbol>),+
        }

        impl<$($symbol: Integer),+> Dimensions for $system<$($symbol),+> {}
        impl<$($symbol: $unit),+> Units for $units<$($symbol),+> {}

        #[allow(non_camel_case_types)]
        impl<$($symbol, $unit),+> Sub<$system<$($unit),+>> for $system<$($symbol),+>
            where $($symbol: Integer + Sub<$unit>),+,
                $($unit: Integer),+,
                $(<$symbol as Sub<$unit>>::Output: Integer),+ {
            type Output = $system<$(<$symbol as Sub<$unit>>::Output),+>;

            fn sub(self, _rhs: $system<$($unit),+>) -> Self::Output {
                unreachable!();
            }
        }
    };
}

//#[macro_export]
//macro_rules! quantities {
//    (mods { $($mod_name:ident: $mod_type:ty,)+ }
//         quantities { $($quantity_mod:ident: $quantity:ident,)+ })
//    => {
//        $(pub mod $mod_name {
//            $(pub type $quantity = $quantity<$mod_type>;)+
//        })+
//    }
//}

#[macro_export]
macro_rules! units {
    ($quantity_mod:ident $quantity:ident { $($unit:ident: $conversion:expr;)+ }) => {
        pub use core::marker::{PhantomData};
        pub use core::ops::{Div, Mul};
        pub use self::Units::*;

        pub mod units {
            $(#[allow(non_camel_case_types)] pub struct $unit {})+
        }

        #[allow(non_camel_case_types)]
        pub enum Units {
            $($unit),+
        }

        impl<B: $crate::Units, V: Div<V> + Mul<V>> $quantity<B, V> {
            pub fn new(value: V, unit: Units) -> Self {
                $quantity {
                    value: value,// * (B::conversion() / unit.conversion()),
                    dimensions: PhantomData,
                    units: PhantomData,
                }
            }
        }
    };
}
