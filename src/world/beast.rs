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

#[derive(PartialEq)]
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
        let fov = 0.0;
        let sight_range = 0.0;

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

    pub fn step(&mut self) {
        self.age += 1.0;
        self.energy -= 1.0;
    }
}