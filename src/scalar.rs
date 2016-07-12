use core::default::{Default};
use core::marker::{PhantomData};
use ::{Dimensions};

#[derive(Clone, Copy)]
pub struct Scalar<D: Dimensions, V> {
    value: V,
    dimensions: PhantomData<D>,
}

impl<D: Dimensions, V: Default> Default for Scalar<D, V> {
    fn default() -> Scalar<D, V> {
        Scalar { value: V::default(), dimensions: PhantomData, }
    }
}
