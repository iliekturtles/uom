#[macro_export]
macro_rules! system {
    ($system:ident: dimensions { $($name:ident: $symbol:ident, $unit:ident;)* }) => {
        use core::marker::{PhantomData};
        use core::ops::{Sub};
        use typenum::{Integer, Z0, P1};
        use $crate::{Dimensions};

        #[derive(Clone, Copy, Debug)]
        pub struct $system<$($symbol: Integer = Z0),*> {
            $($name: PhantomData<$symbol>),*
        }

        impl<$($symbol: Integer),*> Dimensions for $system<$($symbol),*> {}

        #[allow(non_camel_case_types)]
        impl<$($symbol, $unit),*> Sub<$system<$($unit),*>> for $system<$($symbol),*>
            where $($symbol: Integer + Sub<$unit>),*,
                $($unit: Integer),*,
                $(<$symbol as Sub<$unit>>::Output: Integer),* {
            type Output = $system<$(<$symbol as Sub<$unit>>::Output),*>;

            fn sub(self, _rhs: $system<$($unit),*>) -> Self::Output {
                unreachable!();
            }
        }
    };
}

#[macro_export]
macro_rules! subunits {
    ($subunits:ident: $unit:ty { $($subunit:ident: $conversion:expr;)+ }) => {
        use $crate::{Conversion};
        use core::ops::{Div, Mul};

        pub enum $subunits {
            $($subunit,)+
        }

        impl<V, S> Conversion<V, S> for $unit
            where V: Div<V> + Mul<V> {
            fn to_base(value: V, subunit: S) -> <V as Mul<V>>::Output
            {
                value /* match subunit {
                    $($subunit => $conversion,)+
                }*/
            }

            fn from_base(value: V, subunit: S) -> <V as Div<V>>::Output
            {
                value /* match subunit {
                    $($subunit => $conversion,)+
                }*/
            }
        }
    };
}
