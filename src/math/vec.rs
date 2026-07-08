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

use std::ops::Sub;
impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
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

#[derive(Copy, Clone)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w * rhs.w
                - self.x * rhs.x
                - self.y * rhs.y
                - self.z * rhs.z,

            x: self.w * rhs.x
                + self.x * rhs.w
                + self.y * rhs.z
                - self.z * rhs.y,

            y: self.w * rhs.y
                - self.x * rhs.z
                + self.y * rhs.w
                + self.z * rhs.x,

            z: self.w * rhs.z
                + self.x * rhs.y
                - self.y * rhs.x
                + self.z * rhs.w,
        }
    }
}

impl Quaternion {
    pub fn new (x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn conjugate (&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn rotation_quaternion (&self, t: f32) -> Self {
        Self {
            x: self.x * ( t / 2.0 ).sin(),
            y: self.y * ( t / 2.0 ).sin(),
            z: self.z * ( t / 2.0 ).sin(),
            w: ( t / 2.0 ).cos(),
        }
    }

    pub fn rotate(v: Vec3<f32>, t: f32, id: u8) -> Vec3<f32> {
        let mut q: Quaternion;
        match id {
            0 => q = Self::new(1.0, 0.0, 0.0, 0.0).rotation_quaternion(t),
            1 => q = Self::new(0.0, 1.0, 0.0, 0.0).rotation_quaternion(t),
            2 => q = Self::new(0.0, 0.0, 1.0, 0.0).rotation_quaternion(t),
            _ => panic!("Err: Unknown ID provided for quaternion rotation!"),
        }
        let p = Self::new(v.x, v.y, v.z, 0.0);

        let rotated = q.conjugate() * p * q;

        Vec3 {
            x: rotated.x,
            y: rotated.y,
            z: rotated.z,
        }
    } 
}
