pub mod graphics;
pub mod global_constants;
pub mod textures;
pub mod math;

use crate::graphics::*;
use crate::textures::cube::*;
use crate::math::*;
use macroquad::color;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Jumby 3D".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut test: Cube = Cube::new(10, color::WHITE, 1.0);

    let init_cam_pos = vec::Vec3::new(0, 0, -80);
    let mut cam = graphics::camera::Camera::new(init_cam_pos, [90.0, 60.0]);
    println!("{:?} \n\n\n {:?}", test, cam);

    loop {
        clear_background(color::BLACK);
        graphics::render(&test, &cam);
        test.mov(vec::Vec3::new(0, 0, 0));
        next_frame().await;
    }
}