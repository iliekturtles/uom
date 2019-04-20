//! `uom-macros` provides procedural macro support for `uom`. Two function-style procedural macros
//! are available. [`system!`](macro.system.html), to define a [system of quantities][soq], and
//! [`quantity!`](macro.quantity.html), to define [quantities][q] within a system. See the
//! [`uom`](https://crates.io/crates/uom) crate for full details as well as an implementation of
//! the [International System of Quantities][isq] and [International System of Units][si].
//!
//! See the example below for how to use both macros. A system of quantities `Q`, along with a
//! default [system of units][sou] `U`, is defined using the `system!` macro with base quantities
//! of `length`, `mass`, and `time`. All explicitly defined quantities in the system, both
//! [base][bq] and [derived][dq], are listed. Each quantity is defined using the `quantity!` macro
//! to specify the name, description, [quantity dimension][qd], and associated [measurement
//! units][u]. Measurement units have a conversion factor based on the [base unit][bu] of the
//! default system of units along with an abbreviation and descriptions.
//!
//! [q]: https://jcgm.bipm.org/vim/en/1.1.html
//! [soq]: https://jcgm.bipm.org/vim/en/1.3.html
//! [bq]: https://jcgm.bipm.org/vim/en/1.4.html
//! [dq]: https://jcgm.bipm.org/vim/en/1.5.html
//! [isq]: https://jcgm.bipm.org/vim/en/1.6.html
//! [qd]: https://jcgm.bipm.org/vim/en/1.7.html
//! [u]: https://jcgm.bipm.org/vim/en/1.9.html
//! [bu]: https://jcgm.bipm.org/vim/en/1.10.html
//! [sou]: https://jcgm.bipm.org/vim/en/1.13.html
//! [si]: https://jcgm.bipm.org/vim/en/1.16.html
//!
//! ```rust
//! # fn main() {}
//! #
//! uom_macros::system! {
//!     /// System of quantities, Q.
//!     name: Q;
//!     /// System of units, U.
//!     units: U;
//!     base {
//!         /// Length, denoted by symbol L.
//!         length, L;
//!         /// Mass, denoted by symbol M.
//!         mass, M;
//!         /// Time, denoted by symbol T.
//!         time, T;
//!     }
//!     quantities {
//!         use area::Area;
//!         use length::Length;
//!         use mass::Mass;
//!         use time::Time;
//!     }
//! }
//!
//! mod area {
//!     uom_macros::quantity! {
//!         /// Area (base unit square meter, m²).
//!         quantity: Area;
//!         description: "area";
//!         /// Dimension of area, L² (base unit square meter, m²).
//!         dimension: Q<P2 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
//!         units {
//!             square_meter: 1.0_E0, "m²", "square meter", "square meters";
//!         }
//!     }
//! }
//!
//! mod length {
//!     uom_macros::quantity! {
//!         /// Length (base unit meter, m).
//!         quantity: Length;
//!         description: "length";
//!         /// Dimension of length, L (base unit meter, m).
//!         dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
//!         units {
//!             /// The meter is the SI unit of length. It is defined by taking the fixed numerical
//!             /// value of the speed of light in vacuum *c* to be 299 792 458 when expressed in
//!             /// the unit m s⁻¹, where the second is defined in terms of the caesium frequency
//!             /// ∆*ν*<sub>Cs</sub>.
//!             meter: 1.0_E0, "m", "meter", "meters";
//!             foot: 3.048_E-1, "ft", "foot", "feet";
//!         }
//!     }
//! }
//!
//! mod mass {
//!     uom_macros::quantity! {
//!         /// Mass (base unit kilogram, kg).
//!         quantity: Mass;
//!         description: "mass";
//!         /// Dimension of mass, M (base unit kilogram, kg).
//!         dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
//!         units {
//!             /// The kilogram is the SI unit of mass. It is defined by taking the fixed numerical
//!             /// value of the Planck constant *h* to be 6.626 070 15 × 10⁻³⁴ when expressed in
//!             /// the unit J s, which is equal to kg m² s⁻¹, where the meter and the second are
//!             /// defined in terms of *c* and ∆*ν*<sub>Cs</sub>.
//!             kilogram: 1.0_E0, "kg", "kilogram", "kilograms";
//!             gram: 1.0_E-3, "g", "gram", "grams";
//!         }
//!     }
//! }
//!
//! mod time {
//!     uom_macros::quantity! {
//!         /// Time (base unit second, s).
//!         quantity: Time;
//!         description: "time";
//!         /// Dimension of time, T (base unit second, s).
//!         dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
//!         units {
//!             /// The second is the SI unit of time. It is defined by taking the fixed numerical
//!             /// value of the caesium frequency ∆*ν*<sub>Cs</sub>, the unperturbed ground-state
//!             /// hyperfine transition frequency of the caesium 133 atom, to be 9 192 631 770
//!             /// when expressed in the unit Hz, which is equal to s⁻¹.
//!             second: 1.0_E0, "s", "second", "seconds";
//!         }
//!     }
//! }
//! ```

// Rustc lints.
#![forbid(unsafe_code)]
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
// Clippy lints
#![deny(clippy::pedantic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse::Parse, Error};

mod quantity;
mod system;

/// Define a [system of quantities][soq].
///
/// Using `macro_rules!` style syntax the `system!` macro expects the following parameters.
///
/// * `$name_attributes`: System of quantities attributes. Generally used to set documentation
///   comments.
/// * `$name`: Name of the system of quantities (e.g. `Q` as shown in the example below or `ISQ`
///   for the [International System of Quantities][isq]).
/// * `$units_attributes`: Default [system of units][sou] attributes. Generally used to set
///   documentation comments.
/// * `$units`: Name of the default system of units (e.g. `U` as shown in the example below or `SI`
///   for the [International System of Units][si]).
/// * `$base_attributes`: [Base quantity][bq] attributes. Generally used to set documentation
///   comments.
/// * `$base_name`: Base quantity name (e.g. `length`, `meter`). One or more base quantities must
///   be specified.
/// * `$base_symbol`: [Quantity dimension][qd] symbol (e.g. `L`, `M`).
/// * `mod|use`: `mod` or `use` keyword. When the `mod` keyword is used the `system!` macro will
///   add a `mod` statement for the first segment of the `$quantity` path. When the `use` keyword
///   is used the `system!` macro expects the `$quantity` to already be included in the project.
/// * `$quantity`: Path to the base or [derived][dq] (e.g. `length::Length`). All base quantities
///   must be specified along with any derived quantities that are part of the system of quantities.
///
/// [soq]: https://jcgm.bipm.org/vim/en/1.3.html
/// [bq]: https://jcgm.bipm.org/vim/en/1.4.html
/// [dq]: https://jcgm.bipm.org/vim/en/1.5.html
/// [isq]: https://jcgm.bipm.org/vim/en/1.6.html
/// [qd]: https://jcgm.bipm.org/vim/en/1.7.html
/// [sou]: https://jcgm.bipm.org/vim/en/1.13.html
/// [si]: https://jcgm.bipm.org/vim/en/1.16.html
///
/// ```ignore
/// uom_macros::system! {
///     $($name_attributes:meta)*
///     name: $name:ident;
///     $($units_attributes:meta)*
///     units: $units: ident;
///     base {
///         $(
///             $($base_attributes:meta)*
///             $base_name:ident, $base_symbol:ident;
///         )+
///     }
///     quantities {
///         $(
///             $(mod|use) $quantity:path;
///         )+
///     }
/// }
/// ```
///
/// An example invocation is given below for `Q`, a length-mass-time system of quantities. See the
/// [crate-level](Index.html) documentation for a full example of using the `system!` and
/// `quantity!` macros provided by the `uom-macros` crate. See the
/// [`uom`](https://crates.io/crates/uom) crate for full details as well as an implementation of
/// the [International System of Quantities][isq].
///
/// ```rust
/// # fn main() {}
/// #
/// uom_macros::system! {
///     /// System of quantities, Q.
///     name: Q;
///     /// System of units, U.
///     units: U;
///     base {
///         /// Length, denoted by symbol L.
///         length, L;
///         /// Mass, denoted by symbol M.
///         mass, M;
///         /// Time, denoted by symbol T.
///         time, T;
///     }
///     quantities {
///         use area::Area;
///         use length::Length;
///         use mass::Mass;
///         use time::Time;
///     }
/// }
/// #
/// # mod area {
/// #     uom_macros::quantity! {
/// #         /// Area (base unit square meter, m²).
/// #         quantity: Area;
/// #         description: "area";
/// #         /// Dimension of area, L² (base unit square meter, m²).
/// #         dimension: Q<P2 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
/// #         units {
/// #             square_meter: 1.0_E0, "m²", "square meter", "square meters";
/// #         }
/// #     }
/// # }
/// #
/// # mod length {
/// #     uom_macros::quantity! {
/// #         /// Length (base unit meter, m).
/// #         quantity: Length;
/// #         description: "length";
/// #         /// Dimension of length, L (base unit meter, m).
/// #         dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
/// #         units {
/// #             /// The meter is the SI unit of length. It is defined by taking the fixed
/// #             /// numerical value of the speed of light in vacuum *c* to be 299 792 458 when
/// #             /// expressed in the unit m s⁻¹, where the second is defined in terms of the
/// #             /// caesium frequency ∆*ν*<sub>Cs</sub>.
/// #             meter: 1.0_E0, "m", "meter", "meters";
/// #             foot: 3.048_E-1, "ft", "foot", "feet";
/// #         }
/// #     }
/// # }
/// #
/// # mod mass {
/// #     uom_macros::quantity! {
/// #         /// Mass (base unit kilogram, kg).
/// #         quantity: Mass;
/// #         description: "mass";
/// #         /// Dimension of mass, M (base unit kilogram, kg).
/// #         dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
/// #         units {
/// #             /// The kilogram is the SI unit of mass. It is defined by taking the fixed
/// #             /// numerical value of the Planck constant *h* to be 6.626 070 15 × 10⁻³⁴ when
/// #             /// expressed in the unit J s, which is equal to kg m² s⁻¹, where the meter and
/// #             /// the second are defined in terms of *c* and ∆*ν*<sub>Cs</sub>.
/// #             kilogram: 1.0_E0, "kg", "kilogram", "kilograms";
/// #             gram: 1.0_E-3, "g", "gram", "grams";
/// #         }
/// #     }
/// # }
/// #
/// # mod time {
/// #     uom_macros::quantity! {
/// #         /// Time (base unit second, s).
/// #         quantity: Time;
/// #         description: "time";
/// #         /// Dimension of time, T (base unit second, s).
/// #         dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
/// #         units {
/// #             /// The second is the SI unit of time. It is defined by taking the fixed numerical
/// #             /// value of the caesium frequency ∆*ν*<sub>Cs</sub>, the unperturbed ground-state
/// #             /// hyperfine transition frequency of the caesium 133 atom, to be 9 192 631 770
/// #             /// when expressed in the unit Hz, which is equal to s⁻¹.
/// #             second: 1.0_E0, "s", "second", "seconds";
/// #         }
/// #     }
/// # }
/// ```
#[proc_macro]
pub fn system(input: TokenStream) -> TokenStream {
    expand_proc_macro(input, system::expand)
}

/// Define a [quantity](q) within a [system of quantities][soq].
///
/// Using `macro_rules!` style syntax the `quantity! macros expects the following parameters.
///
/// * `$name_attributes`: Quantity attributes. Generally used to set documentation comments.
/// * `$name`: Quantity name (e.g. `Length`).
/// * `$description`: Quantity description (e.g. `"length"`). The description must be an expression
///   that resolves to a `&'static str`.
/// * `$dimension_attributes`: [Quantity dimension][qd] attributes. Generally used to set
///   documentation comments.
/// * `$system`: System of quantities name used in the `system!` macro invocation (e.g. `Q` as shown
///   in the example below or `ISQ` for the [International System of Quantities][isq]).
/// * `$dimension`: Quantity dimension for each [base quantity][bq] (e.g. `P1`, `Z0`, ...).
///   Quantity dimensions are specified in the same order that the base quantities are defined in
///   the `system!` macro invocation and are `typenum::Integer`s.
/// * `$kind`: Optional [kind of quantity][koq] (e.g. `Temperature`).
/// * `$unit_attributes`: [Measurement unit][u] attributes. Generally used to set documentation
///   comments.
/// * `$unit`: Measurement unit name (e.g. `meter`, `foot`). One or more measurement units must be
///   specified. Although not required, it is recommend that the [base unit][bu] in the default
///   [system of units][sou] is specified for base quantities.
/// * `$coefficient`: Coefficient portion of the [conversion factor][cf]:
///   `default unit = $coefficient * unit + $constant` where the default unit is the unit with a
///   coefficient of `1.0_E0` for the default system of units (e.g. `3.048_E-1` is the coefficient
///   for `foot` where `meter` is the base unit for `Length`). The coefficient must be an
///   expression that resolves to a floating point number.
/// * `$constant`: Constant portion of the conversion factor (e.g. `273.15_E0` is the constant for
///   `celsius` where `kelvin` is the base unit for `Temperature`). The coefficient must be an
///   expression that resolves to a floating point number.
/// * `$abbreviation`: Unit abbreviation (e.g. `"m"`, `"ft"`). The abbreviation must be an
///   expression that resolves to a `&'static str`.
/// * `$singular`: Singular unit description (e.g. `"meter"`, `"foot"`). The singular description
///   must be an expression that resolves to a `&'static str`.
/// * `$plural`: Plural unit description (e.g. `"meters"`, `"feet"`). The plural description must be
///   an expression that resolves to a `&'static str`.
///
/// [q]: https://jcgm.bipm.org/vim/en/1.1.html
/// [koq]: https://jcgm.bipm.org/vim/en/1.2.html
/// [soq]: https://jcgm.bipm.org/vim/en/1.3.html
/// [bq]: https://jcgm.bipm.org/vim/en/1.4.html
/// [dq]: https://jcgm.bipm.org/vim/en/1.5.html
/// [isq]: https://jcgm.bipm.org/vim/en/1.6.html
/// [qd]: https://jcgm.bipm.org/vim/en/1.7.html
/// [mu]: https://jcgm.bipm.org/vim/en/1.9.html
/// [bu]: https://jcgm.bipm.org/vim/en/1.10.html
/// [du]: https://jcgm.bipm.org/vim/en/1.11.html
/// [sou]: https://jcgm.bipm.org/vim/en/1.13.html
/// [cf]: https://jcgm.bipm.org/vim/en/1.24.html
///
/// ```ignore
/// uom_macros::quantity! {
///     $($name_attributes:meta)*
///     quantity: $name:ident;
///     description: $description:expr;
///     $($dimension_attributes:meta)*
///     dimension: $system:ident<$($dimension:ident),+>;
///     $(kind: $kind:ident;)?
///     units {
///         $(
///             $($unit_attributes:meta)*
///             $unit:ident: $coefficient, $($constant:expr,)?
///                 $abbreviation:expr, $singular:expr, $plural:expr;
///         )+
///     }
/// }
/// ```
///
/// An example invocation is given below for `Length` in length-mass-time system of quantities. See
/// the [crate-level](Index.html) documentation for a full example of using the `system!` and
/// `quantity!` macros provided by the `uom-macros` crate. See the
/// [`uom`](https://crates.io/crates/uom) crate for full details as well as an implementation of
/// the [International System of Quantities][isq].
///
/// ```rust
/// # fn main() {}
/// #
/// # uom_macros::system! {
/// #     /// System of quantities, Q.
/// #     name: Q;
/// #     /// System of units, U.
/// #     units: U;
/// #     base {
/// #         /// Length, denoted by symbol L.
/// #         length, L;
/// #         /// Mass, denoted by symbol M.
/// #         mass, M;
/// #         /// Time, denoted by symbol T.
/// #         time, T;
/// #     }
/// #     quantities {
/// #         use area::Area;
/// #         use length::Length;
/// #         use mass::Mass;
/// #         use time::Time;
/// #     }
/// # }
/// #
/// # mod area {
/// #     uom_macros::quantity! {
/// #         /// Area (base unit square meter, m²).
/// #         quantity: Area;
/// #         description: "area";
/// #         /// Dimension of area, L² (base unit square meter, m²).
/// #         dimension: Q<P2 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
/// #         units {
/// #             square_meter: 1.0_E0, "m²", "square meter", "square meters";
/// #         }
/// #     }
/// # }
/// #
/// mod length {
///     uom_macros::quantity! {
///         /// Length (base unit meter, m).
///         quantity: Length;
///         description: "length";
///         /// Dimension of length, L (base unit meter, m).
///         dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
///         units {
///             /// The meter is the SI unit of length. It is defined by taking the fixed numerical
///             /// value of the speed of light in vacuum *c* to be 299 792 458 when expressed in
///             /// the unit m s⁻¹, where the second is defined in terms of the caesium frequency
///             /// ∆*ν*<sub>Cs</sub>.
///             meter: 1.0_E0, "m", "meter", "meters";
///             foot: 3.048_E-1, "ft", "foot", "feet";
///         }
///     }
/// }
/// #
/// # mod mass {
/// #     uom_macros::quantity! {
/// #         /// Mass (base unit kilogram, kg).
/// #         quantity: Mass;
/// #         description: "mass";
/// #         /// Dimension of mass, kg<sup>1</sup>.
/// #         dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
/// #         units {
/// #             /// The kilogram is the SI unit of mass. It is defined by taking the fixed
/// #             /// numerical value of the Planck constant *h* to be 6.626 070 15 × 10⁻³⁴ when
/// #             /// expressed in the unit J s, which is equal to kg m² s⁻¹, where the meter and
/// #             /// the second are defined in terms of *c* and ∆*ν*<sub>Cs</sub>.
/// #             kilogram: 1.0_E0, "kg", "kilogram", "kilograms";
/// #             gram: 1.0_E-3, "g", "gram", "grams";
/// #         }
/// #     }
/// # }
/// #
/// # mod time {
/// #     uom_macros::quantity! {
/// #         /// Time (base unit second, s).
/// #         quantity: Time;
/// #         description: "time";
/// #         /// Dimension of time, T (base unit second, s).
/// #         dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
/// #         units {
/// #             /// The second is the SI unit of time. It is defined by taking the fixed numerical
/// #             /// value of the caesium frequency ∆*ν*<sub>Cs</sub>, the unperturbed ground-state
/// #             /// hyperfine transition frequency of the caesium 133 atom, to be 9 192 631 770
/// #             /// when expressed in the unit Hz, which is equal to s⁻¹.
/// #             second: 1.0_E0, "s", "second", "seconds";
/// #         }
/// #     }
/// # }
/// ```
#[proc_macro]
pub fn quantity(input: TokenStream) -> TokenStream {
    expand_proc_macro(input, quantity::expand)
}

fn expand_proc_macro<T>(
    input: TokenStream,
    _f: fn(T) -> Result<proc_macro2::TokenStream, Error>,
) -> TokenStream
where
    T: Parse,
{
    drop(input);

    TokenStream::new()
}
