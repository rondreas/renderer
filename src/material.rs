use crate::ray::Ray;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::vector::*;

pub struct Lambertian {
    pub albedo: Color,
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
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

impl Material for Metal {
    fn scatter(&self, in_ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected: Vec3 = reflect(&unit_vector(&in_ray.direction), &hit.normal);
        let scattered = Ray{origin: hit.point, direction: reflected + self.fuzz*random_in_unit_sphere()};
        if dot(&reflected, &hit.normal) > 0.0 {
            return Some((self.albedo, scattered));
        }
        None
    }
}
