use super::dimension::Dimension;

pub trait Scalar {
    fn name() -> &'static str;
    fn dimensions(&self) -> &Dimension;
}
