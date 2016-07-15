#[macro_export]
macro_rules! system {
    ($system:ident: dimensions {$($name:ident: $symbol:ident, $unit:ident;)*}) => {
        use core::marker::{PhantomData};
        use core::ops::{Sub};
        use typenum::{Integer, Z0, P1};
        use $crate::dimensions::{Dimensions};

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
