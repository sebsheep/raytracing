use std::ops::{Add,Sub,Mul,Div};

#[derive(Copy, Clone)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3d {
    pub fn dot(self, other: Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x: x, y: y, z: z }
    }

    pub fn norm2(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> f64 {
        f64::sqrt(self.norm2())
    }

    pub fn unit(self) -> Vec3d {
        self / self.norm()
    }

    pub fn as_vec_u8(self) -> [u8; 3] {
        [ self.x as u8, self.y as u8, self.z as u8]
    }
}

impl Add for Vec3d {
    type Output = Vec3d;

    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}


impl Mul<Vec3d> for f64 {
    type Output = Vec3d;

    fn mul(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self * other.x, y: self * other.y, z: self * other.z }
    }
}

impl Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, other: f64) -> Vec3d {
        Vec3d { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}