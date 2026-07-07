pub mod graphics;
pub mod global_constants;
pub mod textures;
pub mod math;
pub mod controls;

use crate::graphics::*;
use crate::global_constants::*;
use crate::textures::cube::*;
use crate::math::*;
use crate::controls::*;
use macroquad::color;
use macroquad::prelude::*;
use std::thread;
use std::time::Duration;

fn window_conf() -> Conf {
    Conf {
        window_title: "Jumby 3D".to_owned(),
        window_width: global_constants::SCREEN_RES[0],
        window_height: global_constants::SCREEN_RES[1],
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let dt = (get_frame_time() * 100.0) as u64;

    let mut test: Cube = Cube::new(10.0, color::WHITE, 1.0);

    let init_cam_pos = vec::Vec3::new(0, 0, -80);
    let mut cam = graphics::camera::Camera::new(init_cam_pos, [90.0, 60.0]);
    println!("{:?} \n\n\n {:?}", test, cam);

    loop {
        clear_background(color::BLACK);
        graphics::render(&test, &cam);
        test.mov_obj();
        test.rot();
        next_frame().await;
        thread::sleep(Duration::from_millis(30 - dt));
    }
}
