use raytracer::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(&self, p: f32) -> Vec3 {
        self.origin + p * self.direction
    }
}
