extern crate rand;

use raytracer::vec3::*;
use raytracer::ray::Ray;
use rand::Rng;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    /// The orthonormal basis of the camera. `z0` is simply the unit vector of
    /// the difference of `look_at` and `look_from`, defining the axis of the
    /// view. `x0` is the unit vector of the cross product of
    /// `CameraInfo::view_up` and `z0`, and `y0` is the cross product of `x0`
    /// and `z0`.
    x0: Vec3,
    y0: Vec3,
    //z0: Vec3,
    /// The larger the lens, the more defocus blur there will be around the
    /// focus area, which subsequently will be smaller.
    lens_radius: f32,
}

pub struct CameraInfo {
    /// The origin of the camera.
    pub look_from: Vec3,
    /// The point at which the camera is looking.
    pub look_at: Vec3,
    /// A vector describing the vertical component of the scene from which the
    /// camera's plane's orthonormal basis are calculated.
    pub view_up: Vec3,
    /// The vertical field of view, top to bottom in degrees.
    pub vert_fov: f32,
    /// The ratio of width to height.
    pub aspect: f32,
    /// The smaller the aperture, the less defocus blur there is. A value of
    /// 0.0 turns it off completely.
    pub aperture: f32,
    /// The distance from the origin that describes the focused point.
    pub focus_distance: f32,
}

impl Camera {
    pub fn axis_aligned() -> Camera {
        Camera {
            lower_left_corner: Vec3 { x: -2.0, y: -1.0, z: -1.0 },
            horizontal: Vec3 { x: 4.0, y: 0.0, z: 0.0 },
            vertical: Vec3 { x: 0.0, y: 2.0, z: 0.0 },
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            x0: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
            y0: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
            //z0: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
            lens_radius: 0.0,
        }
    }

    pub fn new(info: CameraInfo) -> Camera {
        // y
        // ^  /|
        // | / | h
        // |/  |
        // |θ--> -z
        // |\  |
        // | \ |
        // |  \|
        //
        // h = tan(θ/2)
        let theta = info.vert_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = info.aspect * half_height;
        let z0 = (info.look_from - info.look_at).to_unit();
        let x0 = cross(info.view_up, z0).to_unit();
        let y0 = cross(z0, x0);
        Camera {
            origin: info.look_from,
            // Subtract from the camera origin each component of the camera's
            // basis multiplied by the canvas dimensions and the focus distance.
            lower_left_corner: info.look_from
                - x0 * info.focus_distance * half_width
                - y0 * info.focus_distance * half_height
                - z0 * info.focus_distance,
            // Adjust the horizontal and verticla components of our camera by
            // scaling the corresponding bases with the canvas dimensions and
            // the focus distance.
            horizontal: x0 * info.focus_distance * 2.0 * half_width,
            vertical: y0 * info.focus_distance * 2.0 * half_height,
            x0: x0,
            y0: y0,
            //z0: z0,
            lens_radius: info.aperture / 2.0,
        }
    }

    pub fn ray(&self, h: f32, v: f32) -> Ray {
        // Offset ray's origin so that it's on a disk around `look_from`.
        let rd = self.lens_radius * rand_in_unit_disk();
        let offset = self.x0 * rd.x + self.y0 * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner
                + h * self.horizontal
                + v * self.vertical
                - self.origin - offset,
        }
    }
}

fn rand_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3 { x: rng.gen(), y: rng.gen(), z: 0.0 }
            - Vec3 { x: 1.0, y: 1.0, z: 0.0 };
        if dot(p, p) < 1.0 {
            return p;
        }
    }
}
