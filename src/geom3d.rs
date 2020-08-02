use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vect {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vect {
    pub fn dot(self, other: Vect) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Vect {
        Vect { x: x, y: y, z: z }
    }

    pub fn norm2(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> f64 {
        f64::sqrt(self.norm2())
    }

    pub fn unit(self) -> Vect {
        self / self.norm()
    }

    pub fn as_vec_u8(self) -> [u8; 3] {
        [self.x as u8, self.y as u8, self.z as u8]
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn to(self, other: Point) -> Vect {
        other - self
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x: x, y: y, z: z }
    }

    pub const ORIGIN: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

impl Add for Vect {
    type Output = Vect;

    fn add(self, other: Vect) -> Vect {
        Vect {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vect> for Point {
    type Output = Point;

    fn add(self, other: Vect) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vect {
    type Output = Vect;

    fn sub(self, other: Vect) -> Vect {
        Vect {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for Point {
    type Output = Vect;

    fn sub(self, other: Point) -> Vect {
        Vect {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vect> for f64 {
    type Output = Vect;

    fn mul(self, other: Vect) -> Vect {
        Vect {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Div<f64> for Vect {
    type Output = Vect;

    fn div(self, other: f64) -> Vect {
        Vect {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
