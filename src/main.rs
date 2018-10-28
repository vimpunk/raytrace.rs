use std::io::Write;
use std::fs::OpenOptions;

mod raytracer;
use raytracer::{Vec3, dot, Hit, Ray, Rgb, Sphere};

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
                direction: lower_left_corner + u * horizontal + v * vertical,
            };

            let color = color(&ray);
            let color = Rgb {
                r: 255.99 * color.r,
                g: 255.99 * color.g,
                b: 255.99 * color.b,
            };

            write!(file, "{} {} {}\n", color.r as i32, color.g as i32, color.b as i32);
        }
    }
}

fn color(ray: &Ray) -> Rgb {
    let sphere = Sphere {
        center: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        radius: 0.5,
    };
    if let Some(hit) = sphere.hit(&ray, 0.0, 2.0) {
        let normal = (ray.point_at(hit.t) - sphere.center).to_unit();
        let normal = 0.5 * Vec3 {
            x: normal.x + 1.0,
            y: normal.y + 1.0,
            z: normal.z + 1.0
        };
        return Rgb::from(normal);
    }

    // Get unit vector so -1 < y < 1.
    let unit_dir = ray.direction.to_unit();
    // Scale that value to 0 < y < 1.
    let t = 0.5 * (unit_dir.y + 1.0);
    // Linear interpolation: blended_val = (1 - t) * start_val + t * end_val.
    let start_val = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
    let end_val = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
    let ler = (1.0 - t) * start_val + t * end_val;
    Rgb::from(ler)
}
