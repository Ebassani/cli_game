use std::fmt::{Display, Formatter};
use std::io::Write;
use std::ops::Range;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::traits::{Point, Position};
use crate::ui::draw::Draw;

#[derive(Default)]
pub struct Wall {
    position: Point<u16>
}

impl Wall {
    pub fn new(x: u16, y: u16) -> Self {
        Wall {
            position: Point::new(x,y)
        }
    }
}

impl Display for Wall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "|")
    }
}

impl Position<u16> for Wall {
    fn position(&self) -> Point<u16> {
        self.position
    }

    fn set_position(&mut self, position: Point<u16>) {
        self.position = position;
    }

    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<u16>, y_range: Range<u16>)
    {
        let new_position = Point::new(rng.gen_range(x_range), rng.gen_range(y_range));
        self.set_position(new_position);
    }
}
