pub mod camera;

use crate::math::vec;
use crate::global_constants;
use macroquad::prelude::*;
use macroquad::color;
use crate::global_constants::SCREEN_RES;

pub trait Renderable {
    fn vertices(&self) -> &[vec::Vec3<f32>];
    fn indices(&self) -> &[Option<usize>];

    fn color(&self) -> Color;

    fn thickness(&self) -> f32;
}

pub fn render(obj: &impl Renderable, cam: &camera::Camera) {
    let indices = obj.indices();

    let mut vertices: Vec<vec::Vec3<f32>> = obj
        .vertices()
        .iter()
        .map(|v| vec::Vec3::new(v.x as f32, v.y as f32, v.z as f32))
        .collect();

    for v in vertices.iter_mut() {
        let x = v.x - cam.camera_pos.x;
        let y = v.y - cam.camera_pos.y;
        let z = v.z - cam.camera_pos.z;

        if z <= 0.1 {
            v.x = f32::INFINITY;
            v.y = f32::INFINITY;
            continue;
        }

        let px = (x * cam.focal[0]) / z;
        let py = (y * cam.focal[1]) / z;

        v.x = px + SCREEN_RES[0] as f32 * 0.5;
        v.y = py + SCREEN_RES[1] as f32 * 0.5;
        v.z = z;
    }

    let mut prev: Option<usize> = None;

    for i in indices.iter() {
        match (prev, i) {
            (Some(pre), Some(index)) => {
                let vpre = &vertices[pre];
                let vindex = &vertices[*index];

                if vpre.x.is_finite()
                    && vindex.x.is_finite()
                    && vpre.y.is_finite()
                    && vindex.y.is_finite()
                {
                    draw_line(
                        vpre.x,
                        vpre.y,
                        vindex.x,
                        vindex.y,
                        obj.thickness(),
                        obj.color(),
                    );
                }
            }
            _ => {}
        }
        prev = *i;
    }
}
