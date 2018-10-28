use std::io::Write;
use std::fs::OpenOptions;

mod raytracer;
use raytracer::{Vec3, unit_vec, Ray, Rgb};

fn main() {
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
    let lower_left_corner = Vec3 { x: -2.0, y: -1.0, z: -1.0 };
    let horizontal = Vec3 { x: 4.0, y: 0.0, z: 0.0 };
    let vertical = Vec3 { x: 0.0, y: 2.0, z: 0.0 };
    let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    write!(file, "P3\n");
    write!(file, "{} {}\n", nx, ny);
    write!(file, "255\n");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let ray = Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical
            };

            let color = color(&ray);
            let color = Rgb {
                r: 255.99 * color.r,
                g: 255.99 * color.g,
                b: 255.99 * color.b,
            };

            //let r = (255.99 * color.r) as i32;
            //let g = (255.99 * color.g) as i32;
            //let b = (255.99 * color.b) as i32;

            write!(file, "{} {} {}\n", color.r as i32, color.g as i32, color.b as i32);
        }
    }
}

fn color(ray: &Ray) -> Rgb {
    // Get unit vector so -1 < y < 1.
    let unit_dir = unit_vec(ray.direction);
    // Scale that value to 0 < y < 1.
    let t = 0.5 * (unit_dir.y + 1.0);
    // Linear interpolation: blended_val = (1 - t) * start_val + t * end_val.
    let ler = (1.0 - t) * Vec3 { x: 1.0, y: 1.0, z: 1.0 } + t * Vec3 { x: 0.5, y: 0.7, z: 1.0 };
    Rgb {
        r: ler.x,
        g: ler.y,
        b: ler.z,
    }
}
