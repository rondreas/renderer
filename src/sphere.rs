use crate::vector::*;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_squared = discriminant.sqrt();

        let mut root = (-half_b - discriminant_squared) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant_squared) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let mut rec = HitRecord{
            point: point,
            normal: point - self.center,
            t: root,
            front_face: true};
        let outward_normal = (point - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        return Some(rec);
    }
}
