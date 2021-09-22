use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug)]
pub enum CameraType {
    ThinLens {
        lens_radius: f32,
        base: (Vec3, Vec3),
        to_lower_left_corner: Vec3,
    },
    Isomorphic {
        direction: Vec3,
    },
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub camera_type: CameraType,
}

impl Camera {
    pub fn new(
        origin: Point3,
        look_at: Point3,
        up_vector: Vec3,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        vertical_fov: f32,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (origin - look_at).unit_vector();
        let u = up_vector.cross(w).unit_vector();
        let v = w.cross(u);

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w * focus_dist;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            camera_type: CameraType::ThinLens {
                lens_radius: aperture / 2.0,
                base: (u, v),
                to_lower_left_corner: lower_left_corner - origin,
            },
        }
    }

    pub fn isomorphic(
        origin: Point3,
        look_at: Point3,
        up_vector: Vec3,
        aspect_ratio: f32,
        vertical_fov: f32,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (origin - look_at).unit_vector();
        let u = up_vector.cross(w).unit_vector();
        let v = w.cross(u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            camera_type: CameraType::Isomorphic { direction: w },
        }
    }

    pub fn get_ray(&self, u: f32, v: f32, r: f32, theta: f32) -> Ray {
        match self.camera_type {
            CameraType::Isomorphic { direction } => Ray::new(
                self.lower_left_corner + self.horizontal * u + self.vertical * v,
                direction,
            ),
            CameraType::ThinLens {
                lens_radius,
                base,
                to_lower_left_corner,
            } => {
                let rd = Vec3::random_in_unit_disk(r, theta) * lens_radius;
                let offset = base.0 * rd.x + base.1 * rd.y;
                Ray::new(
                    self.origin + offset,
                    to_lower_left_corner + self.horizontal * u + self.vertical * v - offset,
                )
            }
        }
    }
}
