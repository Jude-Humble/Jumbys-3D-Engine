use crate::global_constants::SCREEN_RES;
use crate::math::vec::*;

#[derive(Debug)]
pub struct Camera {
    pub camera_pos: Vec3<i32>,
    pub fov: [f32; 2],
    pub n: [f32; 2],
}

impl Camera {
    pub fn new(camera_pos: Vec3<i32>, fov: [f32; 2]) -> Self {
        Self {
            camera_pos,
            fov,
            n: [
                SCREEN_RES[0] as f32 / fov[0].tan(),
                SCREEN_RES[1] as f32 / fov[1].tan(),
            ]
        }
    }

    pub fn move_camera(&mut self, direction: Vec3<i32>) {
        self.camera_pos = self.camera_pos + direction;
    }
}