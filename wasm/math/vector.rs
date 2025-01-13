use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub, SubAssign};

use super::traits::Sqrt;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Vector<T>
where
    for<'a> &'a T: Mul<Output = T> + Add<Output = T>,
{
    pub fn dot(&self, other: &Self) -> T {
        &(&(&self.x * &other.x) + &(&self.y * &other.y)) + &(&self.z * &other.z)
    }
}

impl<T> Vector<T>
where
    for<'a> &'a T: Mul<Output = T> + Add<Output = T>,
    T: Sqrt,
{
    pub fn abs(&self) -> T {
        self.dot(self).sqrt()
    }
}

impl<T> Vector<T>
where
    for<'a> &'a T: Mul<Output = T> + Add<Output = T> + Div<Output = T>,
    T: Sqrt,
{
    pub fn normalized(&self) -> Self {
        self / &self.abs()
    }
}

impl<T> Add for &Vector<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = Vector<T>;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: &self.x + &other.x,
            y: &self.y + &other.y,
            z: &self.z + &other.z,
        }
    }
}

impl<T> Sub for &Vector<T>
where
    for<'a> &'a T: Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: &self.x - &other.x,
            y: &self.y - &other.y,
            z: &self.z - &other.z,
        }
    }
}

impl<T> AddAssign<&Vector<T>> for Vector<T>
where
    for<'a> T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, other: &Vector<T>) {
        self.x += &other.x;
        self.y += &other.y;
        self.z += &other.z;
    }
}

impl<T> SubAssign<&Vector<T>> for Vector<T>
where
    for<'a> T: SubAssign<&'a T>,
{
    fn sub_assign(&mut self, other: &Vector<T>) {
        self.x -= &other.x;
        self.y -= &other.y;
        self.z -= &other.z;
    }
}

impl<T> Neg for &Vector<T>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = Vector<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -&self.x,
            y: -&self.y,
            z: -&self.z,
        }
    }
}

impl<T> Mul for &Vector<T>
where
    for<'a> &'a T: Mul<Output = T> + Sub<Output = T>,
{
    type Output = Vector<T>;

    fn mul(self, other: Self) -> Vector<T> {
        Self::Output {
            x: &(&self.y * &other.z) - &(&self.z * &other.y),
            y: &(&self.z * &other.x) - &(&self.x * &other.z),
            z: &(&self.x * &other.y) - &(&self.y * &other.x),
        }
    }
}

impl<T> Mul<&T> for &Vector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Vector<T>;

    fn mul(self, s: &T) -> Self::Output {
        Self::Output {
            x: &self.x * s,
            y: &self.y * s,
            z: &self.z * s,
        }
    }
}

impl<T> Div<&T> for &Vector<T>
where
    for<'a> &'a T: Div<Output = T>,
{
    type Output = Vector<T>;

    fn div(self, s: &T) -> Self::Output {
        Self::Output {
            x: &self.x / s,
            y: &self.y / s,
            z: &self.z / s,
        }
    }
}

impl<T> DivAssign<&T> for Vector<T>
where
    for<'a> T: DivAssign<&'a T>,
{
    fn div_assign(&mut self, s: &T) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_float() {
        let a = Vector::new(12.3, 0.1, 2.1);
        let b = Vector::new(-10.2, -50.4, -9.0);
        let c = Vector::new(-0.5, -10.3, 3.0);
        assert_eq!(&a + &b, Vector::new(12.3 - 10.2, 0.1 - 50.4, 2.1 - 9.0));
        assert_eq!(&a + &c, Vector::new(12.3 - 0.5, 0.1 - 10.3, 2.1 + 3.0));
        assert_eq!(&b + &c, Vector::new(-10.2 - 0.5, -50.4 - 10.3, -9.0 + 3.0));
    }

    #[test]
    fn add_int() {
        let a = Vector::new(12, 0, 2);
        let b = Vector::new(-10, -50, -9);
        let c = Vector::new(-0, -10, 3);
        assert_eq!(&a + &b, Vector::new(2, -50, -7));
        assert_eq!(&a + &c, Vector::new(12, -10, 5));
        assert_eq!(&b + &c, Vector::new(-10, -60, -6));
    }

    #[test]
    fn sub_float() {
        let a = Vector::new(-1.3, 0.15, 0.0);
        let b = Vector::new(2.2, -0.44, -1.0);
        let c = Vector::new(-4.1, 30.0, -0.09);
        assert_eq!(&a - &b, Vector::new(-1.3 - 2.2, 0.15 + 0.44, 1.0));
        assert_eq!(&a - &c, Vector::new(-1.3 + 4.1, 0.15 - 30.0, 0.09));
        assert_eq!(&b - &c, Vector::new(2.2 + 4.1, -0.44 - 30.0, -1.0 + 0.09));
    }

    #[test]
    fn sub_int() {
        let a = Vector::new(-1, 0, 1);
        let b = Vector::new(2, -44, -1);
        let c = Vector::new(-4, 30, 0);
        assert_eq!(&a - &b, Vector::new(-3, 44, 2));
        assert_eq!(&a - &c, Vector::new(3, -30, 1));
        assert_eq!(&b - &c, Vector::new(6, -74, -1));
    }

    #[test]
    fn add_assign_float() {
        let mut a = Vector::new(-1.3, 0.1, -2.1);
        let b = Vector::new(0.2, 0.4, -10.0);
        let c = Vector::new(0.4, -3.2, 1.8);
        a += &b;
        assert_eq!(a, Vector::new(-1.3 + 0.2, 0.1 + 0.4, -2.1 - 10.0));
        a += &c;
        assert_eq!(
            a,
            Vector::new(-1.3 + 0.2 + 0.4, 0.1 + 0.4 - 3.2, -2.1 - 10.0 + 1.8),
        );
    }

    #[test]
    fn add_assign_int() {
        let mut a = Vector::new(-1, 0, -2);
        let b = Vector::new(2, 4, -10);
        let c = Vector::new(0, -3, 8);
        a += &b;
        assert_eq!(a, Vector::new(1, 4, -12));
        a += &c;
        assert_eq!(a, Vector::new(1, 1, -4));
    }

    #[test]
    fn sub_assign_float() {
        let mut a = Vector::new(1.3, 0.1, -2.1);
        let b = Vector::new(0.5, -0.49, 0.01);
        let c = Vector::new(0.4, -3.2, 1.8);
        a -= &b;
        assert_eq!(a, Vector::new(1.3 - 0.5, 0.1 + 0.49, -2.1 - 0.01));
        a -= &c;
        assert_eq!(
            a,
            Vector::new(1.3 - 0.5 - 0.4, 0.1 + 0.49 + 3.2, -2.1 - 0.01 - 1.8)
        );
    }

    #[test]
    fn sub_assign_int() {
        let mut a = Vector::new(1, 0, -2);
        let b = Vector::new(5, -49, -1);
        let c = Vector::new(4, -32, 18);
        a -= &b;
        assert_eq!(a, Vector::new(-4, 49, -1));
        a -= &c;
        assert_eq!(a, Vector::new(-8, 81, -19));
    }

    #[test]
    fn neg_float() {
        let a = Vector::new(-1.3, 0.15, -30.8);
        assert_eq!(-&a, Vector::new(1.3, -0.15, 30.8));
    }

    #[test]
    fn neg_int() {
        let a = Vector::new(-1, 0, 30);
        assert_eq!(-&a, Vector::new(1, 0, -30));
    }

    #[test]
    fn mul_float() {
        let a = Vector::new(-1.3, 0.15, -30.8);
        assert_eq!(&a * &3.8, Vector::new(-1.3 * 3.8, 0.15 * 3.8, -30.8 * 3.8));
    }

    #[test]
    fn mul_int() {
        let a = Vector::new(-1, 15, -30);
        assert_eq!(&a * &3, Vector::new(-3, 45, -90));
    }

    #[test]
    fn mul_float_vector() {
        let a = &Vector::new(-1.3, 0.15, -30.8);
        let b = &Vector::new(-20.4, -3.8, 11.3);
        let c = &Vector::new(511.35, -2.9, 99.2);
        assert_eq!(
            a * b,
            Vector::new(
                0.15 * 11.3 - 30.8 * 3.8,
                30.8 * 20.4 + 1.3 * 11.3,
                1.3 * 3.8 + 0.15 * 20.4
            )
        );
        assert_eq!(
            b * c,
            Vector::new(
                -3.8 * 99.2 + 11.3 * 2.9,
                11.3 * 511.35 + 20.4 * 99.2,
                20.4 * 2.9 + 3.8 * 511.35
            )
        );
        assert_eq!(
            c * a,
            Vector::new(
                2.9 * 30.8 - 99.2 * 0.15,
                -99.2 * 1.3 + 511.35 * 30.8,
                511.35 * 0.15 - 2.9 * 1.3
            )
        );
    }

    #[test]
    fn mul_int_vector() {
        let a = &Vector::new(-3, 15, -30);
        let b = &Vector::new(-20, -3, 11);
        let c = &Vector::new(511, -9, 99);
        assert_eq!(
            a * b,
            Vector::new(15 * 11 - 30 * 3, 30 * 20 + 3 * 11, 3 * 3 + 15 * 20)
        );
        assert_eq!(
            b * c,
            Vector::new(-3 * 99 + 11 * 9, 11 * 511 + 20 * 99, 20 * 9 + 3 * 511)
        );
        assert_eq!(
            c * a,
            Vector::new(9 * 30 - 99 * 15, -99 * 3 + 511 * 30, 511 * 15 - 9 * 3)
        );
    }

    #[test]
    fn div_float() {
        let a = &Vector::new(-1.3, 0.15, -30.8);
        assert_eq!(a / &3.8, Vector::new(-1.3 / 3.8, 0.15 / 3.8, -30.8 / 3.8));
        assert_eq!(
            a / &-873.64,
            Vector::new(1.3 / 873.64, -0.15 / 873.64, 30.8 / 873.64)
        );
    }

    #[test]
    fn div_int() {
        let a = &Vector::new(-1, 15, -30);
        assert_eq!(a / &3, Vector::new(0, 5, -10));
        assert_eq!(a / &-14, Vector::new(0, -1, 2));
    }

    #[test]
    fn dot() {
        let a = &Vector::new(-1.3, 0.15, -30.8);
        let b = &Vector::new(-20.4, -3.8, 11.3);
        let c = &Vector::new(511.35, -2.9, 99.2);
        assert_eq!(a.dot(b), 1.3 * 20.4 - 0.15 * 3.8 - 30.8 * 11.3);
        assert_eq!(a.dot(c), -1.3 * 511.35 - 0.15 * 2.9 - 30.8 * 99.2);
        assert_eq!(a.dot(a), 1.3 * 1.3 + 0.15 * 0.15 + 30.8 * 30.8);
    }

    #[test]
    fn abs() {
        let a = Vector::new(-1.3, 0.15, -30.8);
        let b = Vector::new(-20.4, -3.8, 11.3);
        let c = Vector::new(511.35, -2.9, 99.2);
        assert_eq!(
            a.abs(),
            (1.3 * 1.3 + 0.15 * 0.15 + 30.8 * 30.8 as f64).sqrt()
        );
        assert_eq!(
            b.abs(),
            (20.4 * 20.4 + 3.8 * 3.8 + 11.3 * 11.3 as f32).sqrt()
        );
        assert_eq!(
            c.abs(),
            (511.35 * 511.35 + 2.9 * 2.9 + 99.2 * 99.2 as f64).sqrt()
        );
    }

    #[test]
    fn normalized() {
        let a = Vector::new(-1.3, 0.15, -30.8);
        let b = Vector::new(-20.4, -3.8, 11.3);
        let ta = (1.3 * 1.3 + 0.15 * 0.15 + 30.8 * 30.8 as f64).sqrt();
        let tb = (20.4 * 20.4 + 3.8 * 3.8 + 11.3 * 11.3 as f64).sqrt();
        assert_eq!(
            a.normalized(),
            Vector::new(-1.3 / ta, 0.15 / ta, -30.8 / ta)
        );
        assert_eq!(
            b.normalized(),
            Vector::new(-20.4 / tb, -3.8 / tb, 11.3 / tb)
        );
    }
}
