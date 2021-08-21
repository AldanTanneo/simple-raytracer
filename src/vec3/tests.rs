#![cfg(test)]
use super::{color::MAX_PIXEL, Vec3};

#[test]
fn check_correct_color_conversion() {
    assert_eq!(MAX_PIXEL as u8, 255);
    assert_eq!(0.0 as u8, 0);
}

#[test]
fn test_vector_length() {
    assert_eq!(Vec3::new(1.0, 2.0, 2.0).length(), 3.0);
    assert_eq!(Vec3::new(1.0, 1.0, 1.0).unit_vector().length(), 1.0);
}

#[test]
fn test_cross_product() {
    assert_eq!(
        Vec3::new(1.0, 0.0, 0.0).cross(Vec3::new(0.0, 1.0, 0.0)),
        Vec3::new(0.0, 0.0, 1.0)
    );
}

#[test]
fn test_dot_product() {
    assert_eq!(Vec3::new(1.0, 1.0, 0.0).dot(Vec3::new(0.0, 0.0, 1.0)), 0.0);
}
