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

        if v.squared_len() < 1.0 {
            return v;
        }
    }
}

pub struct Reflective {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Scatter for Reflective {
    fn scatter(&self, ray: &Ray, point: Vec3, normal: Vec3) -> Option<ScatterRecord> {
        let reflected = reflect(ray.direction.to_unit(), normal);
        if dot(reflected, normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                ray: Ray {
                    origin: point,
                    direction: reflected + self.fuzz * rand_point_in_unit_sphere(),
                }
            })
        } else {
            None
        }
    }
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    // v  n   r
    // \  |  /|
    //  \ | / | b
    //   \|/  |
    // --------
    //    \   |
    //     \  | b
    //      \ |
    //       v
    //
    // |b| = dot(v, n)
    // Scale n to 2b and subtract from v to get r.
    v - 2.0 * dot(v, normal) * normal
}

pub struct Dielectric {
    pub refraction_index: f32,
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: &Ray, point: Vec3, normal: Vec3) -> Option<ScatterRecord> {
        let (outward_normal, ni_over_nt, cos) = {
            // Determine whether the ray is in the refractive object and take
            // the opposite of the surface normal if so.
            if dot(ray.direction, normal) > 0.0 {
                let outward_normal = -1.0 * normal;
                let ni_over_nt = self.refraction_index;
                let cos = self.refraction_index * dot(ray.direction, normal) / ray.direction.len();
                (outward_normal, ni_over_nt, cos)
            } else {
                let outward_normal = normal;
                let ni_over_nt = 1.0 / self.refraction_index;
                let cos = -dot(ray.direction, normal) / ray.direction.len();
                (outward_normal, ni_over_nt, cos)
            }
        };
        let attenuation = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

        if let Some(refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
            let scattered = {
                let reflection_prob = schlick(cos, self.refraction_index);
                if rand::thread_rng().gen::<f32>() >= reflection_prob {
                    refracted
                } else {
                    reflect(ray.direction, normal)
                }
            };
            Some(ScatterRecord {
                attenuation: attenuation,
                ray: Ray {
                    origin: point,
                    direction: scattered,
                },
            })
        } else {
            Some(ScatterRecord {
                attenuation: attenuation,
                ray: Ray {
                    origin: point,
                    direction: reflect(ray.direction, normal),
                },
            })
        }
    }
}

fn refract(v: Vec3, normal: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let unit_v = v.to_unit();
    let dt = dot(unit_v, normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (unit_v - normal * dt) - normal * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cos: f32, refraction_index: f32) -> f32 {
    let r = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r = r * r;
    r + (1.0 - r) * (1.0 - cos).powi(5)
}
