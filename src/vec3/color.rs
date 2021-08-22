use rand::Rng;

pub const COLOR_MATRIX: [[f32; 3]; 3] = [[0.5, 0.5, 0.0], [0.0, 0.5, 0.5], [0.5, 0.0, 0.5]];
pub const INVERSE_MATRIX: [[f32; 3]; 3] = [[1.0, 1.0, -1.0], [-1.0, 1.0, 1.0], [1.0, -1.0, 1.0]];

pub const MAX_PIXEL: f32 = 256.0 - f32::EPSILON;

#[cfg(not(feature = "spectral_colors"))]
macro_rules! new_color {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::vec3::color::Color {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

#[cfg(feature = "spectral_colors")]
macro_rules! new_color {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::vec3::color::Color {
            r: 0.34 * $r + 0.33 * ($g + $b),
            g: 0.34 * $g + 0.33 * ($b + $r),
            b: 0.34 * $b + 0.33 * ($r + $g),
        }
    };
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[cfg(not(feature = "spectral_colors"))]
impl Color {
    #[inline(always)]
    pub fn red(self) -> f32 {
        self.r
    }

    #[inline(always)]
    pub fn green(self) -> f32 {
        self.g
    }

    #[inline(always)]
    pub fn blue(self) -> f32 {
        self.b
    }
}

#[cfg(feature = "spectral_colors")]
impl Color {
    #[inline(always)]
    pub fn red(self) -> f32 {
        67.0 * self.r - 33.0 * (self.g + self.b)
    }

    #[inline(always)]
    pub fn green(self) -> f32 {
        67.0 * self.g - 33.0 * (self.b + self.r)
    }

    #[inline(always)]
    pub fn blue(self) -> f32 {
        67.0 * self.b - 33.0 * (self.r + self.g)
    }
}

impl Color {
    pub const RED: Self = new_color!(1.0, 0.0, 0.0);
    pub const YELLOW: Self = new_color!(1.0, 1.0, 0.0);
    pub const GREEN: Self = new_color!(0.0, 1.0, 0.0);
    pub const CYAN: Self = new_color!(0.0, 1.0, 1.0);
    pub const BLUE: Self = new_color!(0.0, 0.0, 1.0);
    pub const MAGENTA: Self = new_color!(1.0, 0.0, 1.0);
    pub const BLACK: Self = new_color!(0.0, 0.0, 0.0);
    pub const WHITE: Self = new_color!(1.0, 1.0, 1.0);

    #[inline(always)]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        new_color!(r, g, b)
    }

    #[inline]
    pub fn as_bytes(&self, samples_per_pixel: u32) -> [u8; 3] {
        let scale = 1.0 / samples_per_pixel as f32;

        [
            ((self.red() * scale).sqrt() * MAX_PIXEL) as u8,
            ((self.green() * scale).sqrt() * MAX_PIXEL) as u8,
            ((self.blue() * scale).sqrt() * MAX_PIXEL) as u8,
        ]
    }

    #[inline]
    pub const fn components(self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }

    #[inline]
    pub fn random<T: Rng>(rng: &mut T) -> Self {
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }

    #[inline]
    pub fn near_zero(self) -> bool {
        let epsilon = 1e-8_f32;
        self.r.abs() < epsilon && self.g.abs() < epsilon && self.b.abs() < epsilon
    }

    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            r: self.r.min(rhs.r),
            g: self.g.min(rhs.g),
            b: self.b.min(rhs.b),
        }
    }

    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            r: self.r.max(rhs.r),
            g: self.g.max(rhs.g),
            b: self.b.max(rhs.b),
        }
    }
}

/**** Operator overloading for Color ****/

impl std::ops::Neg for Color {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl std::ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Color {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl std::ops::DivAssign for Color {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

/**** Operator overloading for f32 ****/

impl std::ops::Add<f32> for Color {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
        }
    }
}

impl std::ops::AddAssign<f32> for Color {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<f32> for Color {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
        }
    }
}

impl std::ops::SubAssign<f32> for Color {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl std::ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl std::ops::Index<usize> for Color {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Index out of bounds"),
        }
    }
}
