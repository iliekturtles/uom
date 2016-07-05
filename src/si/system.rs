use core::default::{Default};
use core::fmt;
use core::marker::{PhantomData};
use core::ops::{Div, Sub};

use typenum::{Integer, Z0};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct System<
    V,                  // Value type
    L: Integer = Z0,    // length
    M: Integer = Z0,    // mass
    T: Integer = Z0,    // time
    I: Integer = Z0,    // electric current
    Th: Integer = Z0,   // thermodynamic temperature
    N: Integer = Z0,    // amount of substance
    J: Integer = Z0     // luminous intensity
> {
    value: V,
    length: PhantomData<L>,
    mass: PhantomData<M>,
    time: PhantomData<T>,
    electric_curent: PhantomData<I>,
    thermodynamic_temperature: PhantomData<Th>,
    amount_of_substance: PhantomData<N>,
    luminous_intensity: PhantomData<J>,
}

impl<V, L, M, T, I, Th, N, J> Default for System<V, L, M, T, I, Th, N, J>
    where V: Default,
        L: Integer, M: Integer, T: Integer, I: Integer, Th: Integer, N: Integer, J: Integer {
    fn default() -> System<V, L, M, T, I, Th, N, J> {
        System {
            value: V::default(),
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

impl<V, L, M, T, I, Th, N, J> fmt::Debug for System<V, L, M, T, I, Th, N, J>
    where V: fmt::Display,
        L: Integer, M: Integer, T: Integer, I: Integer, Th: Integer, N: Integer, J: Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}, {}, {}, {}, {}, {}, {}>",
            self.value, L::to_i32(), M::to_i32(), T::to_i32(), I::to_i32(), Th::to_i32(),
            N::to_i32(), J::to_i32())
    }
}

impl<V, Ll, Ml, Tl, Il, Thl, Nl, Jl, Lr, Mr, Tr, Ir, Thr, Nr, Jr>
        Div<System<V, Lr, Mr, Tr, Ir, Thr, Nr, Jr>>
        for System<V, Ll, Ml, Tl, Il, Thl, Nl, Jl>
    where V: Default + Div,
        Ll: Integer + Sub<Lr>,
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
    type Output = System<
        V,
        <Ll as Sub<Lr>>::Output,
        <Ml as Sub<Mr>>::Output,
        <Tl as Sub<Tr>>::Output,
        <Il as Sub<Ir>>::Output,
        <Thl as Sub<Thr>>::Output,
        <Nl as Sub<Nr>>::Output,
        <Jl as Sub<Jr>>::Output>;

    fn div(self, _rhs: System<V, Lr, Mr, Tr, Ir, Thr, Nr, Jr>) -> Self::Output {
        //Self::Output::new(self.value / rhs.value)
        Self::Output::default()
    }
}
