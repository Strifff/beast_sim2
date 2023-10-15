extern crate tch;

use std::time::{Duration, Instant};

mod environment {
    pub mod animator;
    pub mod beast;
    pub mod world;
}

use environment::{animator::Animator, world::World};

// Visuals
const ANIMATE: bool = true; // Set this to false to disable animation and speed up training.
pub const WIDTH: usize = 520;
pub const HEIGHT: usize = 520;
pub const FPS: u64 = 60;
pub const BORDER: usize = 10;

// Environment
pub const HERBI_START: usize = 5;
pub const CARNI_START: usize = 5;
pub const BEAST_MEMORY_TIME: f64 = 10.0;
pub const PLANT_START: usize = 10;
pub const PLANT_GRID: usize = 5; // Number of plants per row and column
pub const SPROUT_RATE: f64 = 0.001;
pub const PLANT_ENERGY: f64 = 100.0;

// Neural network

fn main() {
    let delay: Duration = Duration::from_millis(1000 / FPS);
    let mut animator: Animator = Animator::new(WIDTH, HEIGHT);
    let mut world: World = World::new(WIDTH, HEIGHT, PLANT_GRID, BORDER, PLANT_ENERGY, SPROUT_RATE, BEAST_MEMORY_TIME);

    if ANIMATE {
        'animate_loop: while animator.contine_animation() {
            // Init the world
            world.restart_world(PLANT_START, HERBI_START, CARNI_START);
            'simulate_loop: while world.continue_simulation(Some(&animator)) {
                let earlier = Instant::now();
                world.step();
                animator.step(&world, delay, earlier);
            }
        }
    } else {
        'train_loop: loop {
            // Init the world
            world.restart_world(PLANT_START, HERBI_START, CARNI_START);
            'simulate_loop: while world.continue_simulation(None) {
                world.step();
            }
        }
    }
}
