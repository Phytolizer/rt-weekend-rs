use itertools::Itertools;
use rand::Rng;

use crate::random_vec_range;
use crate::vec3::Vec3;
use crate::Point3;

pub struct Perlin {
    ranvec: Box<[Vec3]>,
    perm_x: Box<[isize]>,
    perm_y: Box<[isize]>,
    perm_z: Box<[isize]>,
}

const point_count: usize = 256;

fn perlin_generate_perm() -> Box<[isize]> {
    let mut p = (0..point_count).map(|i| i as isize).collect_vec();

    permute(p.as_mut());

    p.into_boxed_slice()
}

fn permute(p: &mut [isize]) {
    for i in (1..p.len()).rev() {
        let target = rand::thread_rng().gen_range(0..i);
        p.swap(i, target);
    }
}

fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in 0isize..2 {
        for j in 0isize..2 {
            for k in 0isize..2 {
                let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + ((1 - i) as f64) * (1.0 - uu))
                    * (j as f64 * vv + ((1 - j) as f64) * (1.0 - vv))
                    * (k as f64 * ww + ((1 - k) as f64) * (1.0 - ww))
                    * c[i as usize][j as usize][k as usize].dot(&weight_v);
            }
        }
    }
    accum
}

impl Perlin {
    pub fn new() -> Self {
        let ranvec = (0..point_count)
            .map(|_| random_vec_range(-1.0, 1.0))
            .collect_vec()
            .into_boxed_slice();
        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();
        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;

        let mut c = [[[Vec3::zeros(); 2]; 2]; 2];
        for di in 0isize..2 {
            for dj in 0isize..2 {
                for dk in 0isize..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }

        trilinear_interp(&c, u, v, w)
    }
}
