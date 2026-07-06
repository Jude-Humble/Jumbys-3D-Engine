#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

use std::ops::Mul;
impl<T> Mul for Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

use std::ops::Add;
impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


#[derive(Debug)]
pub struct RotMat {
    mat: [[f32; 3]; 3],
}

impl RotMat {
    pub fn new(id: u8, theta: f32) -> Self {
        match id {
            0 => Self { mat: [
                [ 1.0 , 0.0         , 0.0          ],
                [ 0.0 , theta.cos() , -theta.sin() ],
                [ 0.0 , theta.sin() , theta.cos()  ],
            ]},
            1 => Self { mat: [
                [ theta.cos()  , 0.0 , theta.sin() ],
                [ 0.0          , 1.0 , 0.0         ],
                [ -theta.sin() , 0.0 , theta.cos() ],
            ]},
            2 => Self { mat: [
                [ theta.cos() , -theta.sin() , 0.0 ],
                [ theta.sin() , theta.cos()  , 0.0 ],
                [ 0.0         , 0.0          , 1.0 ],
            ]},
            _ => panic!("Unkown Rotation Matrix ID!"),
        }
    }

    pub fn multiply(&self, other: &mut Vec3<f32>) {
        *other = Vec3 {
            x: self.mat[0][0] * other.x + self.mat[0][1] * other.y + self.mat[0][2] * other.z,
            y: self.mat[1][0] * other.x + self.mat[1][1] * other.y + self.mat[1][2] * other.z,
            z: self.mat[2][0] * other.x + self.mat[2][1] * other.y + self.mat[2][2] * other.z,
        }
    }
}
