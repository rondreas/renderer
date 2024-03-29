use crate::ray::Ray;
use crate::vector::{Vec3, dot};
use crate::material::Material;
use std::rc::Rc;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
