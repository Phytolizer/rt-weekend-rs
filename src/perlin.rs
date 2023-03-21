use itertools::Itertools;
use rand::Rng;

use crate::Point3;

pub struct Perlin {
    ran_float: Box<[f64]>,
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

impl Perlin {
    pub fn new() -> Self {
        let ran_float = (0..point_count)
            .map(|_| rand::thread_rng().gen_range(0.0..1.0))
            .collect_vec()
            .into_boxed_slice();
        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();
        Self {
            ran_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = (4.0 * p.x) as isize & 255;
        let j = (4.0 * p.y) as isize & 255;
        let k = (4.0 * p.z) as isize & 255;

        self.ran_float
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}
