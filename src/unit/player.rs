use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, Mul, Range};
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::traits::{Position, Point};

#[derive(Default)]
pub struct Player {
    health: u8,
    speed: f64,
    position: Point<f64>,
    direction: Point<f64>
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

    pub fn accelerate(&mut self) {
        self.speed = 0.9
    }

    pub fn decelerate(&mut self) {
        self.speed = 0.0
    }

    pub fn forward_position(&self) -> Point<u16> {
        let forward = self.position + self.direction;
        forward.round().to_u16()
    }

    pub fn move_forward(&mut self){
        self.position += self.direction * self.speed;
    }

    pub fn turn_left(&mut self) {
        let north = Point::new(0.0, -1.0);
        let northeast = Point::new(0.7061, -0.7061);
        let east = Point::new(1.0, 0.0);
        let southeast = Point::new(0.7061, 0.7061);
        let south = Point::new(0.0, 1.0);
        let southwest = Point::new(-0.7061, 0.7061);
        let west = Point::new(-1.0, 0.0);
        let northwest = Point::new(-0.7061, -0.7061);

        self.direction = match self.direction {
            p if p == north =>  northwest,
            p if p == northeast =>  north,
            p if p == east =>  northeast,
            p if p == southeast =>  east,
            p if p == south =>  southeast,
            p if p == southwest =>  south,
            p if p == west =>  southwest,
            p if p == northwest =>  west,
            _ => self.direction
        }

    }

    pub fn turn_right(&mut self) {
        let north = Point::new(0.0, -1.0);
        let northeast = Point::new(0.7061, -0.7061);
        let east = Point::new(1.0, 0.0);
        let southeast = Point::new(0.7061, 0.7061);
        let south = Point::new(0.0, 1.0);
        let southwest = Point::new(-0.7061, 0.7061);
        let west = Point::new(-1.0, 0.0);
        let northwest = Point::new(-0.7061, -0.7061);

        self.direction = match self.direction {
            p if p == north =>  northeast,
            p if p == northeast =>  east,
            p if p == east =>  southeast,
            p if p == southeast =>  south,
            p if p == south =>  southwest,
            p if p == southwest =>  west,
            p if p == west =>  northwest,
            p if p == northwest =>  north,
            _ => self.direction
        };
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
    speed: f64,
    direction: Point<f64>
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self { health: 0, speed: 0.0, direction: Default::default() }
    }

    pub fn speed(mut self, value: f64) -> Self {
        self.speed = value;
        self
    }

    pub fn health(mut self, value: u8) -> Self {
        self.health = value;
        self
    }

    pub fn direction(mut self, x: f64, y:f64) -> Self {
        self.direction = Point::new(x,y);
        self
    }

    pub fn build(self) -> Player {
        Player{
            health: self.health,
            speed: self.speed,
            position: Default::default(),
            direction: self.direction,
        }
    }
}