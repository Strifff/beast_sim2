use crate::environment::animator::Animator;
use crate::environment::beast::{Beast, BeastType};

use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::cell::RefCell;
use std::rc::Rc;

use super::beast;

#[derive(PartialEq)]
pub enum Entity {
    Plant(Rc<RefCell<Plant>>),
    Beast(Rc<RefCell<Beast>>),
    //Plant(Plant),
    //Beast(Beast),
}

#[derive(PartialEq)]
pub struct Plant {
    location: (f64, f64),
    grid_size: (f64, f64),
    pub x: f64,
    pub y: f64,
    pub energy: f64,
    pub sprouted: bool,
    sprout_rate: f64,
}

impl Plant {
    pub fn new(location: (f64, f64), grid_size: (f64, f64), sprout_rate: f64) -> Self {
        Plant {
            location,
            grid_size,
            x: location.0,
            y: location.1,
            energy: 100.0,
            sprouted: false,
            sprout_rate: sprout_rate,
        }
    }
    pub fn step(&mut self) {
        let mut rng: ThreadRng = thread_rng();
        if !self.sprouted && rng.gen_range(0.0..1.0) < self.sprout_rate {
            self.x = self.location.0 + rng.gen_range(0.0..self.grid_size.0);
            self.y = self.location.1 + rng.gen_range(0.0..self.grid_size.1);
            self.sprouted = true;
        }
    }
}

pub struct World {
    pub entities: Rc<RefCell<Vec<Entity>>>,
    //pub entities: Vec<Entity>,
    pub width: usize,
    pub height: usize,
    pub grid_size: usize,
    pub border: usize,
    pub grid_sqaure_size_x: f64,
    pub grid_sqaure_size_y: f64,
    pub plant_energy: f64,
    pub sprout_rate: f64,
    pub beast_memory_time: f64,
}

impl World {
    pub fn new(
        width: usize,
        height: usize,
        plant_grid: usize,
        border: usize,
        plant_energy: f64,
        sprout_rate: f64,
        beast_memory_time: f64,
    ) -> Self {
        World {
            entities: Rc::new(RefCell::new(Vec::new())),
            //entities: Vec::new(),
            width: width,
            height: height,
            grid_size: plant_grid,
            border: border,
            grid_sqaure_size_x: (width as f64 - (2 * border) as f64) / (plant_grid as f64),
            grid_sqaure_size_y: (height as f64 - (2 * border) as f64) / (plant_grid as f64),
            plant_energy: plant_energy,
            sprout_rate: sprout_rate,
            beast_memory_time: beast_memory_time,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        //self.entities.push(entity);
        let entities = self.entities.clone();
        let mut entities_mut = entities.borrow_mut();
        entities_mut.push(entity);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        //self.entities.retain(|e| e != &entity);
        let mut entities = self.entities.borrow_mut();
        entities.retain(|e| e != &entity);
        // TODO remove from beast memory
    }

    fn contains_type(&self, beast_type: BeastType) -> bool {
        let entities = self.entities.borrow();

        for entity in entities.iter() {
            match entity {
                Entity::Beast(beast_rc) => {
                    let beast = beast_rc.borrow();
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
            None => {
                self.contains_type(BeastType::Herbivore) && self.contains_type(BeastType::Carnivore)
            }
        }
    }

    fn clear_world(&mut self) {
        //self.entities.clear();
        self.entities.borrow_mut().clear();
    }

    pub fn restart_world(&mut self, _plants: usize, herbivores: usize, carnivores: usize) {
        let mut rng = thread_rng();
        self.clear_world();
        for _ in 0..herbivores {
            self.add_beast(BeastType::Herbivore);
        }
        for _ in 0..carnivores {
            self.add_beast(BeastType::Carnivore);
        }
        self.add_plant_uniformly();
    }

    fn add_plant_uniformly(&mut self) {
        for x in 0..self.grid_size {
            for y in 0..self.grid_size {
                let grid_start_x = self.border as f64 + x as f64 * self.grid_sqaure_size_x;
                let grid_start_y = self.border as f64 + y as f64 * self.grid_sqaure_size_y;
                let plant = Plant::new(
                    (grid_start_x, grid_start_y),
                    (self.grid_sqaure_size_x, self.grid_sqaure_size_y),
                    self.sprout_rate,
                );
                self.add_entity(Entity::Plant(plant));
            }
        }
    }

    fn add_beast(&mut self, beast_type: BeastType) {
        let mut rng = thread_rng();
        let location = (
            self.border as f64 + rng.gen_range(0.0..self.width as f64),
            self.border as f64 + rng.gen_range(0.0..self.height as f64),
        );
        let beast = Beast::new(beast_type, location, self.beast_memory_time);
        self.add_entity(Entity::Beast(beast));
    }

    fn world_step_beast(&mut self, beast: &Beast) {}

    pub fn step(&mut self) {
        for entity in &mut self.entities {
            match entity {
                Entity::Plant(plant) => {
                    plant.step();
                }
                Entity::Beast(beast) => {
                    //self.world_step_beast(beast)
                }
            }
        }
    }
}
