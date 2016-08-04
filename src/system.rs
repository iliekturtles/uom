#[macro_export]
macro_rules! system {
    ($system:ident: dimensions { $($name:ident: $symbol:ident, $unit:ident;)* }) => {
        use core::marker::{PhantomData};
        use core::ops::{Sub};
        use typenum::{Integer, Z0};
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
    ($unit_mod:ident; $subunits:ident: $unit:ident { $($subunit:ident: $conversion:expr;)+ }) => {
        #[allow(non_camel_case_types)]
        pub enum $subunits {
            $($subunit,)+
        }

        #[macro_export]
        macro_rules! $unit_mod {
            () => {
                pub type $unit = super::$unit_mod::$unit<V>;

                impl Conversion<V, super::$unit_mod::$subunits> for $unit
                    where V: Div<V> + Mul<V> {
                    fn to_base(value: V, subunit: super::$unit_mod::$subunits) -> <V as Mul<V>>::Output
                    {
                        value * match subunit {
                            $(super::$unit_mod::$subunits::$subunit => ($conversion),)+
                        }
                    }

                    fn from_base(value: V, subunit: super::$unit_mod::$subunits) -> <V as Div<V>>::Output
                    {
                        value / match subunit {
                            $(super::$unit_mod::$subunits::$subunit => $conversion,)+
                        }
                    }

                    fn get(self, subunit: super::$unit_mod::$subunits) -> <V as Div<V>>::Output
                    {
                        <$unit as Conversion<V, super::$unit_mod::$subunits>>::from_base(self.value, subunit)
                    }
                }
            };
        }
    };
}
