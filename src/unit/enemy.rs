use std::fmt::{Display, Formatter};
use std::ops::Range;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::traits::{Point, Position};
use crate::unit::Player;

#[derive(Default)]
pub struct Enemy{
    speed: f64,
    position: Point<f64>
}

impl Display for Enemy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "M")
    }
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

    pub fn move_towards_player(&mut self, player: &Player) {

        let direction_x = player.position().x - self.position.x;
        let direction_y = player.position().y - self.position.y;

        let magnitude = (direction_x.powi(2) + direction_y.powi(2)).sqrt();

        let normalized_x = direction_x / magnitude;
        let normalized_y = direction_y / magnitude;

        let direction = Point::new(normalized_x, normalized_y);

        self.position += direction * self.speed;
    }
}