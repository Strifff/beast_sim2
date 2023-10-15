use rand::{thread_rng, Rng, rngs::ThreadRng};

#[derive(PartialEq)]
pub struct Beast {
    pub beast_type: BeastType,
    pub location: (f64, f64),
    speed: f64,
    pub direction: f64,
    energy: f64,
    age: f64,
    pub fov: f64,
    pub sight_range: f64,
}

#[derive(PartialEq, Debug)]
pub enum BeastType {
    Herbivore,
    Carnivore,
}

impl Beast {
    pub fn new(beast_type: BeastType, location: (f64, f64)) -> Self {
        let speed = 1.0;
        let direction = 0.0;
        let energy = 100.0;
        let age = 0.0;
        let fov = 90.0/ 180.0 * std::f64::consts::PI;
        let sight_range = 25.0;
        let _rng: ThreadRng = thread_rng();

        Self {
            beast_type,
            location,
            speed,
            direction,
            energy,
            age,
            fov,
            sight_range,
        }
    }
    pub fn move_randomly(&mut self) {
        let mut rng: ThreadRng = thread_rng();
        self.direction = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
        self.location.0 += self.speed * self.direction.cos();
        self.location.1 += self.speed * self.direction.sin();
    }

    pub fn step(&mut self) {
        self.move_randomly();
        self.age += 1.0;
        self.energy -= 1.0;
    }
}