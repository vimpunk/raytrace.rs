use raytracer::rgb::Rgb;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Index};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn from(rgb: Rgb) -> Vec3 {
        Vec3 {
            x: rgb.r,
            y: rgb.g,
            z: rgb.b,
        }
    }

    pub fn len(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn to_unit(&self) -> Vec3 {
        let k = 1.0 / self.len();
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.y;
        self.y *= other.z;
        self.z *= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        if i == 0 {
            &self.x
        } else if i == 1 {
            &self.y
        } else if i == 2 {
            &self.z
        } else {
            panic!("Invalid index for Vec3");
        }
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: -(a.x * b.z - a.z * b.x),
        z: a.x * b.y - a.y * b.x
    }
}
