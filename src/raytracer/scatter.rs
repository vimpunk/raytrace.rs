extern crate rand;

use raytracer::ray::*;
use raytracer::vec3::*;
use rand::Rng;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub ray: Ray,
}

pub trait Scatter {
    fn scatter(&self, ray: &Ray, point: Vec3, normal: Vec3) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Scatter for Lambertian {
    fn scatter(&self, _: &Ray, point: Vec3, normal: Vec3) -> Option<ScatterRecord> {
        let target = point + normal + rand_point_in_unit_sphere();
        Some(ScatterRecord {
            attenuation: self.albedo,
            ray: Ray {
                origin: point,
                direction: target - point,
            },
        })
    }
}

fn rand_point_in_unit_sphere() -> Vec3 {
    // TODO optimize this
    let mut rng = rand::thread_rng();
    loop {
        let v = 2.0 * Vec3 { x: rng.gen(), y: rng.gen(), z: rng.gen(), }
            - Vec3 { x: 1.0, y: 1.0, z: 1.0 };

        if v.squared_len() >= 1.0 {
            return v;
        }
    }
}

pub struct Reflective {
    pub albedo: Vec3,
}

impl Scatter for Reflective {
    fn scatter(&self, ray: &Ray, point: Vec3, normal: Vec3) -> Option<ScatterRecord> {
        let reflected = reflect(ray.direction.to_unit(), normal);
        if dot(reflected, normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                ray: Ray {
                    origin: point,
                    direction: reflected,
                },
            })
        } else {
            None
        }
    }
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * dot(v, normal) * normal
}
