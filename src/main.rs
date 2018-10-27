use std::io::Write;
use std::fs::OpenOptions;

mod raytracer;
use raytracer::{Vec3, unit_vec, Ray, Rgb};

fn main() {
    let vec = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
    let ray = Ray { origin: vec, direction: vec };
    let path = "/tmp/raytracer.ppm";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create/*_new*/(true)
        .open(&path);
    let mut file = match file {
        Ok(file) => file,
        Err(msg) => panic!("Could not open file: {}", msg),
    };

    let nx = 200;
    let ny = 100;

    write!(file, "P3\n");
    write!(file, "{} {}\n", nx, ny);
    write!(file, "255\n");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2;

            let r = (255.99 * r) as i32;
            let g = (255.99 * g) as i32;
            let b = (255.99 * b) as i32;

            write!(file, "{} {} {}\n", r, g, b);
        }
    }
}

fn color(ray: &Ray) -> Rgb {
    let unit_dir = unit_vec(ray.direction);
    let t = 0.5 * (unit_dir.y + 1.0);
    let vec = (1.0 - t) * Vec3 { x: 1.0, y: 1.0, z: 1.0 } + t * Vec3 { x: 0.5, y: 0.7, z: 1.0 };
    Rgb {
        r: vec.x,
        g: vec.y,
        b: vec.z,
    }
}
