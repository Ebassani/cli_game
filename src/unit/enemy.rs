use std::ops::Range;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::traits::{Point, Position};

#[derive(Default)]
pub struct Enemy{
    speed: f64,
    position: Point<f64>
}

impl Position<f64> for Enemy {
    fn position(&self) -> Point<f64> {
        self.position
    }

    fn set_position(&mut self, position: Point<f64>) {
        self.position = position;
    }

    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<f64>, y_range: Range<f64>)
    {
        let new_position = Point::new(rng.gen_range(x_range), rng.gen_range(y_range));
        self.set_position(new_position);
    }
}

impl Enemy {
    pub fn with_speed(speed: f64) -> Self {
        Self {speed, position: Default::default() }
    }
}