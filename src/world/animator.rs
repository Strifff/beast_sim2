use minifb::{Key, Window, WindowOptions};
use std::{time::{Duration, Instant}, thread};

use crate::world::world::World;

pub struct Animator {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    pub window: Window,
}

impl Animator {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer: Vec<u32> = vec![0; width * height];
        let window = Window::new("Simple Animator", width, height, WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        Self {
            width,
            height,
            buffer,
            window,
        }
    }

    pub fn step(&mut self, world: &World, delay: Duration) {
        let now = Instant::now();
        // Clear the buffer
        for pixel in self.buffer.iter_mut() {
            *pixel = 0; // Set all pixels to black
        }



        // Update the window with the buffer
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
        let elapsed = now.elapsed();
        dynamic_delay(delay, elapsed)
    }
}


fn dynamic_delay(delay: Duration, elapsed: Duration){
    let sleep_time = delay - elapsed;
    if sleep_time > Duration::from_millis(0) {
        thread::sleep(sleep_time);
    } else {
        println!("Warning: animation is running slow!");
    }
}
