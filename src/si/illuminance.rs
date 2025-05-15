//! Illuminance (base unit lux, cd · sr · m⁻²).

quantity! {
    /// Illuminance (base unit lux, cd · sr · m⁻²).
    quantity: Illuminance; "illuminance";
    /// Dimension of illuminance, L⁻²J (base unit lux, cd·sr·m⁻²)
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
        @yottalux: prefix!(yotta); "Ylx", "yottalux", "yottalux";
        @zettalux: prefix!(zetta); "Zlx", "zettalux", "zettalux";
        @exalux: prefix!(exa); "Elx", "exalux", "exalux";
        @petalux: prefix!(peta); "Plx", "petalux", "petalux";
        @teralux: prefix!(tera); "Tlx", "teralux", "teralux";
        @gigalux: prefix!(giga); "Glx", "gigalux", "gigalux";
        @megalux: prefix!(mega); "Mlx", "megalux", "megalux";
        @kilolux: prefix!(kilo); "klx", "kilolux", "kilolux";
        @hectolux: prefix!(hecto); "hlx", "hectolux", "hectolux";
        @decalux: prefix!(deca); "dalx", "decalux", "decalux";
        /// Derived unit of illuminance.
        @lux: prefix!(none); "lx", "lux", "lux";
        @decilux: prefix!(deci); "dlx", "decilux", "decilux";
        @centilux: prefix!(centi); "clx", "centilux", "centilux";
        @millilux: prefix!(milli); "mlx", "millilux", "millilux";
        @microlux: prefix!(micro); "µlx", "microlux", "microlux";
        @nanolux: prefix!(nano); "nlx", "nanolux", "nanolux";
        @picolux: prefix!(pico); "plx", "picolux", "picolux";
        @femtolux: prefix!(femto); "flx", "femtolux", "femtolux";
        @attolux: prefix!(atto); "alx", "attolux", "attolux";
        @zeptolux: prefix!(zepto); "zlx", "zeptolux", "zeptolux";
        @yoctolux: prefix!(yocto); "ylx", "yoctolux", "yoctolux";

        @phot: 1.0_E4; "ph", "phot", "phot";
        @footcandle: 1.076_391_E1; "fc", "footcandle", "footcandles";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::area as a;
        use crate::si::illuminance as il;
        use crate::si::luminous_intensity as li;
        use crate::si::quantities::*;
        use crate::si::solid_angle as sa;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: Illuminance<V> = (LuminousIntensity::new::<li::candela>(V::one())
                * SolidAngle::new::<sa::steradian>(V::one())
                / Area::new::<a::square_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<li::yottacandela, sa::steradian, a::square_meter, il::yottalux>();
            test::<li::zettacandela, sa::steradian, a::square_meter, il::zettalux>();
            test::<li::exacandela, sa::steradian, a::square_meter, il::exalux>();
            test::<li::petacandela, sa::steradian, a::square_meter, il::petalux>();
            test::<li::teracandela, sa::steradian, a::square_meter, il::teralux>();
            test::<li::gigacandela, sa::steradian, a::square_meter, il::gigalux>();
            test::<li::megacandela, sa::steradian, a::square_meter, il::megalux>();
            test::<li::kilocandela, sa::steradian, a::square_meter, il::kilolux>();
            test::<li::hectocandela, sa::steradian, a::square_meter, il::hectolux>();
            test::<li::decacandela, sa::steradian, a::square_meter, il::decalux>();
            test::<li::candela, sa::steradian, a::square_meter, il::lux>();
            test::<li::decicandela, sa::steradian, a::square_meter, il::decilux>();
            test::<li::centicandela, sa::steradian, a::square_meter, il::centilux>();
            test::<li::millicandela, sa::steradian, a::square_meter, il::millilux>();
            test::<li::microcandela, sa::steradian, a::square_meter, il::microlux>();
            test::<li::nanocandela, sa::steradian, a::square_meter, il::nanolux>();
            test::<li::picocandela, sa::steradian, a::square_meter, il::picolux>();
            test::<li::femtocandela, sa::steradian, a::square_meter, il::femtolux>();
            test::<li::attocandela, sa::steradian, a::square_meter, il::attolux>();
            test::<li::zeptocandela, sa::steradian, a::square_meter, il::zeptolux>();
            test::<li::yoctocandela, sa::steradian, a::square_meter, il::yoctolux>();

            test::<li::yottacandela, sa::steradian, a::square_kilometer, il::exalux>();
            test::<li::zettacandela, sa::steradian, a::square_kilometer, il::petalux>();
            test::<li::exacandela, sa::steradian, a::square_kilometer, il::teralux>();
            test::<li::petacandela, sa::steradian, a::square_kilometer, il::gigalux>();
            test::<li::teracandela, sa::steradian, a::square_kilometer, il::megalux>();
            test::<li::gigacandela, sa::steradian, a::square_kilometer, il::kilolux>();
            test::<li::megacandela, sa::steradian, a::square_kilometer, il::lux>();
            test::<li::kilocandela, sa::steradian, a::square_kilometer, il::millilux>();
            test::<li::candela, sa::steradian, a::square_kilometer, il::microlux>();
            test::<li::millicandela, sa::steradian, a::square_kilometer, il::nanolux>();
            test::<li::microcandela, sa::steradian, a::square_kilometer, il::picolux>();
            test::<li::nanocandela, sa::steradian, a::square_kilometer, il::femtolux>();
            test::<li::picocandela, sa::steradian, a::square_kilometer, il::attolux>();
            test::<li::femtocandela, sa::steradian, a::square_kilometer, il::zeptolux>();
            test::<li::attocandela, sa::steradian, a::square_kilometer, il::yoctolux>();

            test::<li::candela, sa::steradian, a::square_centimeter, il::phot>();
            // f64 calculated result 10.763910416709722 doesn't match sig. figures of square_foot.
            // test::<li::candela, sa::steradian, a::square_foot, il::footcandle>();

            fn test<
                LI: li::Conversion<V>,
                SA: sa::Conversion<V>,
                A: a::Conversion<V>,
                IL: il::Conversion<V>>()
            {
                Test::assert_approx_eq(&Illuminance::new::<IL>(V::one()),
                    &(LuminousIntensity::new::<LI>(V::one())
                        * SolidAngle::new::<SA>(V::one())
                        / Area::new::<A>(V::one())).into());
            }
        }
    }
}
