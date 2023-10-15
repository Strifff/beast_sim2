extern crate tch;
use minifb::{Key, Window, WindowOptions};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{
    thread,
    time::{Duration, Instant},
};

mod environment {
    pub mod animator;
    pub mod beast;
    pub mod world;
}

use environment::{animator::Animator, world::World};

// Visuals
const ANIMATE: bool = true; // Set this to false to disable animation and speed up training.
pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
pub const FPS: u64 = 60;
pub const BORDER: usize = 10;

// World
pub const HERBI_START: usize = 5;
pub const CARNI_START: usize = 5;
pub const PLANT_START: usize = 10;
pub const PLANT_ENERGY: f64 = 100.0;

// Neural network

fn main() {
    let delay: Duration = Duration::from_millis(1000 / FPS);
    let mut animator: Animator = Animator::new(WIDTH, HEIGHT);
    let mut world: World = World::new(WIDTH, HEIGHT);

    if ANIMATE {
        'animate_loop: while Animator::contine_animation(&animator) {
            // Init the world
            world.restart_world(PLANT_START, HERBI_START, CARNI_START);
            'simulate_loop: while world.continue_simulation(Some(&animator)) {
                world.step();
                animator.step(&world, delay);
            }
        }
    } else {
        'train_loop: while true {
            // Init the world
            world.restart_world(PLANT_START, HERBI_START, CARNI_START);
            'simulate_loop: while world.continue_simulation(None) {
                world.step();
            }
        }
    }
}
