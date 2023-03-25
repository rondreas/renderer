use crate::hittable::*;
use crate::ray::*;
use crate::vector::*;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temporary_record = HitRecord {
            point: Vec3::zero(),
            normal: Vec3::zero(),
            t: t_max,
            front_face: true,
        };
        let mut closest_so_far = t_max;
        let mut hit_anything = false;
        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = true;
                temporary_record = hit;
            }
        }

        if hit_anything {
            return Some(temporary_record);
        }

        None
    }
}
