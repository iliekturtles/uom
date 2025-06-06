/// Use this in place of #[cfg(test)] in macro output. This will ensure that the code is acatually
/// only outputted when running this crate in test mode
#[cfg(test)]
#[macro_export]
#[doc(hidden)]
macro_rules! _test_only {
    ($($tt:tt)*) => {
            $($tt)*
    }
}

/// Use this in place of #[cfg(test)] in macro output. This will ensure that the code is acatually
/// only outputted when running this crate in test mode
#[cfg(not(test))]
#[macro_export]
#[doc(hidden)]
macro_rules! _test_only {
    ($($tt:tt)*) => {};
}
