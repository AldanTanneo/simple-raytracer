use rand::Rng;

pub const MAX_PIXEL: f64 = 256.0 - f64::EPSILON;

macro_rules! new_color {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::vec3::color::Colour {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    #[inline(always)]
    pub fn red(self) -> f64 {
        self.r
    }

    #[inline(always)]
    pub fn green(self) -> f64 {
        self.g
    }

    #[inline(always)]
    pub fn blue(self) -> f64 {
        self.b
    }
}

impl Colour {
    pub const RED: Self = new_color!(1.0, 0.0, 0.0);
    pub const YELLOW: Self = new_color!(1.0, 1.0, 0.0);
    pub const GREEN: Self = new_color!(0.0, 1.0, 0.0);
    pub const CYAN: Self = new_color!(0.0, 1.0, 1.0);
    pub const BLUE: Self = new_color!(0.0, 0.0, 1.0);
    pub const MAGENTA: Self = new_color!(1.0, 0.0, 1.0);
    pub const BLACK: Self = new_color!(0.0, 0.0, 0.0);
    pub const WHITE: Self = new_color!(1.0, 1.0, 1.0);

    #[inline(always)]
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        new_color!(r, g, b)
    }

    #[inline]
    pub fn as_bytes(self, samples_per_pixel: u32) -> [u8; 3] {
        let scale = 1.0 / samples_per_pixel as f64;

        [
            ((self.red() * scale).sqrt() * MAX_PIXEL) as u8,
            ((self.green() * scale).sqrt() * MAX_PIXEL) as u8,
            ((self.blue() * scale).sqrt() * MAX_PIXEL) as u8,
        ]
    }

    #[inline]
    pub const fn components(self) -> (f64, f64, f64) {
        (self.r, self.g, self.b)
    }

    #[inline]
    pub fn random<T: Rng>(rng: &mut T) -> Self {
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }

    #[inline]
    pub fn near_zero(self) -> bool {
        let epsilon = 1e-8_f64;
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

    #[inline]
    pub fn clamp(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }
}

/**** Operator overloading for Color ****/

impl std::ops::Neg for Colour {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl std::ops::Add for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Colour {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl std::ops::SubAssign for Colour {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::MulAssign for Colour {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Div for Colour {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl std::ops::DivAssign for Colour {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

/**** Operator overloading for f64 ****/

impl std::ops::Add<f64> for Colour {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Self {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
        }
    }
}

impl std::ops::AddAssign<f64> for Colour {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<f64> for Colour {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Self {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
        }
    }
}

impl std::ops::SubAssign<f64> for Colour {
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl std::ops::MulAssign<f64> for Colour {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<f64> for Colour {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl std::ops::DivAssign<f64> for Colour {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl std::ops::Index<usize> for Colour {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Index out of bounds"),
        }
    }
}
