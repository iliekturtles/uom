//! Luminous flux (base unit lumen, lm, cd · sr).

use crate::si::luminous_intensity::LuminousIntensity;
use crate::si::solid_angle::SolidAngle;

quantity! {
    /// Luminous flux (base unit lumen, lm, cd · sr).
    quantity: LuminousFlux; "luminous flux";
    /// Dimension of luminous flux, Φ (base unit lumen, lm, cd · sr).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        P1>;    // luminous intensity
    kind: dyn (crate::si::marker::LuminousFluxKind);
    units {
        @yottalumen: prefix!(yotta); "Ylm", "yottalumen", "yottalumens";
        @zettalumen: prefix!(zetta); "Zlm", "zettalumen", "zettalumens";
        @exalumen: prefix!(exa); "Elm", "exalumen", "exalumens";
        @petalumen: prefix!(peta); "Plm", "petalumen", "petalumens";
        @teralumen: prefix!(tera); "Tlm", "teralumen", "teralumens";
        @gigalumen: prefix!(giga); "Glm", "gigalumen", "gigalumens";
        @megalumen: prefix!(mega); "Mlm", "megalumen", "megalumens";
        @kilolumen: prefix!(kilo); "klm", "kilolumen", "kilolumens";
        @hectolumen: prefix!(hecto); "hlm", "hectolumen", "hectolumens";
        @decalumen: prefix!(deca); "dalm", "decalumen", "decalumens";
        /// The lumen is defined to be 1 candela steradian.
        @lumen: prefix!(none); "lm", "lumen", "lumens";
        @decilumen: prefix!(deci); "dlm", "decilumen", "decilumens";
        @centilumen: prefix!(centi); "clm", "centilumen", "centilumens";
        @millilumen: prefix!(milli); "mlm", "millilumen", "millilumens";
        @microlumen: prefix!(micro); "µlm", "microlumen", "microlumens";
        @nanolumen: prefix!(nano); "nlm", "nanolumen", "nanolumens";
        @picolumen: prefix!(pico); "plm", "picolumen", "picolumens";
        @femtolumen: prefix!(femto); "flm", "femtolumen", "femtolumens";
        @attolumen: prefix!(atto); "alm", "attolumen", "attolumens";
        @zeptolumen: prefix!(zepto); "zlm", "zeptolumen", "zeptolumens";
        @yoctolumen: prefix!(yocto); "ylm", "yoctolumen", "yoctolumens";
    }
}

// TODO: Explicitly allow luminous flux = luminous intensity * solid angle using
// trait implementations such as the below.
#[doc(hidden)]
macro_rules! impl_ops {
    (
        $MulDivTrait:ident, $muldiv_fun:ident, $muldiv_op:tt
    ) => {
        #[cfg(feature = "autoconvert")]
        impl<Ul, Ur, V> $crate::lib::ops::$MulDivTrait<SolidAngle<Ur, V>>
            for LuminousIntensity<Ul, V>
        where
            Ul: super::Units<V> + ?Sized,
            Ur: super::Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            type Output = LuminousFlux<Ul, V>;

            #[inline(always)]
            fn $muldiv_fun(self, rhs: SolidAngle<Ur, V>) -> Self::Output {
                super::Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value
                        $muldiv_op super::change_base::<Dimension, Ul, Ur, V>(&rhs.value),
                }
            }
        }

        #[cfg(not(feature = "autoconvert"))]
        impl<U, V> $crate::lib::ops::$MulDivTrait<SolidAngle<U, V>>
            for LuminousIntensity<U, V>
        where
            U: super::Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            type Output = LuminousFlux<U, V>;

            #[inline(always)]
            fn $muldiv_fun(self, rhs: SolidAngle<U, V>) -> Self::Output {
                super::Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value $muldiv_op rhs.value,
                }
            }
        }
    };
}

//impl_ops!(Mul, mul, *);

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::si::quantities::*;
        use crate::si::luminous_flux as lf;
        use crate::si::luminous_intensity as li;
        use crate::si::solid_angle as sa;
        use crate::tests::{A, Test};

        quickcheck! {
            #[allow(trivial_casts)]
            fn add(l: A<V>, r: A<V>) -> bool {
                Test::eq(&LuminousFlux::<V>::new::<lf::lumen>(&*l * &*r),
                         &(LuminousIntensity::<V>::new::<li::candela>((*l).clone())
                           * SolidAngle::<V>::new::<sa::steradian>((*r).clone())).into())
            }
        }
    }
}
