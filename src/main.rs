extern crate tch;
use minifb::{Key, Window, WindowOptions};
use rand::{thread_rng, Rng};
use std::{
    thread,
    time::{Duration, Instant},
};

mod world {
    pub mod animator;
    pub mod beast;
    pub mod world;
}

use world::{animator::Animator, world::World};

// Visuals
const ANIMATE: bool = true; // Set this to false to disable animation and speed up training.
pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 800;
pub const FPS: u64 = 10;

fn main() {
    let delay: Duration = Duration::from_millis(1000 / FPS);
    let mut animator: Animator = Animator::new(WIDTH, HEIGHT);
    let mut world: World = World::new();
    let mut rng = thread_rng();

    if ANIMATE {
        while true {
            // Init the world
            'animeate_loop: while world.continue_simulation()
                && animator.window.is_open()
                && !animator.window.is_key_down(Key::Escape)
            {
                world.step();
                animator.step(&world, delay);
            }
        }
    } else {
        while true {
            world.step();
        }
    }
}
