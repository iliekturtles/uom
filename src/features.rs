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
