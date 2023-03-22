use std::ops::Index;

use crate::vec3::Vec3;

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn from_w(n: &Vec3) -> Self {
        let mut s = Self {
            axis: [Vec3::zeros(); 3],
        };
        s.axis[2] = n.normalize();
        let a = if s.w().x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        s.axis[1] = s.w().cross(&a).normalize().into();
        s.axis[0] = s.w().cross(&s.v()).into();
        s
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        a.x * self.u() + a.y * self.v() + a.z * self.w()
    }
}

impl Index<usize> for Onb {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}
