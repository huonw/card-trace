use std::num;
pub struct Vec {
    x: f32,
    y: f32,
    z: f32
}
impl Add<Vec, Vec> for Vec {
    fn add(&self, other: &Vec) -> Vec {
        Vec::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl Mul<f32, Vec> for Vec {
    fn mul(&self, &scale: &f32) -> Vec {
        Vec::new(self.x * scale, self.y * scale, self.z * scale)
    }
}
impl num::Zero for Vec {
    fn zero() -> Vec {
        Vec::new(0., 0., 0.)
    }
    fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0. && self.z == 0.
    }
}

impl Vec {
    pub fn new(x: f32, y: f32, z: f32) -> Vec {
        Vec { x: x, y: y, z: z }
    }
    pub fn dot(&self, other: &Vec) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vec) -> Vec {
        Vec::new(self.y * other.z - self.z * other.y,
                 self.z * other.x - self.x * other.z,
                 self.x * other.y - self.y * other.x)
    }
    pub fn normalise(&self) -> Vec {
        *self * (1. / num::sqrt(self.dot(self)))
    }
}
