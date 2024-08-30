use std::fmt::{Display, Formatter};
use std::ops::Range;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::traits::{Position, Point};

#[derive(Default)]
pub struct Player {
    health: u8,
    speed: f64,
    position: Point<f64>
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[]")
    }
}

impl Player {
    pub fn builder() -> PlayerBuilder {
        PlayerBuilder::new()
    }

    pub fn is_alive(&self) -> bool {
        if self.health > 0 { true }
        else { false }
    }

    pub fn take_damage(&mut self, dmg: u8) {
        self.health -= dmg;
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }
}

impl Position<f64> for Player {
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

#[derive(Default)]
pub struct PlayerBuilder {
    health: u8,
    speed: f64
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self { health: 0, speed: 0.0 }
    }

    pub fn speed(mut self, value: f64) -> Self {
        self.speed = value;
        self
    }

    pub fn health(mut self, value: u8) -> Self {
        self.health = value;
        self
    }

    pub fn build(self) -> Player {
        Player{
            health: self.health,
            speed: self.speed,
            position: Default::default(),
        }
    }
}