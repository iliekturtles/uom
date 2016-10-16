#[macro_export]
macro_rules! system {
    ($system:ident; $units:ident: dimensions { $($name:ident: $symbol:ident;)+ }) => {
        use core::marker::{PhantomData};
        use core::ops::{Sub};
        use typenum::{Integer, Z0};
        use $crate::{Dimensions, Units};

        #[derive(Clone, Copy, Debug)]
        pub struct $system<$($symbol: Integer = Z0),+> {
            $($name: PhantomData<$symbol>),+
        }

        #[derive(Clone, Copy, Debug)]
        pub struct $units<$($symbol: $name::Unit),+> {
            $($name: PhantomData<$symbol>),+
        }

        impl<$($symbol: Integer),+> Dimensions for $system<$($symbol),+> {}
        impl<$($symbol: $name::Unit),+> Units for $units<$($symbol),+> {}

        #[allow(non_camel_case_types)]
        impl<$($symbol, $name),+> Sub<$system<$($name),+>> for $system<$($symbol),+>
            where $($symbol: Integer + Sub<$name>),+,
                $($name: Integer),+,
                $(<$symbol as Sub<$name>>::Output: Integer),+ {
            type Output = $system<$(<$symbol as Sub<$name>>::Output),+>;

            fn sub(self, _rhs: $system<$($name),+>) -> Self::Output {
                unreachable!();
            }
        }

        #[macro_export]
        macro_rules! base_units {
            () => {
                #[allow(non_camel_case_types)]
                impl<$($name,)+ $($symbol),+> $crate::UnitsConversion<super::$system<$($name),+>, V> for super::$units<$($symbol),+>
                    where $($name: ::typenum::Integer,)+
                        $($symbol: super::$name::Unit + $crate::UnitConversion<V>),+ {
                    #[inline(always)]
                    fn conversion() -> V {
                        use core::num::*;

                        1.0 $(* <$symbol as $crate::UnitConversion<V>>::conversion()
                            .powi($name::to_i32()))+
                    }
                }
            };
        }
    };
}

#[macro_export]
macro_rules! units {
    ($quantity_mod:ident::$quantity:ident { $($unit:ident: $conversion:expr;)+ }) => {
        pub use self::Units::*;

        pub trait Unit: $crate::Unit {}

        pub mod units {
            $(#[allow(non_camel_case_types)] #[derive(Clone, Copy, Debug)] pub struct $unit {}
            impl super::Unit for $unit {}
            impl $crate::Unit for $unit {})+
        }

        #[allow(non_camel_case_types)]
        pub enum Units {
            $($unit),+
        }

        #[macro_export]
        macro_rules! $quantity_mod {
            () => {
                pub type $quantity = super::$quantity_mod::$quantity<U, V>;

                impl $quantity {
                    #[inline(always)]
                    pub fn new(value: V, unit: super::$quantity_mod::Units) -> Self {
                        $quantity {
                            value: value
                                * (::core::convert::Into::<V>::into(unit)
                                    / <U as $crate::UnitsConversion<super::$quantity_mod::Dimension, V>>::conversion()),
                            dimensions: ::core::marker::PhantomData,
                            units: ::core::marker::PhantomData,
                        }
                    }

                    #[inline(always)]
                    pub fn get(self, unit: super::$quantity_mod::Units) -> V {
                        self.value
                            / (<U as $crate::UnitsConversion<super::$quantity_mod::Dimension, V>>::conversion()
                                * ::core::convert::Into::<V>::into(unit))
                    }
                }
            };
            ($T:ty) => {
                $quantity_mod!();

                $(impl $crate::UnitConversion<$T> for super::$quantity_mod::units::$unit {
                    #[inline(always)]
                    fn conversion() -> $T {
                        $conversion
                    }
                })+

                impl ::core::convert::Into<$T> for super::$quantity_mod::Units {
                    #[inline(always)]
                    fn into(self) -> $T {
                        match self {
                            $(super::$quantity_mod::Units::$unit => $conversion,)+
                        }
                    }
                }
            };
        }
    };
}
