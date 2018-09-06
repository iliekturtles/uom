# Change log
<!-- Template:
## [version] — YYYY-MM-DD

### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security
-->
## [v0.20.0] — 2018-09-06
This release resolves long-standing issue [#3](https://github.com/iliekturtles/uom/issues/3) to
implement thermodynamic temperature conversions (e.g. Celsius to Fahrenheit). Support is also added
for multiple quantities of the same dimension (e.g. thermodynamic temperature and temperature
interval, ratio and angle).

The release also contains a number of internal changes including a reorganization of tests into
multiple files and updated CI setup to provide faster builds and more feature test coverage.

### Added
 * [#3](https://github.com/iliekturtles/uom/issues/3) Implement thermodynamic temperature
   conversions (e.g. Celsius to Fahrenheit). Extend the `quantity!` macro to accept a coefficient
   and optional constant factor in the `$conversion` parameter to support these conversions.
 * [#14](https://github.com/iliekturtles/uom/issues/14) Implement `FromStr`.
 * [#78](https://github.com/iliekturtles/uom/issues/78) Add a `Kind` associated type to
   `Dimensions`.The new `Kind` associated type, defaulting to `uom::Kind`, allows for multiple
   quantities that have the same dimensions. Quantities of different kinds are not comparable. The
   marker traits implemented by a quantity's `Kind` control which operations are automatically
   implemented.
 * [#95](https://github.com/iliekturtles/uom/issues/95) `TemperatureInterval` quantity added.
   Includes `Add`, `AddAssign`, `Sub`, and `SubAssign` implementations between thermodynamic
   temperature and temperature interval.

### Changed
 * [Breaking] Remove unused `_unit: N` parameters and require turbofish syntax for `get`, `floor`,
   `ceil`, `round`, `trunc`, and `fract` methods of `Quantity`. This is a breaking change and can
   easily be resolved. e.g. `l.get(meter)` becomes `l.get::<meter>()`.
 * [#98](https://github.com/iliekturtles/uom/issues/98),
   [#100](https://github.com/iliekturtles/uom/issues/100) Correct `uom` feature hygiene in macro
   generated code. Previously the `system!` macro generated code that included
   `#[cfg(feature = "...")]` attributes on code in the crate executing the `system!` macro. With
   this fix these attributes will be eagerly evaluated and only generate code when the appropriate
   `uom` feature is enabled.
 * Spelling corrections for a number of documentation comments and unit abbreviations.

## [v0.19.0] — 2018-06-21
This release adds a number of additional quantities, configures `uom` to use `rustfmt`, and directly
references `num` sub-crates to better control feature selection.

### Added
 * `Capacitance` quantity added.
 * `ElectricalCharge` quantity added.
 * `ElectricalConductance` quantity added.
 * `ElectricalResistance` quantity added.
 * `Inductance` quantity added.
 * `Luminance` quantity added.
 * `MagneticFluxDensity` quantity added.
 * `MagneticFlux` quantity added.

### Changed
 * [#57](https://github.com/iliekturtles/uom/issues/57) `num` sub-crates `num-traits`,
   `num-rational`, and `num-bigint` are now directly referenced to control feature selection. `std`,
   `rational`, and `bigint` support are only included based on `uom` feature selection.
 * [#80](https://github.com/iliekturtles/uom/issues/80) Setup `rustfmt` so that `uom` code can be
   automatically formatted and builds are gated on changes not breaking formatting conventions.

## [v0.18.0] — 2018-05-10
This release contains a significant number of new quantities and units, compile time improvements,
and a few bug fixes. See below for full details. Many thanks to [Aehmlo](https://github.com/Aehmlo/)
for all of the new quantities and units. [radix](https://github.com/radix) for the `autoconvert`
feature. [Nemo157](https://github.com/Nemo157) for hygiene fixes in the `storage_types!` macro.

### Added
 * [#54](https://github.com/iliekturtles/uom/issues/54) [Breaking] Add an `autoconvert` feature,
   enabled by default, which allows for operations between quantities with different base units.
   `autoconvert` enabled is the same functionality as prior versions of `uom`. Disabling the feature
   only allows for quantities with the same base units to directly interact. The feature has been
   added to account for current compiler limitations where zero-cost code is not generated for
   non-floating point underlying storage types. With the feature disabled more optimal code will be
   generated for integral types. This feature will likely have no effect and subsequently be
   deprecated and removed once `const fn` and specialization are stabilized.
 * [#61](https://github.com/iliekturtles/uom/issues/61) [Breaking] The `storage_types!` macro now
   always uses underlying storage type features from the `uom` crate instead of the crate where the
   macro is executed.
 * [#2](https://github.com/iliekturtles/uom/issues/2) Additional `Mass` units added.
 * [#5](https://github.com/iliekturtles/uom/issues/5) Additional `Velocity` units added.
 * [#6](https://github.com/iliekturtles/uom/issues/6) Additional `Area` units added.
 * [#7](https://github.com/iliekturtles/uom/issues/7) Additional `Volume` units added.
 * [#8](https://github.com/iliekturtles/uom/issues/8) Additional `Force` units added.
 * [#9](https://github.com/iliekturtles/uom/issues/9) Additional `Acceleration` units added.
 * [#30](https://github.com/iliekturtles/uom/issues/30) Additional `Frequency` units added.
 * [#64](https://github.com/iliekturtles/uom/issues/64) `Ratio` (dimensionless) quantity added.
   Includes `From<Ratio> for V` and `From<V> for Ratio` implementations to allow for easy
   conversions between `Ratio` and the underlying storage type.
 * [#66](https://github.com/iliekturtles/uom/issues/66) `Energy` quantity added.
 * [#67](https://github.com/iliekturtles/uom/issues/67) `AvailableEnergy` quantity added.
 * [#68](https://github.com/iliekturtles/uom/issues/68) `Density` quantity added.
 * [#69](https://github.com/iliekturtles/uom/issues/69) `Power` quantity added.
 * [#70](https://github.com/iliekturtles/uom/issues/70) `Pressure` quantity added.
 * [#71](https://github.com/iliekturtles/uom/issues/71) `MassRate` quantity added.
 * [#72](https://github.com/iliekturtles/uom/issues/72) `VolumeRate` quantity added.
 * [#74](https://github.com/iliekturtles/uom/issues/74) `ElectricPotential` quantity added.
 * [#75](https://github.com/iliekturtles/uom/issues/75) Additional `ElectricCurrent` units added.

### Changed
 * [#52](https://github.com/iliekturtles/uom/issues/52) Type aliases generated by the
   `storage_type!` macro are now public. e.g. `pub type V = f32;`. This change drastically reduces
   compile times, especially when multiple underlying storage types are enabled.
 * Dimension documentation for `ElectricCurrent` corrected to no longer references
   `AmountOfSubstance` and units smaller than `milliampere` have corrected abbreviations and
   descriptions.

### Removed
 * [#15](https://github.com/iliekturtles/uom/issues/15) [Breaking] Remove `Product` implementation
   for `Quantity`. The product of a quantity is not that same quantity. e.g.
   `Length * Length = Area`.

## [v0.17.0] — 2018-03-14
This release implements a number of common traits from the standard library, the
[`num`](https://crates.io/crates/num) crate, and the
[`serde`](https://crates.io/crates/serde) crate.

### Added
 * [#16](https://github.com/iliekturtles/uom/issues/16) Implement `Default`.
 * [#17](https://github.com/iliekturtles/uom/issues/17) Implement `PartialEq`.
 * [#50](https://github.com/iliekturtles/uom/issues/50) Implement `Eq`.
 * [#18](https://github.com/iliekturtles/uom/issues/18) Implement `PartialOrd`.
 * [#51](https://github.com/iliekturtles/uom/issues/51) Implement `Ord`.
 * [#56](https://github.com/iliekturtles/uom/issues/56) Implement `Sum`.
 * [#15](https://github.com/iliekturtles/uom/issues/15) Implement `Product`.
 * [#26](https://github.com/iliekturtles/uom/issues/26) Implement `num::Zero`.
 * [#35](https://github.com/iliekturtles/uom/issues/35) Implement `num::Saturating`.
 * [#37](https://github.com/iliekturtles/uom/issues/35) Implement `serde::Serialize` and
   `serde::Deserialize`. Disabled by default. Enabled with the `use_serde` feature.

## [v0.16.0] — 2017-12-21
This release contains significant changes in order to support underlying storage types that
implement the `Num` trait beyond `f32` and `f64`. Many changes are breaking: marker traits are
simplified and fewer macros are exported. New storage types are not enabled by default and can be
used by including the corresponding feature. See the changes below for full details.

### Added
 * Add missing `#[derive(Hash)]` attributes.
 * [#29](https://github.com/iliekturtles/uom/issues/29) A new macro, `storage_types!`, is now
   available to duplicate code on a per-storage type basis. See macro documentation for full
   details. The minimum supported `rustc` version is now 1.20.0.

### Changed
 * [#29](https://github.com/iliekturtles/uom/issues/29) Underlying storage type now uses the `Num`
   trait from the [`num`](https://crates.io/crates/num) crate instead of fixed implementations for
   `f32` and `f64`. Features for all types implementing `Num` have been added and control the
   availability of the type as an underlying storage type: `usize`, `u8`, `u16`, `u32`, `u64`,
   `isize`, `i8`, `i16`, `i32`, `i64`, `bigint`, `biguint`, `rational`, `rational32`, `rational64`,
   `bigrational`, `f32`, and `f64`. For compile time reasons only `f32` and `f64` are enabled by
   default. Tests are implemented for all underlying storage types but don't account for minimum or
   maximum values and will fail for non-float types where the conversion factor overflows the
   type's limits. A future release will correct this.
 * [#29](https://github.com/iliekturtles/uom/issues/29) [Breaking] Traits and structs generated by
   the `system!` macro have been significantly changed in order to support non-float underlying
   storage types.
   * `Dimension` changed to directly include associated types for quantity dimensions.
   * `Units<V>` is now only parameterized on `V` and contains associated types for base units.
   * `Conversion<V>` has been removed and replaced with `uom::Conversion<V>`.
   * `$quantities<...>` has been removed. Quantity dimensions are now directly in the `Dimension`
     trait.
   * `BaseUnits<...>` has been removed. Base units are not directly in the `Units<V>` trait.
   * `uom::Conversion<V>` added to replace `Conversion<V>` and gives access to a unit's conversion
     factor.
   * `uom::ConversionFactor<V>` added to represent conversion factors for underlying storage types
     where the type can't be used (e.g. `i32`'s conversion factor is represented as `Rational32`.)
 * [Breaking] Macro usage and definitions have been simplified and consolidated. `quantities!`,
   `replace_ty!`, and `unit!` have been consolidated as "private" match arms of their calling macro.
   In order to reduce the chance of macro name collisions `$quantities!` is the only remaining
   generated exported macro (e.g. `ISQ!` for the `si` system). Generated macros for each quantity no
   longer exist. These changes make it easier to have multiple systems containing quantities with
   the same names. In order to support this change quantities in the `units` block of the `system!`
   macro must always be prefixed by the quantity's module (e.g. `length::Length`). Prefixing the
   module with the `mod` keyword instructs the `system!` macro not to generate a
   `#[macro_use] pub mod $module;` statement.

## [v0.15.0] — 2017-07-05
This release adds additional `Time` units; `Frequency`, `Force`, and `Volume` quantities; and
numerous floating point methods such as `min`, `max`, and `powi`.

### Added
 * [#4](https://github.com/iliekturtles/uom/issues/4) Additional `Time` units added.
 * Add missing `giga` units. e.g. `gigameter`.
 * [#11](https://github.com/iliekturtles/uom/issues/11) Add floating point classification methods
   `classify`, `is_finite`, `is_infinte`, `is_nan`,  and `is_normal` for `Quantity`.
 * [#11](https://github.com/iliekturtles/uom/issues/11) Add floating point fractional methods
   `floor`, `ceil`, `round`, `trunc`, and `fract` for `Quantity`.
 * [#11](https://github.com/iliekturtles/uom/issues/11) Add floating point comparison methods
   `max` and `min` for `Quantity`.
 * [#11](https://github.com/iliekturtles/uom/issues/11) Add floating point `mul_add` method for
   `Quantity`.
 * [#11](https://github.com/iliekturtles/uom/issues/11) Add floating point `powi` method for
   `Quantity`.
 * [#7](https://github.com/iliekturtles/uom/issues/7) `Volume` quantity added. Additional units
   still need to be added.
 * [#8](https://github.com/iliekturtles/uom/issues/8) `Force` quantity added. Additional units
   still need to be added.
 * [#30](https://github.com/iliekturtles/uom/issues/30) `Frequency` quantity added. Additional units
   still need to be added.

## [v0.14.0] — 2017-05-30

### Added
 * [Breaking] A new feature, `std`, is now available and is enabled by default. `uom` can still be
   compiled with `no_std` by using `--no-default-features` when compiling the crate or
   `default-features = false` in the `dependencies` section of `Cargo.toml`
 * [#11](https://github.com/iliekturtles/uom/issues/11) `cbrt`, `recip`, and `sqrt` are implemented
   for `Quantity`.

### Changed
 * [#28](https://github.com/iliekturtles/uom/issues/28) `Quantity` fields made public in order to
   allow library users to create `const` values and instances of non-named quantities. e.g.
   `const TIME_STEP: Time = Quantity { dimension: PhantomData, units: PhantomData, value: 0.1 };`
   This functionality will be deprecated and subsequently removed once the
   [`const fn`](https://github.com/rust-lang/rust/issues/24111) feature is stabilized.
 * Tests now run for all enabled underlying storage types (`f32`, `f64`).

### Fixed
 * Fixed incorrect conversion factor when multiplying or dividing `Quantity`s with different base
   units.

## [v0.13.0] — 2017-04-20
This release adds numerous tests both for code generated by the `quantity!`, `system!`, and
`$quantities!` macros as well as derived quantities. Fixes for issues found during testing noted
below. Quantity implementations for `Area` and `Acceleration` also added.

### Added
 * [#6](https://github.com/iliekturtles/uom/issues/6) `Area` quantity added. Additional units still
   need to be added.
 * [#9](https://github.com/iliekturtles/uom/issues/9) `Acceleration` quantity added. Additional
   units still need to be added.

### Changed
 * [Breaking] Remove the unused `_unit` parameter from `Quantity::new` and so summon the turbofish.
   e.g. `Length::new(1.0, meter)` becomes `Length::new::<meter>(1.0)`.
 * [#22](https://github.com/iliekturtles/uom/issues/22) Change `impl Debug` for `Quantity` to use
   precision information. Previously the implementation checked the `alternate` flag which has no
   affect on printing raw floats.
 * [#27](https://github.com/iliekturtles/uom/issues/27) Change superscripts in documentation to the
   format supported by `pulldown` in anticipation of upcomming `rustdoc` changes.

### Fixed
 * [#22](https://github.com/iliekturtles/uom/issues/6) Fix `impl Sub` for `Quantity` to be
   implemented in terms of `-` instead of `+`.

## [v0.12.0] — 2017-04-01
Continuous integration was setup to ensure that `uom` builds on stable, beta, nightly, and 1.15.0
(the minimum `rustc` version). Quantity descriptions, unit abbreviations, and unit descriptions
added. Additional length units
[added](https://github.com/iliekturtles/uom/commit/d1b969b810a4b3298d4cf93d0a795d922261735b) to
demonstrate the simplicity of the process.

### Added
 * [#1](https://github.com/iliekturtles/uom/issues/1) Additional `Length` units added.
 * [Breaking] The `quantity!` macro accepts comments on units. To resolve macro parsing ambiguities
   each unit must be prefixed with an `@`.
 * [#19](https://github.com/iliekturtles/uom/issues/19) [Breaking] The `quantity!` macro includes
   the quantity description. The description can be accessed using the `description()` method in
   quantity submodules.
 * [#19](https://github.com/iliekturtles/uom/issues/19) [Breaking] The `quantity!` macro includes
   unit abbreviations as well as singular and plural descriptions. These values can be accessed from
   new `abbreviation()`, `singular()`, and `plural()` methods on the `Unit` trait. The original
   `Unit<V>` trait has been renamed `Conversion<V>`.
 * [#12](https://github.com/iliekturtles/uom/issues/12) `Debug` manually implemented for
   `Quantity<D, U, V>` to show the underlying value and associated units.
 * Test and compile-fail test modules setup. compile-fail tests for mismatched quantities and units
   added.

## v0.11.0 — 2017-02-26
Proof-of-concept functionality for type-safe zero-cost dimensional analysis. `uom` [0.11.0] allows
for the creation of custom systems or the use of the pre-built SI. Basic mathematical operations
are implemented and a minimal set of quantities (length, mass, time...) and units (meter, kilometer,
foot, mile, ...) are included.

[Unreleased]: https://github.com/iliekturtles/uom/compare/v0.20.0...master
[v0.20.0]: https://github.com/iliekturtles/uom/compare/v0.19.0...v0.20.0
[v0.19.0]: https://github.com/iliekturtles/uom/compare/v0.18.0...v0.19.0
[v0.18.0]: https://github.com/iliekturtles/uom/compare/v0.17.0...v0.18.0
[v0.17.0]: https://github.com/iliekturtles/uom/compare/v0.16.0...v0.17.0
[v0.16.0]: https://github.com/iliekturtles/uom/compare/v0.15.0...v0.16.0
[v0.15.0]: https://github.com/iliekturtles/uom/compare/v0.14.0...v0.15.0
[v0.14.0]: https://github.com/iliekturtles/uom/compare/v0.13.0...v0.14.0
[v0.13.0]: https://github.com/iliekturtles/uom/compare/v0.12.0...v0.13.0
[v0.12.0]: https://github.com/iliekturtles/uom/compare/v0.11.0...v0.12.0
