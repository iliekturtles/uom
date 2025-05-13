/// Expands the given block of code when `uom` is compiled with the `autoconvert` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "autoconvert")]
macro_rules! autoconvert {
    ($($tt:tt)*) => { $($tt)* };
}

/// Does not expand the given block of code when `uom` is compiled without the `autoconvert`
/// feature.
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "autoconvert"))]
macro_rules! autoconvert {
    ($($tt:tt)*) => {};
}

/// Expands the given block of code when `uom` is compiled with the `autoconvert` and `test`
/// features.
#[doc(hidden)]
#[macro_export]
#[cfg(any(feature = "autoconvert", test))]
macro_rules! autoconvert_test {
    ($($tt:tt)*) => { $($tt)* };
}

/// Does not expand the given block of code when `uom` is compiled without the `autoconvert` or
/// `test` features.
#[doc(hidden)]
#[macro_export]
#[cfg(not(any(feature = "autoconvert", test)))]
macro_rules! autoconvert_test {
    ($($tt:tt)*) => {};
}

/// Expands the given block of code when `uom` is compiled without the `autoconvert` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "autoconvert")]
macro_rules! not_autoconvert {
    ($($tt:tt)*) => {};
}

/// Does not expand the given block of code when `uom` is compiled with the `autoconvert` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "autoconvert"))]
macro_rules! not_autoconvert {
    ($($tt:tt)*) => { $($tt)* };
}

/// Expands to a rust doc test when `uom` is compiled with the `si` and `f32` features and the macro
/// is called from within the `uom` crate.
///
/// Examples:
/// `#[doc = doc_example($crate)]`.
/// `#[doc = doc_example($crate, "compile_fail")]`.
#[doc(hidden)]
#[macro_export]
#[cfg(all(feature = "si", feature = "f32"))]
macro_rules! doc_example {
    (::uom) => {
        " ```rust"
    };
    (::uom, $options:literal) => {
        concat!(" ```rust,", $options)
    };
    ($($tt:tt)*) => {
        " ```rust,ignore"
    };
}

/// Expands to an ignored rust doc test when `uom` is compiled without the `si` or `f32` features
/// regardless of which crate the macro is called from.
#[doc(hidden)]
#[macro_export]
#[cfg(not(all(feature = "si", feature = "f32")))]
macro_rules! doc_example {
    ($($tt:tt)*) => {
        " ```rust,ignore"
    };
}

/// Expands the given block of code when `uom` is compiled with the `serde` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "serde")]
macro_rules! serde {
    ($($tt:tt)*) => { $($tt)* };
}

/// Does not expand the given block of code when `uom` is compiled without the `serde` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "serde"))]
macro_rules! serde {
    ($($tt:tt)*) => {};
}

/// Expands the given block of code when `uom` is compiled with the `si` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "si")]
macro_rules! si {
    ($($tt:tt)*) => { $($tt)* };
}

/// Does not expand the given block of code when `uom` is compiled without the `si` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "si"))]
macro_rules! si {
    ($($tt:tt)*) => {};
}

/// Expands the given block of code when `uom` is compiled with the `std` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(feature = "std")]
macro_rules! std {
    ($($tt:tt)*) => { $($tt)* };
}

/// Does not expand the given block of code when `uom` is compiled without the `std` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! std {
    ($($tt:tt)*) => {};
}

/// Expands the given block of code when `uom` is compiled with the `test` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(test)]
macro_rules! test {
    ($($tt:tt)*) => { $($tt)* };
}

/// Does not expand the given block of code when `uom` is compiled without the `test` feature.
#[doc(hidden)]
#[macro_export]
#[cfg(not(test))]
macro_rules! test {
    ($($tt:tt)*) => {};
}
