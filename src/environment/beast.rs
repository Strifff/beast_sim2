use rand::{rngs::ThreadRng, thread_rng, Rng};

use super::world::{Entity, World, Plant};

use std::cell::RefCell;
use std::rc::Rc;

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
    pub memory_time: f64,
    pub memory: Vec<(Entity, f64)>,
}

#[derive(PartialEq, Debug)]
pub enum BeastType {
    Herbivore,
    Carnivore,
}

impl Beast {
    pub fn new(beast_type: BeastType, location: (f64, f64), memory_time: f64) -> Self {
        let speed = 1.0;
        let direction = 0.0;
        let energy = 100.0;
        let age = 0.0;
        let fov = 90.0 / 180.0 * std::f64::consts::PI;
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
            memory_time,
            memory: Vec::new(),
        }
    }

    pub fn in_view(&self, entity: &Entity) -> bool {
        match entity {
            Entity::Plant(plant) => {
                self.plant_in_view(plant) && self.plant_in_range(plant, self.sight_range)
            }
            Entity::Beast(beast) => {
                self.beast_in_view(beast) && self.beast_in_range(beast, self.sight_range)
            }
        }
    }
    pub fn plant_in_view(&self, plant: &Rc<RefCell<Plant>>) -> bool {
        let x = self.location.0;
        let y = self.location.1;
        let plant_borrow = plant.borrow();
        let plant_x = plant_borrow.x;
        let plant_y = plant_borrow.y;
        let dist = ((plant_x - x).powi(2) + (plant_y - y).powi(2)).sqrt();
        dist <= self.sight_range
    }

    pub fn beast_in_view(&self, beast: &Rc<RefCell<Beast>>) -> bool {
        let x = self.location.0;
        let y = self.location.1;
        let beast_borrow = beast.borrow();
        let beast_x = beast_borrow.location.0;
        let beast_y = beast_borrow.location.1;
        let dist = ((beast_x - x).powi(2) + (beast_y - y).powi(2)).sqrt();
        dist <= self.sight_range
    }

    pub fn in_direction(&self, entity: &Entity) -> bool {
        match entity {
            Entity::Plant(plant) => self.plant_in_direction(plant),
            Entity::Beast(beast) => self.beast_in_direction(beast),
        }
    }

    pub fn plant_in_direction(&self, plant: &Rc<RefCell<Plant>>) -> bool {
        let x = self.location.0;
        let y = self.location.1;
        let plant_borrow = plant.borrow();
        let plant_x = plant_borrow.x;
        let plant_y = plant_borrow.y;
        let dist = ((plant_x - x).powi(2) + (plant_y - y).powi(2)).sqrt();
        let angle = (plant_y - y).atan2(plant_x - x);
        let left_bound = self.direction - self.fov / 2.0;
        let right_bound = self.direction + self.fov / 2.0;
        dist <= self.sight_range && (angle >= left_bound && angle <= right_bound)
    }

    pub fn beast_in_direction(&self, beast: &Rc<RefCell<Beast>>) -> bool {
        let x = self.location.0;
        let y = self.location.1;
        let beast_borrow = beast.borrow();
        let beast_x = beast_borrow.location.0;
        let beast_y = beast_borrow.location.1;
        let dist = ((beast_x - x).powi(2) + (beast_y - y).powi(2)).sqrt();
        let angle = (beast_y - y).atan2(beast_x - x);
        let left_bound = self.direction - self.fov / 2.0;
        let right_bound = self.direction + self.fov / 2.0;
        dist <= self.sight_range && (angle >= left_bound && angle <= right_bound)
    }

    pub fn in_range(&self, entity: &Entity) -> bool {
        match entity {
            Entity::Plant(plant) => self.plant_in_range(plant, self.sight_range),
            Entity::Beast(beast) => self.beast_in_range(beast, self.sight_range),
        }
    }

    pub fn plant_in_range(&self, plant: &Rc<RefCell<Plant>>, range: f64) -> bool {
        let x = self.location.0;
        let y = self.location.1;
        let plant_borrow = plant.borrow();
        let plant_x = plant_borrow.x;
        let plant_y = plant_borrow.y;
        let dist = ((plant_x - x).powi(2) + (plant_y - y).powi(2)).sqrt();
        dist <= range
    }

    pub fn beast_in_range(&self, beast: &Rc<RefCell<Beast>>, range: f64) -> bool {
        let x = self.location.0;
        let y = self.location.1;
        let beast_borrow = beast.borrow();
        let beast_x = beast_borrow.location.0;
        let beast_y = beast_borrow.location.1;
        let dist = ((beast_x - x).powi(2) + (beast_y - y).powi(2)).sqrt();
        dist <= range
    }

    pub fn add_to_memory(&mut self, entity: Entity) {
        if self.in_view(&entity) {
            if self.memory.iter().any(|(e, _)| e == &entity) {
                self.memory
                    .retain(|(e, _)| e != &entity);
            }
            self.memory.push((entity, self.memory_time));
        }
    }

    pub fn remove_from_memory(&mut self, entity: Entity) {
        self.memory
            .retain(|(e, _)| e != &entity);
    }

    pub fn memory_forget(&mut self) {
        self.memory.retain(|(_, time)| *time > 0.0);
        for (_, time) in self.memory.iter_mut() {
            *time -= 1.0;
        }
    }

    pub fn move_randomly(&mut self) {
        let mut rng: ThreadRng = thread_rng();
        self.direction = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
        self.location.0 += self.speed * self.direction.cos();
        self.location.1 += self.speed * self.direction.sin();
    }

    pub fn step(&mut self, world: &Rc<RefCell<Vec<Entity>>>) {
        self.memory_forget();

        self.move_randomly();
        self.age += 1.0;
        self.energy -= 1.0;
    }
}
