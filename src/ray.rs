use crate::vector::Vec3;


#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }
}
