//! Electric permittivity (base unit farad per meter, m⁻³ · kg⁻¹ · s⁴ · A²).

quantity! {
    /// Electric permittivity (base unit farad per meter, m⁻³ · kg⁻¹ · s⁴ · A²).
    quantity: ElectricPermittivity; "electric permittivity";
    /// Dimension of electric permittivity, L⁻³M⁻¹T⁴I² (base unit farad per meter,
    /// m⁻³ · kg⁻¹ · s⁴ · A²).
    dimension: ISQ<
        N3,     // length
        N1,     // mass
        P4,     // time
        P2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @farad_per_meter: prefix!(none); "F/m", "farad per meter", "farads per meter";
        @vacuum_electric_permittivity: 8.854_187_812_8_E-12; "ε₀", "vacuum electric permittivity",
            "vacuum electric permittivity";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::capacitance as c;
        use crate::si::electric_permittivity as ep;
        use crate::si::length as l;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricPermittivity<V> = Capacitance::new::<c::farad>(V::one())
                / Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<ep::farad_per_meter, c::farad, l::meter>();

            fn test<EP: ep::Conversion<V>, C: c::Conversion<V>, L: l::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricPermittivity::new::<EP>(V::one()),
                    &(Capacitance::new::<C>(V::one())
                        / Length::new::<L>(V::one())));
            }
        }
    }
}
