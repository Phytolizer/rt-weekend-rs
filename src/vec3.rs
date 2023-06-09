use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3(nalgebra::Vector3<f32>);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(nalgebra::Vector3::new(x, y, z))
    }

    pub fn zeros() -> Self {
        Self(nalgebra::Vector3::zeros())
    }

    pub fn near_zero(&self) -> bool {
        const s: f32 = 1e-8;
        self.0.x.abs() < s && self.0.y.abs() < s && self.0.z.abs() < s
    }

    pub fn normalize(&self) -> Self {
        self.0.normalize().into()
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2.0 * self.dot(normal) * *normal
    }

    pub fn refract(&self, normal: &Self, etai_over_etat: f32) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }
}

impl Deref for Vec3 {
    type Target = nalgebra::Vector3<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl From<nalgebra::Vector3<f32>> for Vec3 {
    fn from(v: nalgebra::Vector3<f32>) -> Self {
        Self(v)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
