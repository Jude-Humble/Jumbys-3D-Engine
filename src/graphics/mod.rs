mod camera;

use crate::math::vec;
use crate::global_constants;
use macroquad::prelude::*;
use macroquad::color;

pub trait Renderable {
    fn vertices(&self) -> &[vec::Vec3<i32>];
    fn indices(&self) -> &[u32];
}

pub fn render(obj: &impl Renderable, cam: &camera::Camera) {
    let indices = obj.indices();
    let mut vertices: Vec<vec::Vec3<f32>> =
        obj.vertices()
            .iter()
            .map(|v: &vec::Vec3<i32>| vec::Vec3::new(v.x as f32, v.y as f32, v.z as f32))
            .collect();

    for v in vertices.iter_mut() {
        v.x = ( v.x * n[0] ) / ( v.z + n[0] ) - cam.camera_pos.x as f32;
        v.y = ( v.y * n[1] ) / ( v.z + n[1] ) - cam.camera_pos.x as f32;
        v.z = 0.0
    }
}