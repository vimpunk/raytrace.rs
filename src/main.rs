extern crate rand;
extern crate image;

mod raytracer;

use std::io::{BufWriter, Write};
use std::fs::OpenOptions;
use rand::Rng;
use raytracer::{Camera, CameraInfo, Hit, Ray, Rgb, Vec3};
use raytracer::{Dielectric, Lambertian, Reflective, Sphere};

fn main() {
    let mut rng = rand::thread_rng();

    let width = 1200;
    let height = 600;
    let n_aa_samples = 24;

    let look_from = Vec3 { x: 13.0, y: 2.0, z: 3.0 };
    let look_at = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let view_up = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    let cam = Camera::new(CameraInfo {
        look_from: look_from,
        look_at: look_at,
        view_up: view_up,
        vert_fov: 20 as f32,
        aspect: width as f32 / height as f32,
        aperture: 0.1,
        focus_distance: 10.0,
    });
    //let cam = Camera::axis_aligned();

    //let world = basic_scene();
    let world = rand_scene();

    let mut pixels = Vec::with_capacity(width * height);
    // Reverse iteration over y coordinates so that image is written top to
    // bottom, left to right.
    for y in (0..height).rev() {
        for x in 0..width {
            // Anti-aliasing
            let mut col = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
            for _ in 0..n_aa_samples {
                let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                let v = (y as f32 + rng.gen::<f32>()) / height as f32;
                let ray = cam.ray(u, v);
                col += compute_color(&ray, &world, 0);
            }
            col /= n_aa_samples as f32;
            let col = Rgb::from(col).gamma_correct();
            let col = Rgb::from(Vec3::from(col) * 255.99);
            pixels.push(col);
        }
    }

    save_ppm(width, height, &pixels);
    save_png(width, height, &pixels);
}

fn save_ppm(width: usize, height: usize, pixels: &Vec<Rgb>) {
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

    write!(file, "P3\n");
    write!(file, "{} {}\n", width, height);
    write!(file, "255\n");

    for pixel in pixels.iter() {
        write!(file, "{} {} {}\n", pixel.r as i32, pixel.g as i32, pixel.b as i32);
    }
}

fn save_png(width: usize, height: usize, pixels: &Vec<Rgb>) {
    let mut img = image::RgbImage::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let idx = x + y * width;
            let pixel = pixels[idx];
            let pixel = image::Rgb::<u8> {
                data: [pixel.r as u8, pixel.g as u8, pixel.b as u8]
            };
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }
    img.save("/tmp/raytracing_weekend.png").unwrap();
}

fn compute_color<T: Hit>(ray: &Ray, world: &T, depth: i32) -> Vec3 {
    // See if the ray hits the world, otherwise paint the background.
    if let Some(hit) = world.hit(&ray, 0.001, std::f32::MAX) {
        let neutral = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        if depth >= 50 {
            neutral
        } else if let Some(scatter) = hit.material.scatter(ray, hit.point, hit.normal) {
            scatter.attenuation * compute_color(&scatter.ray, world, depth + 1)
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

fn basic_scene() -> Vec<Box<dyn Hit>> {
    vec![
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
    ]
}

fn rand_scene() -> Vec<Box<dyn Hit>> {
    let n = 500;
    let mut world: Vec<Box<dyn Hit>> = Vec::with_capacity(n);
    world.push(Box::new(Sphere {
        center: Vec3 { x: 0.0, y: -1000.0, z: 0.0 },
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Vec3 { x: 0.5, y: 0.5, z: 0.5 },
        }),
    }));
    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3 {
                x: a as f32 + 0.9 * rng.gen::<f32>(),
                y: 0.2,
                z: b as f32 + 0.9 * rng.gen::<f32>(),
            };
            if (center - Vec3 { x: 4.0, y: 0.2, z: 0.0 }).len() > 0.9 {
                let r = rng.gen::<f32>();
                if r < 0.8 {
                    world.push(Box::new(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Vec3 {
                                x: rng.gen::<f32>() * rng.gen::<f32>(),
                                y: rng.gen::<f32>() * rng.gen::<f32>(),
                                z: rng.gen::<f32>() * rng.gen::<f32>(),
                            },
                        }),
                    }));
                } else if r < 0.95 {
                    world.push(Box::new(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(Reflective {
                            albedo: Vec3 {
                                x: 0.5 * (1.0 + rng.gen::<f32>()),
                                y: 0.5 * (1.0 + rng.gen::<f32>()),
                                z: 0.5 * (1.0 + rng.gen::<f32>()),
                            },
                            fuzz: rng.gen::<f32>() * rng.gen::<f32>(),
                        }),
                    }));
                } else {
                    world.push(Box::new(Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(Dielectric { refraction_index: 1.5 }),
                    }));
                }
            }
        }
    }

    world.push(Box::new(Sphere {
        center: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Box::new(Dielectric { refraction_index: 1.5 }),
    }));
    world.push(Box::new(Sphere {
        center: Vec3 { x: -4.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Box::new(Lambertian {
            albedo: Vec3 { x: 0.4, y: 0.2, z: 0.1 },
        }),
    }));
    world.push(Box::new(Sphere {
        center: Vec3 { x: 4.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Box::new(Reflective {
            albedo: Vec3 {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            fuzz: 0.0,
        }),
    }));

    world
}
