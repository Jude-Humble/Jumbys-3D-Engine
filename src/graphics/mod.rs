pub mod camera;

use crate::math::vec;
use crate::global_constants;
use macroquad::prelude::*;
use macroquad::color;

pub trait Renderable {
    fn vertices(&self) -> &[vec::Vec3<i32>];
    fn indices(&self) -> &[Option<usize>];

    fn color(&self) -> Color;

    fn thickness(&self) -> f32;
}

pub fn render(obj: &impl Renderable, cam: &camera::Camera) {
    let indices = obj.indices();
    let mut vertices: Vec<vec::Vec3<f32>> =
        obj.vertices()
            .iter()
            .map(|v: &vec::Vec3<i32>| vec::Vec3::new(v.x as f32, v.y as f32, v.z as f32))
            .collect();

    for v in vertices.iter_mut() {
        v.x = ( v.x * cam.n[0] ) / ( v.z + cam.n[0] ) - cam.camera_pos.x as f32;
        v.y = ( v.y * cam.n[1] ) / ( v.z + cam.n[1] ) - cam.camera_pos.x as f32;
        v.z = 0.0
    }

    let mut prev: Option<usize> = None;
    for i in indices.iter() {
        if let Some(pre) = prev && let Some(index) = i {
            draw_line(vertices[pre].x, vertices[pre].y, vertices[*index].x, vertices[*index].y, obj.thickness(), obj.color());
        }
        prev = *i;
    }
}