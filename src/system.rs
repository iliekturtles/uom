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
        use core::ops::{Div, Index, Mul};

        #[allow(non_camel_case_types)]
        pub enum $subunits {
            $($subunit,)+
        }

        impl<V> Conversion<V, $subunits> for $unit
            where V: Div<f64> + Mul<f64> {
            fn to_base(value: V, subunit: $subunits) -> <V as Mul<f64>>::Output
            {
                value * match subunit {
                    $($subunits::$subunit => $conversion,)+
                }
            }

            fn from_base(value: V, subunit: $subunits) -> <V as Div<f64>>::Output
            {
                value / match subunit {
                    $($subunits::$subunit => $conversion,)+
                }
            }
        }

        //impl<V> Index<$subunits> for $unit
        //    where V: Div<f64> + Mul<f64> {
        //    type Output = <V as Div<f64>>::Output;

        //    fn index(&self, index: $subunits) -> &Self::Output {
        //        &Conversion::from_base(self.value, index)
        //    }
        //}
    };
}
