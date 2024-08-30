use std::fmt::{Display, Formatter};
use std::ops::Range;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::traits::{Point, Position};

#[derive(Default)]
pub struct Collectible {
    position: Point<u16>
}

impl Display for Collectible {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*")
    }
}


impl Position<u16> for Collectible {
    fn position(&self) -> Point<u16> {
        self.position
    }

    fn set_position(&mut self, position: Point<u16>) {
        self.position = position;
    }

    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<u16>, y_range: Range<u16>) {
        let new_position = Point::new(rng.gen_range(x_range), rng.gen_range(y_range));
        self.set_position(new_position);
    }
}