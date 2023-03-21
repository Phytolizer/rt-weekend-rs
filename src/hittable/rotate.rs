use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Point3;

use super::HitRecord;
use super::Hittable;

pub struct RotateY {
    ptr: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(ptr: Box<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = ptr.bounding_box(0.0, 1.0);

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        let bbmax = bbox.as_ref().map(|bb| bb.max).unwrap_or(Vec3::zeros());
        let bbmin = bbox.as_ref().map(|bb| bb.min).unwrap_or(Vec3::zeros());
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbmax.x + (1 - i) as f64 * bbmin.x;
                    let y = j as f64 * bbmax.y + (1 - j) as f64 * bbmin.y;
                    let z = k as f64 * bbmax.z + (1 - k) as f64 * bbmin.z;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            bbox,
            ptr,
            sin_theta,
            cos_theta,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_r = Ray::new(origin, direction, ray.time);

        let rec = self.ptr.hit(&rotated_r, t_min, t_max)?;

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        Some(HitRecord::new(
            p,
            normal,
            rec.t,
            rec.u,
            rec.v,
            &rotated_r,
            rec.mat_ptr,
        ))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox.clone()
    }
}
