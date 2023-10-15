use crate::environment::animator::Animator;
use crate::environment::beast::{Beast, BeastType};

use rand::{rngs::ThreadRng, thread_rng, Rng};

use super::beast;

#[derive(PartialEq)]
pub enum Entity {
    Plant(Plant),
    Beast(Beast),
}

#[derive(PartialEq)]
pub struct Plant {
    pub location: (f64, f64),
}

pub struct World {
    pub entities: Vec<Entity>,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        World {
            entities: Vec::new(),
            width: width,
            height: height,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.retain(|e| e != &entity);
    }

    fn contains_type(&mut self, beast_type: BeastType) -> bool {
        for entity in self.entities.iter() {
            match entity {
                Entity::Beast(beast) => {
                    if beast.beast_type == beast_type {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    pub fn continue_simulation(&mut self, animator: Option<&Animator>) -> bool {
        match animator {
            Some(animator) => {
                self.contains_type(BeastType::Herbivore)
                    && self.contains_type(BeastType::Carnivore)
                    && animator.contine_animation()
            }
            None => self.contains_type(BeastType::Herbivore) && self.contains_type(BeastType::Carnivore),
        }
    }

    fn clear_world(&mut self) {
        self.entities.clear();
    }

    pub fn restart_world(&mut self, plants: usize, herbivores: usize, carnivores: usize) {
        let mut rng = thread_rng();
        self.clear_world();
        for _ in 0..herbivores {
            let x = rng.gen_range(0.0..(self.width as f64));
            let y = rng.gen_range(0.0..(self.height as f64));
            self.add_beast(BeastType::Herbivore, (x, y));
        }
        for _ in 0..carnivores {
            let x = rng.gen_range(0.0..(self.width as f64));
            let y = rng.gen_range(0.0..(self.height as f64));
            self.add_beast(BeastType::Carnivore, (x, y));
        }
    }

    fn add_plant(&mut self, location: (f64, f64)) {
        self.add_entity(Entity::Plant(Plant { location }));
    }

    fn add_beast(&mut self, beast_type: BeastType, location: (f64, f64)) {
        let beast = Beast::new(beast_type, location);
        self.add_entity(Entity::Beast(beast));
    }

    pub fn step(&mut self) {
        for entity in self.entities.iter_mut() {
            match entity {
                Entity::Plant(plant) => {
                    //plant.step();
                }
                Entity::Beast(beast) => {
                    beast.step();
                }
            }
        }
    }
}
