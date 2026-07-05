use macroquad::color;
use crate::math::vec::Vec3;
use crate::graphics;

#[derive(Debug)]
pub struct Cube {
    pub vertices: Vec<Vec3<i32>>,
    pub indices: Vec<Option<usize>>,
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

        let indice_pattern: Vec<Option<usize>> = vec![
            Some(0), Some(1), Some(3), None, Some(0), Some(2), Some(3), None, // top face
            Some(0), Some(4), Some(5), None, Some(0), Some(1), Some(5), None, // back face
            Some(0), Some(4), Some(6), None, Some(0), Some(2), Some(6), None, // right face
            Some(7), Some(6), Some(2), None, Some(7), Some(3), Some(2), None, // front face
            Some(7), Some(6), Some(4), None, Some(7), Some(5), Some(4), None, // bottom face
            Some(7), Some(5), Some(1), None, Some(7), Some(3), Some(1), None, // left face
        ];

        Self {
            vertices: init_cube_size,
            indices: indice_pattern,
            color,
            thickness,
        }
    }

    pub fn mov (&mut self, direction: Vec3<i32>) {
        for v in self.vertices.iter_mut() {
            v.x += direction.x;
            v.y += direction.y;
            v.z += direction.z;
        }
    }
}

impl crate::graphics::Renderable for Cube {
    fn vertices(&self) -> &[Vec3<i32>] {
        self.vertices.as_slice()
    }

    fn indices(&self) -> &[Option<usize>] {
        self.indices.as_slice()
    }

    fn color(&self) -> color::Color { self.color }

    fn thickness(&self) -> f32 {self.thickness }
}