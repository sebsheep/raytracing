use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vect {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vect {
    pub const NULL: Vect = Vect::new(0.0, 0.0, 0.0);
    pub fn dot(self, other: Vect) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vect) -> Vect {
        Vect {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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

    pub fn unit(self) -> Unit {
        Unit(self / self.norm())
    }

    pub fn as_vec_u8(self) -> [u8; 3] {
        [self.x as u8, self.y as u8, self.z as u8]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Unit(Vect);

impl Unit {
    pub fn to_vect(&self) -> Vect {
        self.0
    }

    pub fn cross(self, other: Unit) -> Unit {
        self.to_vect().cross(other.to_vect()).unit()
    }

    pub fn dot(self, other: Unit) -> f64 {
        self.to_vect().dot(other.to_vect())
    }
}

/// Returns the vector v reflected in respect to the
/// first coordinate of the (e1, e2, e3) basis.
///
/// (e1, e2, e3) is supposed to be an orthonormal basis
pub fn x_reflexion(v: Unit, (e1, e2, e3): (Unit, Unit, Unit)) -> Unit {
    /* (x_, y_, z_) are the coordinates of v in the
       (e1, e2, e3) basis
    */
    let x_ = v.to_vect().dot(e1.to_vect());
    let y_ = v.to_vect().dot(e2.to_vect());
    let z_ = v.to_vect().dot(e3.to_vect());

    /* Converting back to the canonical basis, reverting
       the forst coordinate (hence the - sign).
    */
    Unit((-x_ * e1.to_vect()) + y_ * e2.to_vect() + z_ * e3.to_vect())
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::geom3d::*;

    #[test]
    fn unit_test() {
        let e1 = Vect::new(1.0, 0.0, 0.0);
        let e2 = Vect::new(0.0, 1.0, 0.0);
        let e3 = Vect::new(0.0, 0.0, 1.0);
        assert_eq!(e1, e1.unit().to_vect());
        assert_eq!(e2, e2.unit().to_vect());
        assert_eq!(e3, e3.unit().to_vect());
    }

    #[test]
    fn x_reflexion_test() {
        let ray = Vect::new(1.0, 0.0, -1.0).unit();
        let e1 = Vect::new(1.0, 0.0, 0.0).unit();
        let e2 = Vect::new(0.0, 1.0, 0.0).unit();
        let e3 = Vect::new(0.0, 0.0, 1.0).unit();
        let x_ref = x_reflexion(ray, (e1, e2, e3));
        assert_eq!(x_ref, Vect::new(-1.0, 0.0, -1.0).unit());
    }
}
