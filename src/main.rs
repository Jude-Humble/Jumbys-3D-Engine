pub mod graphics;
pub mod global_constants;
pub mod textures;
pub mod math;

use crate::graphics::*;
use crate::textures::cube::*;
use crate::math::*;
use macroquad::color;
use macroquad::prelude::*;

#[macroquad::main("Render 3D Window")]
async fn main() {

    let mut test: Cube = Cube::new(20, color::WHITE, 1.0);

    let init_cam_pos: vec::Vec3<i32> = vec::Vec3::new(-100, -100, -10);
    let mut cam: graphics::camera::Camera = graphics::camera::Camera::new(init_cam_pos, [90.0, 60.0]);

    println!("{:?} \n\n\n {:?}", test, cam);

    loop {
        clear_background(color::BLACK);
        graphics::render(&test, &cam);
        test.mov(vec::Vec3::new(1, 1, 0));
        next_frame().await;
    }
}