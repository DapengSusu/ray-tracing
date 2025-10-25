use crate::{Point3, common};

/// Perlin noise, it takes a 3D point as input and always returns
/// the same randomish number. Nearby points return similar numbers.
pub struct Perlin {
    rand_floats: [f64; POINT_COUNT],
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

        let mut c = [[[0_f64; 2]; 2]; 2];

        (0_usize..2).for_each(|di| {
            (0_usize..2).for_each(|dj| {
                (0_usize..2).for_each(|dk| {
                    // Note: di, dj, dk 都不会超过 1，所以转换成 i64 是安全的
                    let i = ((i + di as i64) & 255) as usize;
                    let j = ((j + dj as i64) & 255) as usize;
                    let k = ((k + dk as i64) & 255) as usize;

                    c[di][dj][dk] =
                        self.rand_floats[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]];
                });
            });
        });

        trilinear_interp(&c, p)

        // // Note: 这里的 p.x 是 f64，数值有正有负，不能转为 u64，
        // // 否则会丢失数据得到异常的图像
        // let i = ((4. * p.x) as i64 & 255) as usize;
        // let j = ((4. * p.y) as i64 & 255) as usize;
        // let k = ((4. * p.z) as i64 & 255) as usize;

        // self.rand_floats[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

// 线性插值
fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], p: &Point3) -> f64 {
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();

    c.iter()
        .enumerate()
        .map(|(i, ic)| {
            ic.iter()
                .enumerate()
                .map(|(j, jc)| {
                    jc.iter()
                        .enumerate()
                        .map(|(k, val)| {
                            (i as f64 * u + (1. - i as f64) * (1. - u))
                                * (j as f64 * v + (1. - j as f64) * (1. - v))
                                * (k as f64 * w + (1. - k as f64) * (1. - w))
                                * val
                        })
                        .sum::<f64>()
                })
                .sum::<f64>()
        })
        .sum::<f64>()
}

impl Default for Perlin {
    fn default() -> Self {
        let mut rand_floats = [0.; POINT_COUNT];
        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            rand_floats[i] = common::random();
            perm_x[i] = i;
            perm_y[i] = i;
            perm_z[i] = i;
        }

        permute_perm(&mut perm_x);
        permute_perm(&mut perm_y);
        permute_perm(&mut perm_z);

        Self {
            rand_floats,
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
