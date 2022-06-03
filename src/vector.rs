mod vector {
    use std::ops::{Neg, AddAssign, MulAssign, DivAssign};

    // Seems we can 'derive' and get some traits for free,
    // Debug here is used for printing the Vec3 in formatting
    // and PartialEq for when we make asserts_eq in tests.
    #[derive(Debug, PartialEq)]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32
    }

    // Implement operator traits,
    impl Neg for Vec3 {
        type Output = Self; // TODO: figure out this standard,

        fn neg(self) -> Self {
            return Self {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            };
        }
    }

    // Overloading implementations for different types, here we impl for both Vec3 and f32
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

    impl MulAssign for Vec3 {
        fn mul_assign(&mut self, other: Self) {
            *self = Self {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            };
        }
    }

    impl DivAssign for Vec3 {
        fn div_assign(&mut self, other: Self) {
            *self = Self {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
            };
        }
    }

    pub trait Dot {
        fn dot(&self, other: &Self) -> f32;
    }

    impl Dot for Vec3 {
        fn dot(&self, other: &Self) -> f32 {
            return self.x * other.x + self.y * other.y + self.z * other.z;
        }
    }

    // Can't find how to extend .len() so doing this,
    pub trait Length {
        fn length(&self) -> f32;
    }

    impl Length for Vec3 {
        fn length(&self) -> f32 {
            return self.dot(self).sqrt();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::vector::*;

    #[test]
    fn test_vec3_neg() {
        let a = Vec3{x: 1.0, y: 0.0, z: -3.0};
        assert_eq!(-a, Vec3{x: -1.0, y: -0.0, z: 3.0});
    }

    #[test]
    fn test_vec3_add_assign_vec3() {
        let mut a = Vec3{x:1.0, y:2.0, z:3.0};
        let b = Vec3{x:1.0, y:1.0, z:1.0};

        a += b;

        assert_eq!(a,Vec3{x: 2.0, y: 3.0, z: 4.0});
    }

    #[test]
    fn test_vec3_add_assign_f32() {
        let mut a = Vec3{x:1.0, y:2.0, z:3.0};

        a += 1.0;

        assert_eq!(a,Vec3{x: 2.0, y: 3.0, z: 4.0});
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = Vec3{x: 1.0, y: 2.0, z: 0.5};

        a *= b;

        assert_eq!(a, Vec3{x: 1.0, y: 4.0, z: 1.5});
    }

    #[test]
    fn test_vec3_div_assign() {
        let mut a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = Vec3{x: 2.0, y: 3.0, z: 0.8};

        a /= b;

        assert_eq!(a,Vec3{x: 0.5, y: 0.6666667, z: 3.75});
    }

    #[test]
    fn test_vec3_dot() {
        let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = Vec3{x: 2.0, y: 3.0, z: 4.0};

        let dot_product = a.dot(&b);

        assert_eq!(dot_product, 20.0);
    }
}