extern crate rand;

mod raytracer;

use std::io::{BufWriter, Write};
use std::fs::OpenOptions;
use rand::Rng;
use raytracer::{Camera, CameraInfo, Hit, Ray, Rgb, Vec3};
use raytracer::{Dielectric, Lambertian, Reflective, Sphere};

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

    let mut rng = rand::thread_rng();

    let width = 800;
    let height = 400;
    let n_aa_samples = 24;

    let look_from = Vec3 { x: -1.5, y: 1.0, z: 1.0 };
    let look_at = Vec3 { x: 0.0, y: 0.0, z: -1.0 };
    let view_up = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    let aperture = 0.1;
    let focus_dist = (look_from - look_at).len();
    let cam = Camera::new(CameraInfo {
        look_from: look_from,
        look_at: look_at,
        view_up: view_up,
        vert_fov: 40 as f32,
        aspect: width as f32 / height as f32,
        aperture: aperture,
        focus_distance: focus_dist,
    });
    let cam = Camera::axis_aligned();

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
                albedo: Vec3 { x: 0.5, y: 0.5, z: 0.5 }
            }),
        }),
        Box::new(Sphere {
            center: Vec3 { x: 1.0, y: 0.0, z: -1.0 },
            radius: 0.5,
            material: Box::new(Reflective {
                albedo: Vec3 { x: 0.4, y: 0.6, z: 0.8 },
                fuzz: 0.9,
            }),
        }),
        //Box::new(Sphere {
            //center: Vec3 { x: -1.0, y: 0.0, z: -1.0 },
            //radius: 0.5,
            //material: Box::new(Dielectric { refraction_index: 1.5 }),
        //}),
        Box::new(Sphere {
            center: Vec3 { x: -1.0, y: 0.0, z: -1.0 },
            radius: 0.5,
            material: Box::new(Reflective {
                albedo: Vec3 { x: 0.8, y: 0.8, z: 0.8 },
                fuzz: 0.3,
            }),
        }),
    ];

    for y in (0..height).rev() {
        for x in 0..width {
            // Anti-aliasing.
            let mut col = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
            for _ in 0..n_aa_samples {
                let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                let v = (y as f32 + rng.gen::<f32>()) / height as f32;
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
