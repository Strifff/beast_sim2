use crate::world::beast::{Beast, BeastType};

#[derive(PartialEq)]
pub enum Entity {
    Plant(Plant),
    Beast(Beast),
}

#[derive(PartialEq)]
struct Plant {
    location: (f64, f64),    
}

pub struct World {
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: Vec::new(),
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

    pub fn continue_simulation(&mut self) -> bool {
        self.contains_type(BeastType::Herbivore) && self.contains_type(BeastType::Carnivore)
    }

    fn clear_world(&mut self) {
        self.entities.clear();
    }

    pub fn restart_world(&mut self) {
        self.clear_world();

    }

    fn add_plant(&mut self, location: (f64, f64)) {
        self.add_entity(Entity::Plant(Plant { location }));
    }

    fn add_beast(&mut self, beast_type: BeastType, location: (f64, f64)) {
        self.add_entity(Entity::Beast(Beast::new(beast_type, location)));
    }

    pub fn step(&mut self) {
        for entity in self.entities.iter_mut() {
            match entity {
                Entity::Plant(plant) => {
                    //plant.step();
                }
                Entity::Beast(beast) => {
                    //beast.step();
                }
            }
        }
    }
}
