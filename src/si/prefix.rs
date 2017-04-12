/// Macro to implement the [SI][si] prefixes for [multiples of units][mult] and
/// [submultiples of units][submult].
///
/// Implemented using `macro_rules!` instead of `const` so that type inference at call sites can
/// generate the appropriate float type. Using explicit constants would require duplicate
/// definitions for `f32` and `f64` or casting from `f64` in `f32` contexts.
///
/// [si]: http://jcgm.bipm.org/vim/en/1.16.html
/// [mult]: http://jcgm.bipm.org/vim/en/1.17.html
/// [submult]: http://jcgm.bipm.org/vim/en/1.18.html
#[macro_export]
macro_rules! prefix {
    (yotta) => { 1.0E24 };
    (zetta) => { 1.0E21 };
    (exa) => { 1.0E18 };
    (peta) => { 1.0E15 };
    (tera) => { 1.0E12 };
    (giga) => { 1.0E9 };
    (mega) => { 1.0E6 };
    (kilo) => { 1.0E3 };
    (hecto) => { 1.0E2 };
    (deca) => { 1.0E1 };
    (none) => { 1.0E0 };
    (deci) => { 1.0E-1 };
    (centi) => { 1.0E-2 };
    (milli) => { 1.0E-3 };
    (micro) => { 1.0E-6 };
    (nano) => { 1.0E-9 };
    (pico) => { 1.0E-12 };
    (femto) => { 1.0E-15 };
    (atto) => { 1.0E-18 };
    (zepto) => { 1.0E-21 };
    (yocto) => { 1.0E-24 };
}
