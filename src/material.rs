use crate::ray::Ray;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::vector::{Vec3, random_unit_vector, dot};

pub struct Lambertian {
    pub albedo: Color,
}

pub struct Metall {
    pub albedo: Color,
}

pub struct Dielectrics {}

pub trait Material {
    fn scatter(&self, in_ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>; 
}

impl Material for Lambertian {
    fn scatter(&self, in_ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let mut scattered_direction: Vec3 = hit.normal + random_unit_vector();
        if scattered_direction.near_zero() {
            scattered_direction = hit.normal;
        }
        let scattered = Ray{origin: hit.point, direction: scattered_direction};
        return Some((self.albedo, scattered));
    }
}

impl Material for Metall {
    fn scatter(&self, in_ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected: Vec3 = hit.normal + random_unit_vector();
        let scattered = Ray{origin: hit.point, direction: reflected};
        if dot(&reflected, &hit.normal) < 0.0 {
            return Some((self.albedo, scattered));
        }
        None
    }
}
