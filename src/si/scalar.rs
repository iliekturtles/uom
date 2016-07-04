use super::dimensions::Dimensions;

pub trait Scalar {
    fn name() -> &'static str;
    fn dimensions(&self) -> &Dimensions;
}
