# Change log
<!-- Template:
## [version] — YYYY-MM-DD

### Security
### Added
### Changed
### Deprecated
### Removed
### Fixed
-->
## [v0.37.0] — 2025-05-14
This release adds new quantities, new units, updates `uom` to 2021 edition, and fixes bitrot exposed
by the latest tools.

Many thanks to [aevyrie](https://github.com/aevyrie),
[boondocklabs](https://github.com/boondocklabs), [caspermeijn](https://github.com/caspermeijn),
[cutlerjake](https://github.com/cutlerjake), [elsandosgrande](https://github.com/elsandosgrande),
[mkdjr](https://github.com/mkdjr), [Netzwerk2](https://github.com/Netzwerk2),
[nsunderland1](https://github.com/nsunderland1), and [portyanikhin](https://github.com/portyanikhin)
for pull requests included and issues resolved in this release.

### Added
 * [#479](https://github.com/iliekturtles/uom/pull/479) Add `AngularMomentum` quantity.
 * [#489](https://github.com/iliekturtles/uom/pull/489) Add `ton_long_per_cubic_foot` and
   `ton_short_per_cubic_foot` units to `MassDensity`.
 * [#489](https://github.com/iliekturtles/uom/pull/489) Add numerous units to `SpecificVolume`.
 * [#502](https://github.com/iliekturtles/uom/pull/502) Add `inch_per_minute` unit to `Velocity`.
 * [#507](https://github.com/iliekturtles/uom/pull/507) Add `SurfaceTension` quantity.
 * [#508](https://github.com/iliekturtles/uom/pull/508) Add `KinematicViscosity` quantity.
 * Add `gram_force` unit to `Force`.

### Changed
 * [#477](https://github.com/iliekturtles/uom/pull/477) Replace relative URLs in documents with Rust
   item links.
 * [#516](https://github.com/iliekturtles/uom/pull/516) Rust 1.86.0 is now used for `rustfmt`,
   `clippy`, and `tarpaulin` jobs.
 * [#514](https://github.com/iliekturtles/uom/pull/514) Update to 2021 edition.
 * Automatically check unit validity by replacing explicit `TypeId` checks with
   `Conversion::is_valid` calls.

### Fixed
 * [#462](https://github.com/iliekturtles/uom/pull/462) Fix abbreviations for units in
   `MassPerEnergy`.
 * [#473](https://github.com/iliekturtles/uom/pull/473) Fix abbreviations for units in
   `InverseVelocity`.
 * [#478](https://github.com/iliekturtles/uom/pull/478) Fix NIST Special Publication 811 links.
 * [#500](https://github.com/iliekturtles/uom/pull/500) Resolve `unexpected_cfg` warnings caused by
   `cfg` references in macro-generated code.
 * Correct `cargo-clippy`/`clippy` feature usage for `--check-cfg`.
 * Resolve `clippy` empty docs warnings.

## [v0.36.0] — 2024-03-20
This release adds new quantities, new units, bumps the MSRV (minimum supported Rust version) to
1.65.0, and fixes bitrot in Github actions.

Many thanks to [Aehmlo](https://github.com/Aehmlo), [Code-Maniac](https://github.com/Code-Maniac),
[baarkerlounger](https://github.com/baarkerlounger), [g1aeder](https://github.com/g1aeder),
[hellow554](https://github.com/hellow554), [igiona](https://github.com/igiona),
[waywardmonkeys](https://github.com/waywardmonkeys), and
[yacinelakel](https://github.com/yacinelakel) for pull requests included and issues resolved in this
release.

### Added
 * [#429](https://github.com/iliekturtles/uom/pull/429) Add `minute_per_kilometer` unit to
   `InverseVelocity`.
 * [#436](https://github.com/iliekturtles/uom/pull/436) Add explicit `serde` feature. The new
   `serde` feature deprecates the old `use_serde` feature which is now an alias for `serde` and will
   be removed in a future `uom` release.
 * [#446](https://github.com/iliekturtles/uom/pull/446) Add `ArealHeatCapacity` quantity.
 * [#450](https://github.com/iliekturtles/uom/pull/450) Add `ThermalResistance` quantity.

### Changed
 * [#425](https://github.com/iliekturtles/uom/pull/425) Clarify `Conversion` documentation for
   converting to and from the base unit.
 * [#432](https://github.com/iliekturtles/uom/pull/432) Use `Cargo.toml` `rust-version` key to
   identify the MSRV (minimum supported Rust version).
 * [#445](https://github.com/iliekturtles/uom/pull/445) Update basic example with example code to do
   unit conversions.
 * [#456](https://github.com/iliekturtles/uom/pull/456) Commit `Cargo.lock` to pin certain crates to
   specific versions that support `uom`'s MSRV (minimum supported Rust version).
 * [#459](https://github.com/iliekturtles/uom/pull/459) Increase MSRV (minimum supported Rust
   version) to `1.65.0`. No changes in this release require the new MSRV.
 * [#457](https://github.com/iliekturtles/uom/pull/457) Update github `checkout` and `cache` actions
   to `v4`.
 * [#459](https://github.com/iliekturtles/uom/pull/459) Rust 1.76.0 is now used for `rustfmt`,
   `clippy`, and `tarpaulin` jobs.
 * [#443](https://github.com/iliekturtles/uom/pull/443),
   [#459](https://github.com/iliekturtles/uom/pull/459) Resolve numerous `rustc` and `clippy`
   warnings.

## [v0.35.0] — 2023-07-10
This release adds new quantities, bumps the MSRV (minimum supported Rust version) to 1.60.0, and
fixes bitrot in Github actions.

Many thanks to [calbaker](https://github.com/calbaker), [groscoe2](https://github.com/groscoe2),
[professoralex13](https://github.com/professoralex13), [robinohs](https://github.com/robinohs), and
[Uzaaft](https://github.com/Uzaaft) for pull requests included and issues resolved in this release.

### Added
 * [#406](https://github.com/iliekturtles/uom/pull/406) Add `cubic_meter_per_minute` and
   `cubic_meter_per_hour` units to `VolumeRate`.
 * [#409](https://github.com/iliekturtles/uom/pull/409) Add `AngularAbsement` quantity.
 * [#410](https://github.com/iliekturtles/uom/pull/410) Add `MassPerEnergy` quantity.
 * [#417](https://github.com/iliekturtles/uom/pull/417) [Breaking] Correct `ThermalConductance`
   `meter_per` units to be `meter_squared_per`. Descriptions and abbreviations were already correct.
 * [#419](https://github.com/iliekturtles/uom/pull/419) Add `PowerRate` quantity.
 * [#420](https://github.com/iliekturtles/uom/pull/420) Add `InverseVelocity` quantity.

### Changed
 * [#314](https://github.com/iliekturtles/uom/issues/314) Update Github actions to use
   `dtolnay/rust-toolchain`. `actions-rs` is no longer maintained.
 * [#387](https://github.com/iliekturtles/uom/issues/387) Increase MSRV (minimum supported Rust
   version) to `1.60.0`. No changes in this release require the new MSRV.

### Fixed
 * Fix unnecessary qualifications warnings in quantity tests.

## [v0.34.0] — 2022-10-26
This release adds a significant number of new quantities and units as well as a few minor fixes.

Many thanks to [adamreichold](https://github.com/adamreichold),
[calbaker](https://github.com/calbaker), [Eagle941](https://github.com/Eagle941),
[jossriLDR](https://github.com/jossriLDR), [swaits](https://github.com/swaits),
[Uzaaft](https://github.com/Uzaaft) [werdahias](https://github.com/werdahias), and
[zdimension](https://github.com/zdimension) for pull requests included and issues resolved in this
release. Special thanks to [crystal-growth](https://github.com/crystal-growth) for the *thirty nine*
pull requests included in this release.

### Added
 * [#318](https://github.com/iliekturtles/uom/pull/318) Add `particle` units to `AmountOfSubstance`,
   `CatalyticActivity`, `CatalyticActivityConcentration`, `MolarConcentration`, `MolarEnergy`, and
   `MolarHeatCapacity` quantities.
 * [#319](https://github.com/iliekturtles/uom/pull/319) Add `MolarFlux` quantity.
 * [#322](https://github.com/iliekturtles/uom/pull/322) Add `DiffusionCoefficient` quantity.
 * [#323](https://github.com/iliekturtles/uom/pull/323) Add `DynamicViscosity` quantity.
 * [#324](https://github.com/iliekturtles/uom/pull/324) Add atomic units of energy, mass, length,
   electric charge, and current.
 * [#325](https://github.com/iliekturtles/uom/pull/325) Add `Action` quantity.
 * [#326](https://github.com/iliekturtles/uom/pull/326) Add `ElectricField` quantity.
 * [#327](https://github.com/iliekturtles/uom/pull/327) Add `ElectricDipoleMoment` and
   `ElectricQuadrupoleMoment` quantities.
 * [#329](https://github.com/iliekturtles/uom/pull/329) Add `ElectricalMobility` quantity.
 * [#330](https://github.com/iliekturtles/uom/pull/330) Add `ElectricalResistivity` quantity.
 * [#331](https://github.com/iliekturtles/uom/pull/331) Add `ElectricalConductivity` quantity.
 * [#332](https://github.com/iliekturtles/uom/pull/332) Add `ElectricPermittivity` quantity.
 * [#333](https://github.com/iliekturtles/uom/pull/333) Add `ElectricCurrentDensity` quantity.
 * [#334](https://github.com/iliekturtles/uom/pull/334) Add `ArealNumberDensity`,
   `LinearNumberDensity`, and `VolumetricNumberDensity` quantities.
 * [#335](https://github.com/iliekturtles/uom/pull/335) Add `ReciprocalLength` quantity.
 * [#336](https://github.com/iliekturtles/uom/pull/336) Add `MassFlux` quantity.
 * [#337](https://github.com/iliekturtles/uom/pull/337) Add `MagneticPermeability` quantity.
 * [#339](https://github.com/iliekturtles/uom/pull/339) Add `ArealNumberRate`, `LinearNumberRate`,
   and `VolumetricNumberRate` quantities.
 * [#341](https://github.com/iliekturtles/uom/pull/341) Add `ArealMassDensity`, `LinearMassDensity`,
   and `VolumetricMassDensity` quantities.
 * [#342](https://github.com/iliekturtles/uom/pull/342) Add `SurfaceElectricCurrentDensity`
   quantity.
 * [#343](https://github.com/iliekturtles/uom/pull/343) Add `MagneticMoment` quantity.
 * [#344](https://github.com/iliekturtles/uom/pull/344) Add `ElectricChargeArealDensity`,
   `ElectricChargeLinearDensity`, and `ElectricChargeVolumetricDensity` quantities.
 * [#345](https://github.com/iliekturtles/uom/pull/345) Add `ElectricDisplacementField` quantity.
 * [#346](https://github.com/iliekturtles/uom/pull/346) Add `MagneticFieldStrength` quantity.
 * [#347](https://github.com/iliekturtles/uom/pull/347) Add `MolarVolume` quantity.
 * [#348](https://github.com/iliekturtles/uom/pull/348) Add `ElectricFlux` quantity.
 * [#349](https://github.com/iliekturtles/uom/pull/349) Add `TemperatureGradient` quantity.
 * [#350](https://github.com/iliekturtles/uom/pull/350) Add `LinearPowerDensity` and
   `VolumetricPowerDensity` quantities as well as additional `HeatFluxDensity` units.
 * [#351](https://github.com/iliekturtles/uom/pull/351) Add `standard_gravity` unit to
   `Acceleration`.
 * [#352](https://github.com/iliekturtles/uom/pull/352) Add `SpecificArea` and `SpecificVolume`
   quantities.
 * [#353](https://github.com/iliekturtles/uom/pull/353) Add `boltzmann_constant` unit to
   `HeatCapacity` and `molar_gas_constant` unit to `MolarHeatCapacity`.
 * [#354](https://github.com/iliekturtles/uom/pull/354) Add `TemperatureCoefficient` quantity.
 * [#355](https://github.com/iliekturtles/uom/pull/355) Add `ArealDensityOfStates`,
   `LinearDensityOfStates`, and `VolumetricDensityOfStates` quantities.
 * [#356](https://github.com/iliekturtles/uom/pull/356) Add `MomentOfInertia` quantity.
 * [#358](https://github.com/iliekturtles/uom/pull/358) Add molar flow rate units to
   `AmountOfSubstance` and `CatalyticActivity`.
 * [#361](https://github.com/iliekturtles/uom/pull/361) Add `atomic_unit_of_velocity`,
   `natural_unit_of_velocity` and `speed_of_light_in_vacuum` units to `Velocity`.
 * [#368](https://github.com/iliekturtles/uom/pull/368) Add quantities for `Radioactivity`,
   `MolarRadioactivity`, and `SpecificRadioactivity` as well as additional `VolumetricNumberRate`
   units.
 * [#369](https://github.com/iliekturtles/uom/pull/369) Add `VolumetricHeatCapacity` quantity as
   well as additional `HeatCapacity` and `SpecificHeatCapacity` units.
 * [#370](https://github.com/iliekturtles/uom/pull/370) Add `Molality` quantity.
 * [#388](https://github.com/iliekturtles/uom/pull/388) Add `FrequencyDrift` quantity.
 * [#396](https://github.com/iliekturtles/uom/pull/396) Add `SpecificPower` quantity.
 * [#399](https://github.com/iliekturtles/uom/pull/399) Add `liter per minute` unit to `VolumeRate`
   and `newton per square millimeter` unit to `Pressure`.
 * [#405](https://github.com/iliekturtles/uom/pull/405) Add `ton_per_minute`, `ton_per_hour`, and
   `ton_per_day` units to `MassRate`.
 * [#408](https://github.com/iliekturtles/uom/pull/408) Add `ThermalConductance` quantity.

### Fixed
 * [#383](https://github.com/iliekturtles/uom/issues/383) Fix typos in past release notes.
 * [#392](https://github.com/iliekturtles/uom/issues/392) Disable certain `Area` and `Volume` tests
   on ARM CPUs until floating point behavior issues can be resolved.
 * [#393](https://github.com/iliekturtles/uom/pull/393) Fix typo in the spelling of `millijoule`.
 * Correct volume rate unit tests.

## [v0.33.0] — 2022-06-28
This release adds one new quantity, `Absement`. Two new underlying storage types, `Complex32` and
`Complex64`. Eight new exponential and logarithmic functions on `Ratio` and changes to use
`#[must_use]` and `#[non_exhaustive]`.

Many thanks to [adamreichold](https://github.com/adamreichold),
[gonzaponte](https://github.com/gonzaponte), [jacg](https://github.com/jacg),
[nick-pascucci-spire](https://github.com/nick-pascucci-spire), and
[TobTobXX](https://github.com/TobTobXX) for pull requests included and issues resolved in this
release.

### Added
 * [#284](https://github.com/iliekturtles/uom/pull/284),
   [#285](https://github.com/iliekturtles/uom/pull/285) `Absement` quantity added.
 * [#287](https://github.com/iliekturtles/uom/pull/287) Add support for `Complex32` and `Complex64`
   as underlying storage types.
 * [#290](https://github.com/iliekturtles/uom/pull/290) Implement `exp2`, `exp_m1`, `exp`, `ln_1p`,
   `ln`, `log10`, `log2`, and `log` for `Ratio`.
 * [#306](https://github.com/iliekturtles/uom/pull/306) Add missing `#[must_use]` on all methods
   returning a value. `must_use_candidate` and `return_self_not_must_use` clippy lints are now
   enabled to ensure future methods include the attribute.

### Changed
 * [#272](https://github.com/iliekturtles/uom/pull/272) Improve documentation on how to enable
  `serde` for `big*` and `rational*` underlying storage types.
 * Enable `#[non_exhaustive]` on `Units` `enum`s. The `#[doc(hidden)] __nonexhaustive` trick is not
   longer used.

## [v0.32.0] — 2022-01-14
This release adds one new quantity, `MolarHeatCapacity`, a new trait, `ConstZero`, and many internal
improvements. The `quickcheck` 1.0 update uncovered a number of issues with floating point precision
that were able to be resolved while still maintaining zero-cost guarantees.

Many thanks to [adamreichold](https://github.com/adamreichold), [B-Reif](https://github.com/B-Reif),
[remilauzier](https://github.com/remilauzier), and [T-Bakker](https://github.com/T-Bakker) for pull
requests included and issues resolved in this release.

### Added
 * [#250](https://github.com/iliekturtles/uom/pull/250) Add `ConstZero` trait which is implemented
   by `Quantity`.
 * [#263](https://github.com/iliekturtles/uom/pull/263) `MolarHeatCapacity` quantity added.

### Changed
 * [#258](https://github.com/iliekturtles/uom/pull/258) Use `RUSTFLAGS="-D warnings"` in CI to
   ensure that `rustc` warnings and caught and fail their respective workflows.
 * [Breaking] Rename `Conversion::into_conversion` to `Conversion::conversion`. Name change resolves
   `Clippy` `wrong_self_convention` warnings.
 * [#260](https://github.com/iliekturtles/uom/pull/260) Increase minimum supported `rustc` version
   to 1.43.0. Required to support `quickcheck` 1.0.
 * [#260](https://github.com/iliekturtles/uom/pull/260) Update to `quickcheck` 1.0. Change required
   the MSRV update as well as refactoring `from_base`, `to_base`, and multiple tests. The
   `from_base` and `to_base` changes provide better floating point precision while still maintaining
   zero-cost guarantees. The test changes better handle floating point precision issues as well as
   the wider range of values generated by `quickcheck` 1.0's `Arbitrary` implementation.
 * [#268](https://github.com/iliekturtles/uom/pull/268) Clarify `powi` documentation.

### Deprecated
 * [#260](https://github.com/iliekturtles/uom/pull/260) Deprecate `try-from` feature. The feature
   will be removed in a future release of `uom`. Functionality previously exposed by the feature is
   now enabled by default.

### Fixed
 * [#252](https://github.com/iliekturtles/uom/pull/252) Fix links within the documentation to use
   https. Many previously used http and were broken.
 * [#260](https://github.com/iliekturtles/uom/pull/260) Fix `TryFrom<Duration> for Time<U, V>`.
   Previously the conversion used the `Duration`'s subsecond microseconds as nanoseconds. The
   conversion now correct uses the subsecond nanoseconds.

## [v0.31.1] — 2021-03-01
This release corrects documentation issues and documents fewer underlying
storage types on docs.rs so that container time and memory limits are not
exceeded.

### Added
 * [#241](https://github.com/iliekturtles/uom/issues/241) Run `rustdoc` as part
   of tool checks and resolve previously ignored warnings.

### Changed
 * [#241](https://github.com/iliekturtles/uom/issues/241) Document fewer
   underlying storage types on docs.rs so that container time and memory limits
   are not exceeded. Previous releases documented all features and caused
   documentation to fail to build.

## [v0.31.0] — 2021-01-05
This release adds a new macro, `unit!`, to allow for units to be defined outside of the
`quantity!` macro as well as a `Units` enum for each quantity. A number of standard library
traits are implemented. Build regressions caused by issues with the CI system and changes in
Rust are now fixed.

Many thanks to [bheisler](https://github.com/bheisler),
[CreepySkeleton](https://github.com/CreepySkeleton),
[DusterTheFirst](https://github.com/DusterTheFirst), [Lucretiel](https://github.com/Lucretiel), and
[neoeinstein](https://github.com/neoeinstein) for pull requests included and issues resolved in
this release.

### Added
 * [#173](https://github.com/iliekturtles/uom/issues/173) Allow new units to be defined using
   `unit!` outside of `quantity!`. When using the pre-built SI system included with `uom` this
   macro allows for new units to quickly be defined without requiring a release.
   [Pull requests](https://github.com/iliekturtles/uom/pulls) to add new units upstream area
   always greatly appreciated.
 * [#215](https://github.com/iliekturtles/uom/pull/215) Add `Units` enum and
   `fn units() -> impl Iterator<Item = Units>` function for each quantity.
 * [#227](https://github.com/iliekturtles/uom/issues/227) Ensure `UnwindSafe` and `RefUnwindSafe`
   are implemented.
 * [#217](https://github.com/iliekturtles/uom/pull/217) Add trait implementations for `Display` and
   `Error` to `ParseQuantityError`.

### Changed
 * [#214](https://github.com/iliekturtles/uom/pull/214) The `FromStr` implementation for quantities
   now supports the unit singular and plural descriptions in addition to the unit abbreviation.
 * [#225](https://github.com/iliekturtles/uom/pull/225) Convert CI to use Github Actions. This
   change fixes a number of problems with the old system and greatly improves build times.
 * [#223](https://github.com/iliekturtles/uom/issues/223) Correct build regressions introduced
   while no test job was run with the old CI system.

## [v0.30.0] — 2020-10-17
This release adds a new quantity, `RadiantExposure`, implements `Unpin` for `Quantity` and
upgrades `uom` to the 2018 edition. These changes also include an increase of the minimum
supported `rustc` to 1.37.0. Many thanks to [nicodemus26](https://github.com/nicodemus26) and
(Michael-F-Bryan)[https://github.com/Michael-F-Bryan] for pull requests included and issues
resolved in this release.

### Added
 * [#202](https://github.com/iliekturtles/uom/pull/202) `RadiantExposure` quantity added.
 * [#204](https://github.com/iliekturtles/uom/issues/204) Implement `Unpin` for `Quantity`.

### Changed
 * [#206](https://github.com/iliekturtles/uom/pull/206) Upgrade `uom` to compile using the 2018
   edition. Generated code still supports both the 2015 and 2018 edition.
 * [#207](https://github.com/iliekturtles/uom/pull/207) Increase minimum supported `rustc` version
   to 1.37.0. Required to support upgrading to the latest dependencies and to allow for the Kleene
   `?` "at most one" repetition operator.

## [v0.29.0] — 2020-08-06
This release includes a number of changes for `Angle`, the addition of `SolidAngle`, and a new
units for `Energy` and `Luminance`. Many thanks to
[adamreichold](https://github.com/adamreichold), [AnickaBurova](https://github.com/AnickaBurova),
and [Atmelfan](https://github.com/Atmelfan) for pull requests included and issues resolved in
this release.

### Added
 * [#196](https://github.com/iliekturtles/uom/pull/196) `SolidAngle` quantity added.
 * [#191](https://github.com/iliekturtles/uom/pull/191) Add constants to `Angle` and `SolidAngle`
   representing half and full turns.
 * [#194](https://github.com/iliekturtles/uom/pull/194) Add `foot-candle` unit to `Luminance`.
 * [#200](https://github.com/iliekturtles/uom/pull/200) `electronvolt`-based `Energy` units added.

### Changed
 * [#192](https://github.com/iliekturtles/uom/issues/192) Use `f{32|64}::powi` instead of
   `Typenum::Pow::powi`.
 * [#123](https://github.com/iliekturtles/uom/issues/123) Change examples to use `Display` instead
   of `Debug`.

### Removed
 * [#188](https://github.com/iliekturtles/uom/issues/188) [Breaking] Remove `From`/`Into` impls for
   `Angle` and `SolidAngle`.

## [v0.28.0] — 2020-05-21
This release includes a number of trigonometric improvements as well as new units for `Time`.
Many thanks to [adamreichold](https://github.com/adamreichold) and
[Aehmlo](https://github.com/Aehmlo) for pull requests included and issues resolved in this
release.

### Added
 * [#182](https://github.com/iliekturtles/uom/pull/182) Add inverse trigonometric functions to
   `Ratio` (`acos`, `acosh`, `asin`, `asinh`, `atan`, `atanh`) and `Angle` (`atan2`).
 * [#184](https://github.com/iliekturtles/uom/pull/184) Tropical and sidereal units added to `Time`.

### Changed
 * [#186](https://github.com/iliekturtles/uom/pull/186) Make `hypot` available for all quantities,
   not just `Length`.
 * [#187](https://github.com/iliekturtles/uom/pull/187) [Breaking] Change trigonometric functions to
   return `Ratio` instead of the underlying storage type so that identities like
   `x.sin().asin() == x` are well-typed.

## [v0.27.0] — 2020-02-14
This release adds a number of quantities.

### Added
 * [#167](https://github.com/iliekturtles/uom/pull/167) `CatalyticActivity`,
   `CatalyticActivityConcentration`, `MassConcentration`, and `MolarConcentration` quantities added.
 * [#175](https://github.com/iliekturtles/uom/pull/175) `Curvature` quantity added.

## [v0.26.0] — 2019-11-04
This release adds a number of thermodynamic quantities in addition to `TryFrom` implementations for
`Time` and trigonometric functions on `Angle`.

### Added
 * [#164](https://github.com/iliekturtles/uom/pull/164) `HeatFluxDensity` quantity added.
 * [#155](https://github.com/iliekturtles/uom/pull/155) `HeatCapacity`, `HeatTransfer`,
   `MolarEnergy`, `MolarMass`, `SpecificHeatCapacity`, `ThermalConductivity` quantities added.
 * [#150](https://github.com/iliekturtles/uom/pull/150) `TryFrom` implemented between
   `std::time::Duration` and `uom::si::Time`.
 * [#151](https://github.com/iliekturtles/uom/pull/151) Trigonometric functions added to `Angle`.

### Changed
 * Increase minimum supported `rustc` version to 1.31.0. Required to because of backwards
   incompatibilities with `Cargo.toml` `edition` keyword. `cfg-if` introducted the keyword in a
   minor version update and other issues with the keyword prompted the update.

## [v0.25.0] — 2019-08-12
This release includes the long-requested `Information` and `InformationRate` quantities as well as
support for `i128`/`u128` as underlying storage types. A number of documentation updates, including
for the 9th edition of the SI, are also included.

### Added
 * [#31](https://github.com/iliekturtles/uom/issues/31) `Information` (`bit`, `byte`, ...) and
   `InformationRate` (`bit/s`, `byte/s`, ...) quantities added. Units for both SI (`kilo`, `mega`,
   ...) and binary (`kibi`, `mebi`, ...) prefixes are included.
 * [#85](https://github.com/iliekturtles/uom/issues/85) Add support for `i128` and `u128` as
   underlying storage types.
 * [#160](https://github.com/iliekturtles/uom/pull/160) `AngularJerk` quantity added.

### Changed
 * [#20](https://github.com/iliekturtles/uom/issues/20) Document generic parameters.
 * [#21](https://github.com/iliekturtles/uom/issues/21) Move links inline for trait methods to work
   around documentation generation bug.
 * [#111](https://github.com/iliekturtles/uom/issues/111) Update documentation for SI base units for
   the 9th edition of the SI.
 * [#127](https://github.com/iliekturtles/uom/issues/127) Remove superscripts for power 1 to keep
   all SI documentation consistent.

## [v0.24.0] — 2019-06-20
This release fixes two separate issues to ensure that zero-cost code is generated. Many thanks to
[raimundomartins](https://github.com/raimundomartins), [apopiak](https://github.com/apopiak), and
[gnzlbg](https://github.com/gnzlbg) for pull requests included and issues resolved in this release.

### Added
 * [#145](https://github.com/iliekturtles/uom/issues/145) Add `#[repr(transparent)]` to `Quantity`
   to ensure that the ABI of the underlying storage type is used instead of struct ABI in FFI
   contexts.

### Changed
 * [#148](https://github.com/iliekturtles/uom/pull/148) Increase minimum supported `rustc` version
   to 1.28.0. Required to support `#[repr(transparent)]`.

### Fixed
 * [#143](https://github.com/iliekturtles/uom/issues/143) Correct `to_base` and `from_base` to be
   zero-cost for float storage types.
 * [#147](https://github.com/iliekturtles/uom/pull/147) Correct typos in README and crate-level
   documentation.

## [v0.23.1] — 2019-05-29
This release fixes an issue with the `autoconvert` feature introduced in [v0.23.0]. Many thanks to
[dmit](https://github.com/dmit) for pull requests included in this release.

### Fixed
 * [#141](https://github.com/iliekturtles/uom/issues/141) `From` implementations to convert between
   quantities of different kinds now correctly use the `autoconvert` feature and no longer cause a
   compile error when the feature is disabled.

## [v0.23.0] — 2019-05-13
This release adds three new quantities, `AngularAcceleration`, `AngularVelocity`, and `Torque`.
Changes to make `Kind`s more ergonomic to use are also included along with documentation changes.
Many thanks to [dunmatt](https://github.com/dunmatt/) and [Aehmlo](https://github.com/Aehmlo) for
pull requests included in this release.

### Added
 * [#136](https://github.com/iliekturtles/uom/pull/136) `AngularAcceleration` quantity added.
 * [#135](https://github.com/iliekturtles/uom/pull/135) `AngularVelocity` quantity added.
 * [#117](https://github.com/iliekturtles/uom/issues/117) `Torque` quantity added.
 * Introduce `AngleKind` and `si::marker` to hold SI specific marker traits. `From` implementations
   for `AngleKind` added to more easily convert between `uom::Kind` and `si::marker::AngleKind`.
 * Allow documentation to be specified for base quantities in the `system!` macro. Documentation for
   the seven base SI quantities added.

### Changed
 * [#138](https://github.com/iliekturtles/uom/issues/138) Maintain kind when multiplying a number by
   a quantity. Multiplying a quantity by a number already maintains kind.
 * [#130](https://github.com/iliekturtles/uom/pull/130) Rename `Density` to `MassDensity`. A type
   alias for `Density` is available for backwards compatibility.
 * [#127](https://github.com/iliekturtles/uom/issues/127) Make all SI quantity documentation follow
   a single, consistent format.

### Deprecated
 * [#130](https://github.com/iliekturtles/uom/pull/130) `Density` has been renamed to `MassDensity`
   and is deprecated. `Density` will be removed in some future release.

## [v0.22.2] — 2019-04-28
This release adds `Angle` and `Jerk` quantities along with unit additions and conversion precision
improvements to `Acceleration` and `Velocity`. Many thanks to [dunmatt](https://github.com/dunmatt/)
and [nicodemus26](https://github.com/nicodemus26/) respectively.

### Added
 * [#89](https://github.com/iliekturtles/uom/issues/89) `Angle` quantity added.
 * [#128](https://github.com/iliekturtles/uom/issues/128) `Jerk` quantity added. `Acceleration` and
   `Velocity` units added and precision for some existing units improved.

### Changed
 * Continuous integration setup updated and improved.
   * Rust 1.34.0 is now used for `rustfmt`, `clippy`, and `tarpaulin` jobs.
   * `rustfmt` configuration updated.
   * `clippy` configuration corrected to run for all packages.

## [v0.22.1] — 2019-04-02
This release adds additional `liter`-based `VolumeRate` units.

### Added
 * [#121](https://github.com/iliekturtles/uom/pull/121) `Liter`-based `VolumeRate` units added.

## [v0.22.0] — 2019-03-30
This release adds the `Momentum` quantity and additional `liter`-based `volume` units.

### Added
 * [#114](https://github.com/iliekturtles/uom/issues/114) `Momentum` quantity added.
 * [#116](https://github.com/iliekturtles/uom/pull/116) `Liter`-based `volume` units added.

### Changed
 * Continuous integration setup updated and improved.
   * Rust 1.33.0 is now used for `rustfmt`, `clippy`, and `tarpaulin` jobs.
   * The deny warnings job is joined into the `clippy` job.
   * The stable + tests job has been updated to catch errors with non-default underlying storage
     types in SI quantity tests.

### Fixed
 * [#119](https://github.com/iliekturtles/uom/issues/119) Macros corrected to generate valid code
   for both 2015 and 2018 editions. Previously the doc test on the `$quantities!` macro would fail
   in a Rust 2018 crate. A new test crate, `edition_check`, was added to ensure `uom` remains usable
   in Rust 2018 code.

## [v0.21.1] — 2019-03-03
This release adds a few new units for `ElectricCharge` and `Energy`.

### Added
 * [#112](https://github.com/iliekturtles/uom/pull/112) Additional `ElectricCharge` and `Energy`
   units added.

## [v0.21.0] — 2019-01-13
This release adds display tools for quantities resolving another long-standing issue,
[#13](https://github.com/iliekturtles/uom/issues/13).

### Added
 * [#13](https://github.com/iliekturtles/uom/issues/13) Add display tools for quantities. Allows
   `Quantity` to be formatted with the standard library formatting traits: `Binary`, `Debug`,
   `Display`, `LowerExp`, `LowerHex`, `Octal`, `UpperExp`, `UpperHex`.

### Changed
 * Improve continuous integration setup.
   * Update `clippy` and `rustfmt` jobs to use the now stable non `-preview` components.
   * Run OSX and Windows builds on TravisCI.
   * Improve individual job build time by reducing unnecessary cached data.

## [v0.20.1] — 2018-09-13
This release resolves nightly rustc error E0659 the base.rs example caused by
[rust-lang/rust](https://github.com/rust-lang/rust) pull request
[#52841](https://github.com/rust-lang/rust/pull/52841) "resolve: Implement prelude search for macro
paths, implement tool attributes."

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
 * `ElectricCharge` quantity added.
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
 * [#37](https://github.com/iliekturtles/uom/issues/37) Implement `serde::Serialize` and
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
 * [#22](https://github.com/iliekturtles/uom/issues/22) Fix `impl Sub` for `Quantity` to be
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

[Unreleased]: https://github.com/iliekturtles/uom/compare/v0.37.0...master
[v0.37.0]: https://github.com/iliekturtles/uom/compare/v0.36.0...v0.37.0
[v0.36.0]: https://github.com/iliekturtles/uom/compare/v0.35.0...v0.36.0
[v0.35.0]: https://github.com/iliekturtles/uom/compare/v0.34.0...v0.35.0
[v0.34.0]: https://github.com/iliekturtles/uom/compare/v0.33.0...v0.34.0
[v0.33.0]: https://github.com/iliekturtles/uom/compare/v0.32.0...v0.33.0
[v0.32.0]: https://github.com/iliekturtles/uom/compare/v0.31.1...v0.32.0
[v0.31.1]: https://github.com/iliekturtles/uom/compare/v0.31.0...v0.31.1
[v0.31.0]: https://github.com/iliekturtles/uom/compare/v0.30.0...v0.31.0
[v0.30.0]: https://github.com/iliekturtles/uom/compare/v0.29.0...v0.30.0
[v0.29.0]: https://github.com/iliekturtles/uom/compare/v0.28.0...v0.29.0
[v0.28.0]: https://github.com/iliekturtles/uom/compare/v0.27.0...v0.28.0
[v0.27.0]: https://github.com/iliekturtles/uom/compare/v0.26.0...v0.27.0
[v0.26.0]: https://github.com/iliekturtles/uom/compare/v0.25.0...v0.26.0
[v0.25.0]: https://github.com/iliekturtles/uom/compare/v0.24.0...v0.25.0
[v0.24.0]: https://github.com/iliekturtles/uom/compare/v0.23.1...v0.24.0
[v0.23.1]: https://github.com/iliekturtles/uom/compare/v0.23.0...v0.23.1
[v0.23.0]: https://github.com/iliekturtles/uom/compare/v0.22.2...v0.23.0
[v0.22.2]: https://github.com/iliekturtles/uom/compare/v0.22.1...v0.22.2
[v0.22.1]: https://github.com/iliekturtles/uom/compare/v0.22.0...v0.22.1
[v0.22.0]: https://github.com/iliekturtles/uom/compare/v0.21.1...v0.22.0
[v0.21.1]: https://github.com/iliekturtles/uom/compare/v0.21.0...v0.21.1
[v0.21.0]: https://github.com/iliekturtles/uom/compare/v0.20.1...v0.21.0
[v0.20.1]: https://github.com/iliekturtles/uom/compare/v0.20.0...v0.20.1
[v0.20.0]: https://github.com/iliekturtles/uom/compare/v0.19.0...v0.20.0
[v0.19.0]: https://github.com/iliekturtles/uom/compare/v0.18.0...v0.19.0
[v0.18.0]: https://github.com/iliekturtles/uom/compare/v0.17.0...v0.18.0
[v0.17.0]: https://github.com/iliekturtles/uom/compare/v0.16.0...v0.17.0
[v0.16.0]: https://github.com/iliekturtles/uom/compare/v0.15.0...v0.16.0
[v0.15.0]: https://github.com/iliekturtles/uom/compare/v0.14.0...v0.15.0
[v0.14.0]: https://github.com/iliekturtles/uom/compare/v0.13.0...v0.14.0
[v0.13.0]: https://github.com/iliekturtles/uom/compare/v0.12.0...v0.13.0
[v0.12.0]: https://github.com/iliekturtles/uom/compare/v0.11.0...v0.12.0
