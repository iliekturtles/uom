//! Illuminance (base unit lux, lx, cd · sr / m²).

quantity! {
    /// Illuminance (base unit lux, lx, cd · sr / m²).
    quantity: Illuminance; "illuminance";
    /// Dimension of illuminance, E (base unit lux, lx, cd · sr / m²).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        P1>;    // luminous intensity
    kind: dyn (crate::si::marker::IlluminanceKind);
    units {
        @yottalux: prefix!(yotta); "Ylm", "yottalux", "yottaluxs";
        @zettalux: prefix!(zetta); "Zlm", "zettalux", "zettaluxs";
        @exalux: prefix!(exa); "Elm", "exalux", "exaluxs";
        @petalux: prefix!(peta); "Plm", "petalux", "petaluxs";
        @teralux: prefix!(tera); "Tlm", "teralux", "teraluxs";
        @gigalux: prefix!(giga); "Glm", "gigalux", "gigaluxs";
        @megalux: prefix!(mega); "Mlm", "megalux", "megaluxs";
        @kilolux: prefix!(kilo); "klm", "kilolux", "kiloluxs";
        @hectolux: prefix!(hecto); "hlm", "hectolux", "hectoluxs";
        @decalux: prefix!(deca); "dalm", "decalux", "decaluxs";
        /// The lux is defined to be 1 lumen per square meter,
        /// or 1 candela steradian per square meter.
        @lux: prefix!(none); "lm", "lux", "luxs";
        @decilux: prefix!(deci); "dlm", "decilux", "deciluxs";
        @centilux: prefix!(centi); "clm", "centilux", "centiluxs";
        @millilux: prefix!(milli); "mlm", "millilux", "milliluxs";
        @microlux: prefix!(micro); "µlm", "microlux", "microluxs";
        @nanolux: prefix!(nano); "nlm", "nanolux", "nanoluxs";
        @picolux: prefix!(pico); "plm", "picolux", "picoluxs";
        @femtolux: prefix!(femto); "flm", "femtolux", "femtoluxs";
        @attolux: prefix!(atto); "alm", "attolux", "attoluxs";
        @zeptolux: prefix!(zepto); "zlm", "zeptolux", "zeptoluxs";
        @yoctolux: prefix!(yocto); "ylm", "yoctolux", "yoctoluxs";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::si::quantities::*;
        use crate::si::luminous_flux as lf;
        use crate::si::area as a;
        use crate::si::illuminance as i;
        use crate::tests::{A, Test};

        quickcheck! {
            #[allow(trivial_casts)]
            fn add(l: A<V>, r: A<V>) -> bool {
                Test::eq(&Illuminance::<V>::new::<i::lux>(&*l / &*r),
                         &(LuminousFlux::<V>::new::<lf::lumen>((*l).clone())
                           / Area::<V>::new::<a::square_meter>((*r).clone())).into())
            }
        }
    }
}
