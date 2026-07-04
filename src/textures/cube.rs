use macroquad::color;
use crate::math::vec::Vec3;
use crate::graphics;

#[derive(Debug)]
pub struct Cube {
    pub vertices: Vec<Vec3<i32>>,
    pub indices: Vec<u32>,
    color: color::Color,
    thickness: f32,
}

impl Cube {

    pub fn new(side_length: i32, color: color::Color, thickness: f32) -> Self {
        let init_cube_size = vec![
            Vec3::new(  side_length,  side_length,  side_length ), // top-right-back     #0
            Vec3::new(  side_length, -side_length,  side_length ), // top-left-back      #1
            Vec3::new( -side_length,  side_length,  side_length ), // top-right-front    #2
            Vec3::new( -side_length, -side_length,  side_length ), // top-left-fron      #3
            Vec3::new(  side_length,  side_length, -side_length ), // bottom-right-back  #4
            Vec3::new(  side_length, -side_length, -side_length ), // bottom-left-back   #5
            Vec3::new( -side_length,  side_length, -side_length ), // bottom-right-front #6
            Vec3::new( -side_length, -side_length, -side_length ), // bottom-left-front  #7
        ];

        let indice_pattern = vec![
            0, 1, 3, 0, 2, 3, // top face
            0, 4, 5, 0, 1, 5, // back face
            0, 4, 6, 0, 2, 6, // right face
            7, 6, 2, 7, 3, 2, // front face
            7, 6, 4, 7, 5, 4, // bottom face
            7, 5, 1, 7, 3, 1, // left face
        ];

        Self {
            vertices: init_cube_size,
            indices: indice_pattern,
            color,
            thickness,
        }
    }
}

impl crate::graphics::Renderable for Cube {
    fn vertices(&self) -> &[Vec3<i32>] {
        self.vertices.as_slice()
    }

    fn indices(&self) -> &[u32] {
        self.indices.as_slice()
    }
}