mod camera;

use crate::math::vec;
use macroquad::prelude::*;
use macroquad::color;

pub trait Renderable {
    fn vertices(&self) -> &[vec::Vec3<i32>];
    fn indices(&self) -> &[u32];
}

pub fn render(obj: &impl Renderable) {
    let indices = obj.indices();
    let vertices = obj.vertices();

    for index in indices.windows(2) {
        draw_line(
            vertices[index[0] as usize].x as f32,
            vertices[index[0] as usize].y as f32,
            vertices[index[1] as usize].x as f32,
            vertices[index[1] as usize].y as f32,
            5.0,
            color::WHITE
        );
    }
}