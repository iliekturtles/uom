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

## [Unreleased]

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

[Unreleased]: https://github.com/iliekturtles/uom/compare/v0.12.0...master
[v0.12.0]: https://github.com/iliekturtles/uom/compare/v0.11.0...v0.12.0
