#[derive(Default)]
pub struct Player {
    health: u8,
    speed: f64
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
            speed: self.speed
        }
    }
}