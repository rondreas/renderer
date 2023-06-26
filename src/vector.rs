use rand_distr::{Distribution, UnitSphere};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

// Seems we can 'derive' and get some traits for free,
// Debug here is used for printing the Vec3 in formatting
// and PartialEq for when we make asserts_eq in tests.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn near_zero(&self) -> bool {
        return self.x.abs() < f32::MIN_POSITIVE && self.y.abs() < f32::MIN_POSITIVE && self.z.abs() < f32::MIN_POSITIVE;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v,n) * *n
}

// https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics/snell'slaw
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(&-*uv, n).min(1.0);
    let r_out_perp: Vec3 = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel: Vec3 = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * *n;
    return r_out_perp + r_out_parallel;
}

#[inline]
pub fn random_in_unit_sphere() -> Vec3 {
    let v: [f32; 3] = UnitSphere.sample(&mut rand::thread_rng());
    Vec3 {
        x: v[0],
        y: v[1],
        z: v[2],
    }
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

// Implement operator traits,
impl Neg for Vec3 {
    type Output = Self; // TODO: figure out this standard,

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Overloading implementations for different types
//         | RHS    |LHS
//         V        V
impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// We want to be able to perform addition as f32 + Vec3, as well as Vec3 + f32
impl Add<Vec3> for f32 {
    // self here will be a f32, but the output type I want to be a Vec3
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

impl Add<f32> for Vec3 {
    // here however self will be a Vec3,
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

pub trait Dot {
    fn dot(&self, other: &Self) -> f32;
}

impl Dot for Vec3 {
    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// Can't find how to extend .len() so doing this,
pub trait Length {
    fn length_squared(&self) -> f32;
    fn length(&self) -> f32;
}

impl Length for Vec3 {
    fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }
}

// Utility functions,
pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

// https://developer.download.nvidia.com/cg/cross.html
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

#[cfg(test)]
mod tests {
    use crate::vector::*;

    #[test]
    fn test_vec3_neg() {
        let a = Vec3 {
            x: 1.0,
            y: 0.0,
            z: -3.0,
        };
        assert_eq!(
            -a,
            Vec3 {
                x: -1.0,
                y: -0.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn test_vec3_add_vec3() {
        // Test that we can add two vectors,
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        let c = a + b;
        assert_eq!(
            c,
            Vec3 {
                x: 4.0,
                y: 4.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn test_vec3_add_f32() {
        // Test addition between a vector and a float
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = a + 1.0;
        assert_eq!(
            b,
            Vec3 {
                x: 2.0,
                y: 3.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn test_f32_add_vec3() {
        // Test addition between a float and a vector
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = 1.0 + a;
        assert_eq!(
            b,
            Vec3 {
                x: 2.0,
                y: 3.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn test_vec3_add_assign_vec3() {
        let mut a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        a += b;

        assert_eq!(
            a,
            Vec3 {
                x: 2.0,
                y: 3.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn test_vec3_add_assign_f32() {
        let mut a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        a += 1.0;

        assert_eq!(
            a,
            Vec3 {
                x: 2.0,
                y: 3.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn test_vec3_mul_f32() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = a * 2.0;

        assert_eq!(
            b,
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn test_f32_mul_vec3() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = 2.0 * a;

        assert_eq!(
            b,
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 0.5,
        };

        a *= b;

        assert_eq!(
            a,
            Vec3 {
                x: 1.0,
                y: 4.0,
                z: 1.5
            }
        );
    }

    #[test]
    fn test_vec3_div_assign() {
        let mut a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 0.8,
        };

        a /= b;

        assert_eq!(
            a,
            Vec3 {
                x: 0.5,
                y: 0.6666667,
                z: 3.75
            }
        );
    }

    #[test]
    fn test_vec3_dot() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        let dot_product = a.dot(&b);

        assert_eq!(dot_product, 20.0);
    }
}
