//! Primitive traits and types representing basic properties of types.

/// Trait to denote that a quantity is able to be added with a quantity of the same dimension. When
/// a specific quantity's kind inherits this trait `ops::Add` is implemented automatically.
pub trait Add {}

/// Trait to denote that a quantity is able to be added with a quantity of the same dimension. When
/// a specific quantity's kind inherits this trait `ops::AddAssign` is implemented automatically.
pub trait AddAssign {}

/// Trait to denote that a quantity is able to be subtracted with a quantity of the same dimension.
/// When a specific quantity's kind inherits this trait `ops::Sub` is implemented automatically.
pub trait Sub {}

/// Trait to denote that a quantity is able to be subtracted with a quantity of the same dimension.
/// When a specific quantity's kind inherits this trait `ops::SubAssign` is implemented
/// automatically.
pub trait SubAssign {}

/// Trait to denote that a quantity is able to be multiplied with a quantity of the same dimension.
/// When a specific quantity's kind inherits this trait `ops::Mul` is implemented automatically.
pub trait Mul {}

/// Trait to denote that a quantity is able to be multiplied with a quantity of the same dimension.
/// When a specific quantity's kind inherits this trait `ops::MulAssign` is implemented
/// automatically.
pub trait MulAssign {}

/// Trait to denote that a quantity is able to be divided with a quantity of the same dimension.
/// When a specific quantity's kind inherits this trait `ops::Div` is implemented automatically.
pub trait Div {}

/// Trait to denote that a quantity is able to be divided with a quantity of the same dimension.
/// When a specific quantity's kind inherits this trait `ops::DivAssign` is implemented
/// automatically.
pub trait DivAssign {}

/// Trait to denote that a quantity is able to be negated. When a specific quantity's kind inherits
/// this trait `ops::Neg` is implemented automatically.
pub trait Neg {}

/// Trait to denote that a quantity is able to calculate a remainder with a quantity of the same
/// dimension. When a specific quantity's kind inherits this trait `ops::Rem` is implemented
/// automatically.
pub trait Rem {}

/// Trait to denote that a quantity is able to calculate a remainder with a quantity of the same
/// dimension When a specific quantity's kind inherits this trait `ops::RemAssign` is implemented
/// automatically.
pub trait RemAssign {}

/// Trait to denote that a quantity is able to perform saturating additions and subtractions with a
/// quantity of the same dimension. When a specific quantity's kind inherits this trait
/// `ops::Saturating` is implemented automatically.
pub trait Saturating {}

/// Default [kind][kind] of quantities to allow addition, subtraction, multiplication, division,
/// remainder, negation, and saturating addition/subtraction.
///
/// [kind]: https://jcgm.bipm.org/vim/en/1.2.html
pub trait Kind:
    Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Rem
    + RemAssign
    + Neg
    + Saturating
{
}
