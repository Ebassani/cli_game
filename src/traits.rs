use num::traits::NumAssign;
use rand::{
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    rngs::ThreadRng,
    Rng,
};
use std::ops::Range;

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Point<T>{
    pub(crate) x: T,
    pub(crate) y: T
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Self {
            x,
            y
        }
    }
}


pub trait Position<T: NumAssign + Copy + PartialEq> {
    fn position(&self) -> Point<T>;
    fn set_position(&mut self, position: Point<T>);
    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<T>, y_range: Range<T>)
    where
        T: PartialOrd + SampleUniform,
        Standard: Distribution<T>,
    {
        let new_position = Point::new(rng.gen_range(x_range), rng.gen_range(y_range));
        self.set_position(new_position);
    }
}
