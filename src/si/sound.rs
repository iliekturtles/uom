//! Sound (base unit bel).

quantity! {
    /// Sound (base unit bel).
    quantity: Sound; "sound";
    dimension: ISQ<
        P1, // sound
        Z0, // power
        Z0, // electrical_potential
        >;
    units {
        @bel: prefix!(none); "B", "bel", "bels";
        @decibel: prefix!(none) / prefix!(deci); "dB", "decibel", "decibels";
    }
}
