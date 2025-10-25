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
        let i = ((4. * p.x) as i64 & 255) as usize;
        let j = ((4. * p.y) as i64 & 255) as usize;
        let k = ((4. * p.z) as i64 & 255) as usize;

        self.rand_floats[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
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
