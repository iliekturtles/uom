#[macro_export]
#[cfg(feature = "autoconvert")]
macro_rules! autoconvert {
    ($($tt:tt)*) => { $($tt)* };
}

#[macro_export]
#[cfg(not(feature = "autoconvert"))]
macro_rules! autoconvert {
    ($($tt:tt)*) => {};
}

#[macro_export]
#[cfg(any(feature = "autoconvert", test))]
macro_rules! autoconvert_test {
    ($($tt:tt)*) => { $($tt)* };
}

#[macro_export]
#[cfg(not(any(feature = "autoconvert", test)))]
macro_rules! autoconvert_test {
    ($($tt:tt)*) => {};
}

#[macro_export]
#[cfg(feature = "autoconvert")]
macro_rules! not_autoconvert {
    ($($tt:tt)*) => {};
}

#[macro_export]
#[cfg(not(feature = "autoconvert"))]
macro_rules! not_autoconvert {
    ($($tt:tt)*) => { $($tt)* };
}

#[macro_export]
#[cfg(feature = "serde")]
macro_rules! serde {
    ($($tt:tt)*) => { $($tt)* };
}

#[macro_export]
#[cfg(not(feature = "serde"))]
macro_rules! serde {
    ($($tt:tt)*) => {};
}

#[macro_export]
#[cfg(feature = "si")]
macro_rules! si {
    ($($tt:tt)*) => { $($tt)* };
}

#[macro_export]
#[cfg(not(feature = "si"))]
macro_rules! si {
    ($($tt:tt)*) => {};
}

#[macro_export]
#[cfg(feature = "std")]
macro_rules! std {
    ($($tt:tt)*) => { $($tt)* };
}

#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! std {
    ($($tt:tt)*) => {};
}

#[macro_export]
#[cfg(test)]
macro_rules! test {
    ($($tt:tt)*) => { $($tt)* };
}

#[macro_export]
#[cfg(not(test))]
macro_rules! test {
    ($($tt:tt)*) => {};
}
