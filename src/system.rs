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

        impl<$($symbol: Integer),*> $system<$($symbol),*> {
            #[inline(always)]
            pub fn conversionf32($($unit: f32),*) -> f32 {
                use core::num::*;
                1.0 $(* $unit.powi($symbol::to_i32()))*
            }

            #[inline(always)]
            pub fn conversionf64($($unit: f64),*) -> f64 {
                use core::num::*;
                1.0 $(* $unit.powi($symbol::to_i32()))*
            }
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
        pub use self::$subunits::*;

        #[allow(non_camel_case_types)]
        pub enum $subunits {
            $($subunit,)+
        }

        #[macro_export]
        macro_rules! $unit_mod {
            () => {
                pub type $unit = super::$unit_mod::$unit<B, V>;

                impl Conversion<V, super::$unit_mod::$subunits> for $unit
                    where V: Div<V> + Mul<V> {
                    fn to_base(value: V, subunit: super::$unit_mod::$subunits) -> <V as Mul<V>>::Output
                    {
                        value
                            * (super::$unit_mod::Dimensions::conversionf32(L, M, T, I, Th, N, J)
                                * match subunit {
                                    $(super::$unit_mod::$subunits::$subunit => ($conversion),)+
                                })
                    }

                    fn from_base(value: V, subunit: super::$unit_mod::$subunits) -> <V as Div<V>>::Output
                    {
                        value
                            / (super::$unit_mod::Dimensions::conversionf32(L, M, T, I, Th, N, J)
                               * match subunit {
                                    $(super::$unit_mod::$subunits::$subunit => $conversion,)+
                                })
                    }

                    fn new(value: V, subunit: super::$unit_mod::$subunits) -> Self {
                        $unit {
                            value: Self::to_base(value, subunit),
                            dimensions: ::core::marker::PhantomData,
                            base: ::core::marker::PhantomData,
                        }
                    }

                    fn get(self, subunit: super::$unit_mod::$subunits) -> <V as Div<V>>::Output
                    {
                        Self::from_base(self.value, subunit)
                    }
                }
            };
        }
    };
}
