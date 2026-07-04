pub mod graphics;
pub mod global_constants;
pub mod textures;
pub mod math;

use crate::graphics::*;
use crate::textures::cube::*;
use macroquad::color;
use macroquad::prelude::*;

#[macroquad::main("Render 3D Window")]
async fn main() {

    let mut test: Cube = Cube::new(20, color::WHITE, 1.0);

    loop {
        clear_background(color::BLACK);
        graphics::render(&test);
        next_frame().await;
    }
}