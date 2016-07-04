use core::fmt;
use core::marker::{PhantomData};
use core::ops::{Div, Sub};

use typenum::{Integer, Z0};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dimensions<
    L: Integer = Z0,    // length
    M: Integer = Z0,    // mass
    T: Integer = Z0,    // time
    I: Integer = Z0,    // electric current
    Th: Integer = Z0,   // thermodynamic temperature
    N: Integer = Z0,    // amount of substance
    J: Integer = Z0     // luminous intensity
> {
    length: PhantomData<L>,
    mass: PhantomData<M>,
    time: PhantomData<T>,
    electric_curent: PhantomData<I>,
    thermodynamic_temperature: PhantomData<Th>,
    amount_of_substance: PhantomData<N>,
    luminous_intensity: PhantomData<J>,
}

impl<L, M, T, I, Th, N, J> Dimensions<L, M, T, I, Th, N, J>
    where L: Integer, M: Integer, T: Integer, I: Integer, Th: Integer, N: Integer, J: Integer {
    pub fn new() -> Dimensions<L, M, T, I, Th, N, J> {
        Dimensions {
            length: PhantomData,
            mass: PhantomData,
            time: PhantomData,
            electric_curent: PhantomData,
            thermodynamic_temperature: PhantomData,
            amount_of_substance: PhantomData,
            luminous_intensity: PhantomData,
        }
    }
}

impl<L, M, T, I, Th, N, J> fmt::Debug for Dimensions<L, M, T, I, Th, N, J>
    where L: Integer, M: Integer, T: Integer, I: Integer, Th: Integer, N: Integer, J: Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dimensions<{}, {}, {}, {}, {}, {}, {}>",
            L::to_i32(), M::to_i32(), T::to_i32(), I::to_i32(), Th::to_i32(), N::to_i32(),
            J::to_i32())
    }
}

impl<Ll, Ml, Tl, Il, Thl, Nl, Jl, Lr, Mr, Tr, Ir, Thr, Nr, Jr>
        Div<Dimensions<Lr, Mr, Tr, Ir, Thr, Nr, Jr>>
        for Dimensions<Ll, Ml, Tl, Il, Thl, Nl, Jl>
    where Ll: Integer + Sub<Lr>,
        Ml: Integer + Sub<Mr>,
        Tl: Integer + Sub<Tr>,
        Il: Integer + Sub<Ir>,
        Thl: Integer + Sub<Thr>,
        Nl: Integer + Sub<Nr>,
        Jl: Integer + Sub<Jr>,
        <Ll as Sub<Lr>>::Output: Integer,
        <Ml as Sub<Mr>>::Output: Integer,
        <Tl as Sub<Tr>>::Output: Integer,
        <Il as Sub<Ir>>::Output: Integer,
        <Thl as Sub<Thr>>::Output: Integer,
        <Nl as Sub<Nr>>::Output: Integer,
        <Jl as Sub<Jr>>::Output: Integer,
        Lr: Integer,
        Mr: Integer,
        Tr: Integer,
        Ir: Integer,
        Thr: Integer,
        Nr: Integer,
        Jr: Integer {
    type Output = Dimensions<
        <Ll as Sub<Lr>>::Output,
        <Ml as Sub<Mr>>::Output,
        <Tl as Sub<Tr>>::Output,
        <Il as Sub<Ir>>::Output,
        <Thl as Sub<Thr>>::Output,
        <Nl as Sub<Nr>>::Output,
        <Jl as Sub<Jr>>::Output>;

    fn div(self, _rhs: Dimensions<Lr, Mr, Tr, Ir, Thr, Nr, Jr>) -> Self::Output {
        //unreachable!();
        Self::Output::new()
    }
}
