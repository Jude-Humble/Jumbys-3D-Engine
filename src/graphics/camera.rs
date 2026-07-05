use crate::global_constants::SCREEN_RES;
use crate::math::vec::Vec3;

#[derive(Debug)]
pub struct Camera {
    pub camera_pos: Vec3<f32>,
    pub fov: [f32; 2],
    pub focal: [f32; 2],
}

impl Camera {
    pub fn new(camera_pos: Vec3<i32>, fov: [f32; 2]) -> Self {
        let aspect_x = SCREEN_RES[0] as f32;
        let aspect_y = SCREEN_RES[1] as f32;

        let focal = [
            (aspect_x * 0.5) / (fov[0].to_radians() * 0.5).tan(),
            (aspect_y * 0.5) / (fov[1].to_radians() * 0.5).tan(),
        ];

        Self {
            camera_pos: Vec3::new(
                camera_pos.x as f32,
                camera_pos.y as f32,
                camera_pos.z as f32,
            ),
            fov,
            focal,
        }
    }

    pub fn move_camera(&mut self, direction: Vec3<f32>) {
        self.camera_pos = self.camera_pos + direction;
    }
}