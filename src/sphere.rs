use crate::maths::*;
use crate::ray::*;

pub struct Sphere {
    pub position: Vector,
    pub radius: f64
}

impl Sphere {
    pub fn intersect(&self, ray: Ray) -> Option<(f64, f64)> {
        let sphere_to_ray = ray.origin - self.position;

        let a = ray.direction * ray.direction;
        let b = ray.direction * sphere_to_ray * 2.0;
        let c = sphere_to_ray * sphere_to_ray - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        return Some((t1, t2));
    }
}
