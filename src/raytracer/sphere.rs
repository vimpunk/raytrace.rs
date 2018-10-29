use raytracer::hit::*;
use raytracer::ray::*;
use raytracer::scatter::*;
use raytracer::vec3::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Scatter>,
}

impl Hit for Sphere {
    fn hit<'a, 'b: 'a>(
        &'b self,
        ray: &Ray,
        min: f32,
        max: f32,
    ) -> Option<HitRecord<'a>> {
        // t^2*dot(B, B) + 2t*dot(B, A-C) + dot(A-C, A-C) - R^2 = 0 where:
        // A = ray origin, B = ray direction, C = sphere center, R = sphere
        // radius
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let b = 2.0 * dot(ray.direction, oc);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            // Solve the quadratic equation, which gives us the `t` offset with
            // which when the ray is advanced, it will hit the sphere.
            let solution = (-b - discriminant.sqrt()) / (2.0 * a);
            if solution > min && solution < max {
                let point = ray.point_at(solution);
                return Some(HitRecord {
                    t: solution,
                    point: point,
                    normal: (point - self.center).to_unit(),
                    material: &*self.material,
                });
            }

            let solution = (-b + discriminant.sqrt()) / (2.0 * a);
            if solution > min && solution < max {
                let point = ray.point_at(solution);
                return Some(HitRecord {
                    t: solution,
                    point: point,
                    normal: (point - self.center).to_unit(),
                    material: &*self.material,
                });
            }
        }

        None
    }
}
