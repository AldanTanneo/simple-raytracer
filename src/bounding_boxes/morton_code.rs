use crate::vec3::Vec3;

const MAX_42_BITS: f64 = (1u128 << 42) as f64;

/// Expands 42 bits integer to 128 bits by
/// inserting two zeros after each bit
fn expand_to_126_bits(mut x: u128) -> u128 {
    x &= 0x3ffffffffff;
    x = (x | x << 64) & 0x3ff0000000000000000ffffffff;
    x = (x | x << 32) & 0x3ff00000000ffff00000000ffff;
    x = (x | x << 16) & 0x30000ff0000ff0000ff0000ff0000ff;
    x = (x | x << 8) & 0x300f00f00f00f00f00f00f00f00f00f;
    x = (x | x << 4) & 0x30c30c30c30c30c30c30c30c30c30c3;
    x = (x | x << 2) & 0x9249249249249249249249249249249;
    x
}

/// Calculates the morton code for a 3D point taken in the unit cube
pub fn morton_code(point: Vec3) -> u128 {
    let x = (point.x * MAX_42_BITS).clamp(0.0, MAX_42_BITS - 1.0);
    let y = (point.y * MAX_42_BITS).clamp(0.0, MAX_42_BITS - 1.0);
    let z = (point.z * MAX_42_BITS).clamp(0.0, MAX_42_BITS - 1.0);
    let xx = expand_to_126_bits(x as u128);
    let yy = expand_to_126_bits(y as u128);
    let zz = expand_to_126_bits(z as u128);
    (xx << 2) | (yy << 1) | zz
}

pub fn find_split<A, B>(list: &[(A, B, u128)]) -> usize {
    let n = list.len();
    assert_ne!(n, 0);
    let first_code = list[0].2;
    let last_code = list[n - 1].2;

    if first_code == last_code {
        return n / 2;
    }

    let common_prefix = (first_code ^ last_code).leading_zeros();

    let mut split = 0;
    let mut step = n - 1;

    loop {
        step = (step + 1) / 2;
        let new_split = split + step;

        let split_code = list[new_split].2;
        let prefix = (first_code ^ split_code).leading_zeros();
        if prefix > common_prefix {
            split = new_split;
        }

        if step <= 1 {
            break split + 1;
        }
    }
}
