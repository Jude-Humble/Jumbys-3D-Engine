use crate::math::vec::*;

pub struct Camera {
    camera_pos: Vec3<i32>,
    fov: f32,
}

impl Camera {
    pub fn new(camera_pos: Vec3<i32>, fov: f32) -> Self {
        Self {
            camera_pos,
            fov,
        }
    }

    pub fn move_camera(&mut self, direction: Vec3<i32>) {
        self.camera_pos = self.camera_pos + direction;
    }
}