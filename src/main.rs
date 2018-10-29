extern crate rand;

use std::io::{BufWriter, Write};
use std::fs::OpenOptions;
use rand::Rng;

mod raytracer;
use raytracer::{Vec3, Hit, Ray, Rgb};
use raytracer::{Lambertian, Reflective, Sphere};

fn main() {
    let path = "/tmp/raytracer.ppm";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create/*_new*/(true)
        .open(&path);
    let file = match file {
        Ok(file) => file,
        Err(msg) => panic!("Could not open file: {}", msg),
    };
    let mut file = BufWriter::new(file);

    let width = 800;
    let height = 400;
    let n_aa_samples = 24;
    let cam = Camera::axis_aligned();
    let mut rng = rand::thread_rng();

    write!(file, "P3\n");
    write!(file, "{} {}\n", width, height);
    write!(file, "255\n");

    let world: Vec<Box<dyn Hit>> = vec![
        Box::new(Sphere {
            center: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
            radius: 0.5,
            material: Box::new(Lambertian {
                albedo: Vec3 { x: 0.8, y: 0.3, z: 0.3 }
            }),
        }),
        Box::new(Sphere {
            center: Vec3 { x: 0.0, y: -100.5, z: -1.0 },
            radius: 100.0,
            material: Box::new(Lambertian {
                albedo: Vec3 { x: 0.8, y: 0.8, z: 0.0 }
            }),
        }),
        Box::new(Sphere {
            center: Vec3 { x: 1.0, y: 0.0, z: -1.0 },
            radius: 0.5,
            material: Box::new(Reflective { albedo: Vec3 { x: 0.4, y: 0.6, z: 0.8 } }),
        }),
        Box::new(Sphere {
            center: Vec3 { x: -1.0, y: 0.0, z: -1.0 },
            radius: 0.5,
            material: Box::new(Reflective { albedo: Vec3 { x: 0.8, y: 0.8, z: 0.8 } }),
        }),
    ];

    for j in (0..height).rev() {
        for i in 0..width {
            // Anti-aliasing.
            let mut col = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
            for _ in 0..n_aa_samples {
                let u = (i as f32 + rng.gen::<f32>()) / width as f32;
                let v = (j as f32 + rng.gen::<f32>()) / height as f32;
                let ray = cam.ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= n_aa_samples as f32;
            let col = Rgb {
                r: 255.99 * col.x.sqrt(),
                g: 255.99 * col.y.sqrt(),
                b: 255.99 * col.z.sqrt(),
            };

            write!(file, "{} {} {}\n", col.r as i32, col.g as i32, col.b as i32);
        }
    }
}

struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    fn axis_aligned() -> Camera {
        Camera {
            lower_left_corner: Vec3 { x: -2.0, y: -1.0, z: -1.0 },
            horizontal: Vec3 { x: 4.0, y: 0.0, z: 0.0 },
            vertical: Vec3 { x: 0.0, y: 2.0, z: 0.0 },
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        }
    }

    fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner
                + u * self.horizontal
                + v * self.vertical
                - self.origin,
        }
    }
}

fn color<T: Hit>(ray: &Ray, world: &T, depth: i32) -> Vec3 {
    // See if the ray hits the world, otherwise paint the background.
    if let Some(hit) = world.hit(&ray, 0.001, std::f32::MAX) {
        let neutral = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        if depth >= 50 {
            neutral
        } else if let Some(scatter) = hit.material.scatter(ray, hit.point, hit.normal) {
            scatter.attenuation * color(&scatter.ray, world, depth + 1)
        } else {
            neutral
        }
    } else {
        // Get unit vector of ray's direction so -1 < y < 1 and scale that value
        // to 0 < y < 1.
        let t = 0.5 * (ray.direction.to_unit().y + 1.0);
        // Linear interpolation: blended_val = (1 - t) * start_val + t * end_val.
        let start_val = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        let end_val = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
        let ler = (1.0 - t) * start_val + t * end_val;
        ler
    }
}
