use crate::vector::Vec3;


#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub trait At {
    fn at(&self, t: f32) -> Vec3;
}

impl At for Ray {
    fn at(&self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }
}
