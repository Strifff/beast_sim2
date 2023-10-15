use minifb::{Key, Window, WindowOptions};
use std::{
    thread,
    time::{Duration, Instant},
};

use crate::environment::beast::{Beast, BeastType};
use crate::environment::world::{Entity, Plant, World};

pub struct Animator {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    pub window: Window,
}

impl Animator {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer: Vec<u32> = vec![0; width * height];
        let window = Window::new("Beast simulator", width, height, WindowOptions::default())
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

    pub fn contine_animation(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    fn clear_buffer(&mut self, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }

    fn draw_circle(&mut self, x: usize, y: usize, radius: usize, color: u32) {
        let mut x = x as isize + radius as isize - 1;
        let mut y = y as isize;
        let mut x_usize = x as usize;
        let mut y_usize = y as usize;
        let mut dx = 1;
        let mut dy = 1;
        let mut err = dx - radius/2;
    
        while x_usize >= y_usize {
            self.buffer[x_usize + y_usize * self.width] = color;
            self.buffer[y_usize + x_usize * self.width] = color;
            self.buffer[x_usize + y_usize * self.width] = color;
            self.buffer[y_usize + x_usize * self.width] = color;
            self.buffer[x_usize + y_usize * self.width] = color;
            self.buffer[y_usize + x_usize * self.width] = color;
            self.buffer[x_usize + y_usize * self.width] = color;
            self.buffer[y_usize + x_usize * self.width] = color;
    
            if err <= 0 {
                y += 1;
                y_usize = y as usize;
                err += dy;
                dy += 2;
            }
    
            if err > 0 {
                x -= 1;
                x_usize = x as usize;
                dx += 2;
                err += dx - radius/2;
            }
        }
    }
    
    
    

    fn draw_cone(
        &mut self,
        x: usize,
        y: usize,
        radius: f64,
        fov: f64,
        direction: f64,
        color: u32,
    ) {
        let dir = direction / 180 as f64 * std::f64::consts::PI;
        let left_bound = dir - fov / 2.0;
        let right_bound = dir + fov / 2.0;
        for i in 0..radius as usize {
            for j in 0..radius as usize {
                let x = x as f64;
                let y = y as f64;
                let i = i as f64;
                let j = j as f64;
                let dist = (i * i + j * j).sqrt();
                let angle = (j / i).atan();
                if dist <= radius
                    && (angle >= left_bound && angle <= right_bound)
                    && (x + i).round() >= 0.0
                    && (x + i).round() < self.width as f64
                    && (y + j).round() >= 0.0
                    && (y + j).round() < self.height as f64
                {
                    self.buffer[(x + i).round() as usize
                        + ((y + j).round() as usize) * self.width] = color;
                }
            }
        }
    }

    fn draw_beast(&mut self, beast: &Beast) {
        let x = beast.location.0 as usize;
        let y = beast.location.1 as usize;

        let fov = beast.fov;
        let sight_range = beast.sight_range;
        let direction = beast.direction;
        self.draw_cone(x, y, sight_range, fov, direction, 0x67d6f5); // Set the pixel to black

        match beast.beast_type {
            BeastType::Herbivore => {
                self.draw_circle(x, y, 7, 0x69440e); // Set the pixel to blue
            }
            BeastType::Carnivore => {
                self.draw_circle(x, y, 10, 0xf27b1f); // Set the pixel to red
            }
        }
    }

    pub fn step(&mut self, world: &World, delay: Duration) {
        let earlier = Instant::now();
        // Clear the buffer
        self.clear_buffer(0xFFFFFF);

        // Draw the world
        for entity in world.entities.iter() {
            match entity {
                Entity::Plant(plant) => {
                    let x = plant.location.0 as usize;
                    let y = plant.location.1 as usize;
                    self.draw_circle(x, y, 3, 0x00FF00); // Set the pixel to green
                }
                Entity::Beast(beast) => self.draw_beast(beast),
            }
        }

        // Update the window with the buffer
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
        let now = Instant::now();
        let elapsed = now.duration_since(earlier);
        println!("Elapsed: {:?}", elapsed);
        dynamic_delay(delay, elapsed);
    }
}

fn dynamic_delay(delay: Duration, elapsed: Duration) {
    if elapsed < delay {
        let sleep_time = delay - elapsed;
        thread::sleep(sleep_time);
    } else {
        println!("Warning: animation is running slow!");
    }
}

// TODO - Add background as predrawn buffer
// TODO - Add a way to smooth shapes
