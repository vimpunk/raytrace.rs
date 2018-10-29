use raytracer::ray::Ray;
use raytracer::vec3::Vec3;

/// Describes how a ray hit an object (implementing the `Hit` trait).
pub struct HitRecord {
    /// The offset parameter with which to advance ray to get the point at which
    /// it hits the object.
    pub t: f32,
    /// The point at which ray hits the object.
    pub p: Vec3,
    /// The surface normal.
    pub normal: Vec3,
}

/// A trait that implementors can use to describe how a `Ray` may hit them.
pub trait Hit {
    /// Calculates whether the ray hits this object. For convenience, a valid
    /// hit interval range may also be added with with the `min` and `max`
    /// parameters, wich can be used to limit the area that counts as a hit.
    /// If this object is hit by the ray, the details about the hit are
    /// stored in a `HitRecord`.
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<HitRecord>;
}

/// Hit trait for a list of 'Hit' objects.
impl Hit for Vec<Box<Hit>> {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<HitRecord> {
        let mut record = None;
        let mut closest = max;
        for hitable in self.iter() {
            if let Some(rec) = hitable.hit(ray, min, closest) {
                closest = rec.t;
                record = Some(rec);
            }
        }
        record
    }
}
