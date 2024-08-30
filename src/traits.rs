use num::traits::NumAssign;
use rand::{
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    rngs::ThreadRng,
    Rng,
};
use std::ops::{Add, AddAssign, Mul, Range};

#[derive(Default, Copy, Clone, PartialEq, Debug)]
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

impl Add for Point<f64> {
    type Output = Point<f64>;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.x;
        Point::new(x,y)
    }
}

impl Point<f64>{
    pub fn round(self) -> Point<f64> {
        let round_x = self.x.round();
        let round_y = self.y.round();

        Point::new(round_x, round_y)
    }

    pub fn to_u16(self) -> Point<u16> {
        let x = self.x as u16;
        let y = self.y as u16;

        Point::new(x, y)
    }
}

impl Mul<f64> for Point<f64> {
    type Output = Point<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl AddAssign for Point<f64> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
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
