use crate::{Point3, Vec3, common};

/// Perlin noise, it takes a 3D point as input and always returns
/// the same randomish number. Nearby points return similar numbers.
#[derive(Debug, Clone)]
pub struct Perlin {
    rand_vecs: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

const POINT_COUNT: usize = 256;

impl Perlin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        // Note: 这里的 p.x 是 f64，数值有正有负，不能转为 u64，
        // 否则会丢失数据得到异常的图像
        let i = p.x.floor() as i64;
        let j = p.y.floor() as i64;
        let k = p.z.floor() as i64;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        (0_usize..2).for_each(|di| {
            (0_usize..2).for_each(|dj| {
                (0_usize..2).for_each(|dk| {
                    // Note: di, dj, dk 都不会超过 1，所以转换成 i64 是安全的
                    let i = ((i + di as i64) & 255) as usize;
                    let j = ((j + dj as i64) & 255) as usize;
                    let k = ((k + dk as i64) & 255) as usize;

                    c[di][dj][dk] =
                        self.rand_vecs[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]];
                });
            });
        });

        perlin_interp(&c, p)
    }

    /// 湍流
    ///
    /// A composite noise that has multiple summed frequencies is used.
    /// This is usually called turbulence, and is a sum of repeated calls to noise.
    pub fn turbulence(&self, p: &Point3, depth: usize) -> f64 {
        // let mut accum = 0.;
        let mut weight = 1. * 2.;
        let mut temp_p = p / 2.;

        (0..depth)
            .map(|_| {
                // let accum = weight * self.noise(temp_p);
                weight *= 0.5;
                temp_p *= 2.;

                weight * self.noise(&temp_p)
                // accum
            })
            .sum::<f64>()
            .abs()

        // for _ in 0..depth {
        //     accum += weight * self.noise(temp_p);
        //     weight *= 0.5;
        //     temp_p *= 2.;
        // }

        // accum.abs()
    }
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], p: &Point3) -> f64 {
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();

    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);

    c.iter()
        .enumerate()
        .map(|(i, ic)| {
            ic.iter()
                .enumerate()
                .map(|(j, jc)| {
                    jc.iter()
                        .enumerate()
                        .map(|(k, val)| {
                            let i = i as f64;
                            let j = j as f64;
                            let k = k as f64;

                            let weight_v = Vec3::new(u - i, v - j, w - k);

                            (i * uu + (1. - i) * (1. - uu))
                                * (j * vv + (1. - j) * (1. - vv))
                                * (k * ww + (1. - k) * (1. - ww))
                                * val.dot(&weight_v)
                        })
                        .sum::<f64>()
                })
                .sum::<f64>()
        })
        .sum::<f64>()
}

impl Default for Perlin {
    fn default() -> Self {
        let mut rand_vecs = [Vec3::zero(); POINT_COUNT];
        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            rand_vecs[i] = Vec3::random_range(-1., 1.).to_unit();

            perm_x[i] = i;
            perm_y[i] = i;
            perm_z[i] = i;
        }

        permute_perm(&mut perm_x);
        permute_perm(&mut perm_y);
        permute_perm(&mut perm_z);

        Self {
            rand_vecs,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

fn permute_perm<const N: usize>(perm: &mut [usize; N]) {
    for i in (1..N).rev() {
        let target = common::random_range(0_usize, i);

        perm.swap(i, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn permute_perm_should_panic() {
        let mut perm = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        permute_perm(&mut perm);

        assert_eq!(perm, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    }

    #[test]
    fn permute_perm_should_work() {
        let mut perm = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        permute_perm(&mut perm);
        perm.sort();

        assert_eq!(perm, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    }
}
