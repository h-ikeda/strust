use super::{
    traits::{Cos, Hypot, Sin},
    vector::Vector,
};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Quaternion<T> {
    pub v: Vector<T>,
    pub w: T,
}

impl<T> Quaternion<T> {
    pub const fn new(v: Vector<T>, w: T) -> Self {
        Self { v, w }
    }
}

impl<T> From<T> for Quaternion<T>
where
    T: Default,
{
    fn from(value: T) -> Self {
        Self {
            v: Vector::default(),
            w: value,
        }
    }
}

impl<T> Quaternion<T>
where
    T: From<u8> + Sin + Cos + Hypot + PartialOrd,
    for<'a> &'a T: Add<Output = T>,
    for<'a> &'a T: Div<Output = T>,
    for<'a> &'a T: Mul<Output = T>,
{
    /// This function needs explicit type specification to be called because of a compiler bug.
    ///
    /// The `axis` vector's direction is parallel to the rotation axis, and its norm represents the rotation angle.
    pub fn from_rotation(axis: &Vector<T>) -> Self {
        match axis.abs() {
            theta if theta > 0.into() => Self {
                v: &(axis / &theta) * &(&theta / &2.into()).sin(),
                w: (&theta / &2.into()).cos(),
            },
            theta => Self {
                v: axis * &theta,
                w: theta.cos(),
            },
        }
    }
}

impl<T> Quaternion<T>
where
    T: From<u8> + Clone,
{
    pub fn from_translation(t: &Vector<T>) -> Self {
        Self {
            v: t.clone(),
            w: 0.into(),
        }
    }
}

impl<T> Quaternion<T>
where
    for<'a> &'a T: Add<Output = T> + Mul<Output = T>,
{
    pub fn dot(&self, other: &Self) -> T {
        &(&self.w * &other.w) + &self.v.dot(&other.v)
    }
}

impl<T> Quaternion<T>
where
    T: Hypot,
{
    pub fn abs(&self) -> T {
        self.v.abs().hypot(&self.w)
    }
}

impl<T> Quaternion<T>
where
    for<'a> &'a T: Div<Output = T>,
    T: Hypot,
{
    pub fn normalized(&self) -> Self {
        self / &self.abs()
    }
}

impl<T> Quaternion<T>
where
    T: Clone,
    for<'a> &'a T: Neg<Output = T>,
{
    pub fn conj(&self) -> Self {
        Self {
            w: self.w.clone(),
            v: -&self.v,
        }
    }
}
impl<T> Quaternion<T>
where
    for<'a> &'a T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T>,
    T: Clone,
{
    pub fn inv(&self) -> Self {
        &self.conj() / &self.dot(self)
    }
}

impl<T> Add for &Quaternion<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = Quaternion<T>;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            v: &self.v + &other.v,
            w: &self.w + &other.w,
        }
    }
}

impl<T> Sub for &Quaternion<T>
where
    for<'a> &'a T: Sub<Output = T>,
{
    type Output = Quaternion<T>;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            v: &self.v - &other.v,
            w: &self.w - &other.w,
        }
    }
}

impl<T> AddAssign<&Quaternion<T>> for Quaternion<T>
where
    for<'a> T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, other: &Quaternion<T>) {
        self.v += &other.v;
        self.w += &other.w;
    }
}

impl<T> SubAssign<&Quaternion<T>> for Quaternion<T>
where
    for<'a> T: SubAssign<&'a T>,
{
    fn sub_assign(&mut self, other: &Quaternion<T>) {
        self.v -= &other.v;
        self.w -= &other.w;
    }
}

impl<T> Neg for &Quaternion<T>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = Quaternion<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            v: -&self.v,
            w: -&self.w,
        }
    }
}

impl<T> Mul for &Quaternion<T>
where
    for<'a> &'a T: Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type Output = Quaternion<T>;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            v: &(&(&self.v * &other.w) + &(&other.v * &self.w)) + &(&self.v * &other.v),
            w: &(&self.w * &other.w) - &self.v.dot(&other.v),
        }
    }
}

impl<T> Mul<&T> for &Quaternion<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Quaternion<T>;

    fn mul(self, s: &T) -> Self::Output {
        Self::Output {
            v: &self.v * s,
            w: &self.w * s,
        }
    }
}

impl<T> MulAssign<&Quaternion<T>> for Quaternion<T>
where
    for<'a> &'a T: Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    fn mul_assign(&mut self, other: &Quaternion<T>) {
        let w = &(&self.w * &other.w) - &self.v.dot(&other.v);
        self.v = &(&(&self.v * &other.w) + &(&other.v * &self.w)) + &(&self.v * &other.v);
        self.w = w;
    }
}

impl<T> MulAssign<&T> for Quaternion<T>
where
    for<'a> T: MulAssign<&'a T>,
{
    fn mul_assign(&mut self, s: &T) {
        self.v *= s;
        self.w *= s;
    }
}

impl<T> Div<&T> for &Quaternion<T>
where
    for<'a> &'a T: Div<Output = T>,
{
    type Output = Quaternion<T>;

    fn div(self, s: &T) -> Self::Output {
        Self::Output {
            v: &self.v / s,
            w: &self.w / s,
        }
    }
}

impl<T> DivAssign<&T> for Quaternion<T>
where
    for<'a> T: DivAssign<&'a T>,
{
    fn div_assign(&mut self, s: &T) {
        self.v /= s;
        self.w /= s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        assert_eq!(
            Quaternion::from(-3),
            Quaternion::new(Vector::new(0, 0, 0), -3)
        );
        assert_eq!(
            Quaternion::from(3.3),
            Quaternion::new(Vector::new(0.0, 0.0, 0.0), 3.3)
        );
    }

    #[test]
    fn add() {
        let a = &Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = &Quaternion::new(Vector::new(0.2, -0.4, 0.0), 0.11);
        assert_eq!(
            a + b,
            Quaternion::new(Vector::new(1.3 + 0.2, 0.1 - 0.4, -2.1), -0.8 + 0.11)
        );
    }

    #[test]
    fn sub() {
        let a = &Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = &Quaternion::new(Vector::new(0.2, -0.4, 0.0), 0.11);
        assert_eq!(
            a - b,
            Quaternion::new(Vector::new(1.3 - 0.2, 0.1 + 0.4, -2.1), -0.8 - 0.11)
        );
    }

    #[test]
    fn add_assign() {
        let mut a = Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = &Quaternion::new(Vector::new(0.2, -0.4, 0.0), 0.11);
        a += b;
        assert_eq!(
            a,
            Quaternion::new(Vector::new(1.3 + 0.2, 0.1 - 0.4, -2.1), -0.8 + 0.11)
        );
    }

    #[test]
    fn sub_assign() {
        let mut a = Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = &Quaternion::new(Vector::new(0.2, -0.4, 0.0), 0.11);
        a -= b;
        assert_eq!(
            a,
            Quaternion::new(Vector::new(1.3 - 0.2, 0.1 + 0.4, -2.1), -0.8 - 0.11)
        );
    }

    #[test]
    fn neg() {
        let a = &Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        assert_eq!(-a, Quaternion::new(Vector::new(-1.3, -0.1, 2.1), 0.8));
    }

    #[test]
    fn mul() {
        let a = &Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = &Quaternion::new(Vector::new(0.2, -0.4, 31.1), 0.11);
        assert_eq!(
            a * b,
            Quaternion::new(
                Vector::new(
                    0.11 * 1.3 - 0.8 * 0.2 + (0.1 * 31.1 - 2.1 * 0.4),
                    0.11 * 0.1 + 0.8 * 0.4 + (-2.1 * 0.2 - 1.3 * 31.1),
                    -0.11 * 2.1 - 0.8 * 31.1 + (-1.3 * 0.4 - 0.1 * 0.2),
                ),
                -0.8 * 0.11 - (1.3 * 0.2 - 0.1 * 0.4 - 2.1 * 31.1),
            )
        );
    }

    #[test]
    fn mul_scalar() {
        let a = &Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        assert_eq!(
            a * &2.3,
            Quaternion::new(Vector::new(1.3 * 2.3, 0.1 * 2.3, -2.1 * 2.3), -0.8 * 2.3),
        );
        assert_eq!(
            a * &-3.6,
            Quaternion::new(Vector::new(-1.3 * 3.6, -0.1 * 3.6, 2.1 * 3.6), 0.8 * 3.6),
        );
    }

    #[test]
    fn mul_assign() {
        let mut a = Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = Quaternion::new(Vector::new(0.2, -0.4, 31.1), 0.11);
        a *= &b;
        assert_eq!(
            a,
            Quaternion::new(
                Vector::new(
                    0.11 * 1.3 - 0.8 * 0.2 + (0.1 * 31.1 - 2.1 * 0.4),
                    0.11 * 0.1 + 0.8 * 0.4 + (-2.1 * 0.2 - 1.3 * 31.1),
                    -0.11 * 2.1 - 0.8 * 31.1 + (-1.3 * 0.4 - 0.1 * 0.2),
                ),
                -0.8 * 0.11 - (1.3 * 0.2 - 0.1 * 0.4 - 2.1 * 31.1),
            )
        );
    }

    #[test]
    fn mul_assign_scalar() {
        let mut a = Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        a *= &2.3;
        assert_eq!(
            a,
            Quaternion::new(&Vector::new(1.3, 0.1, -2.1) * &2.3, -0.8 * 2.3),
        );
        a *= &-3.6;
        assert_eq!(
            a,
            Quaternion::new(
                &(&Vector::new(-1.3, -0.1, 2.1) * &2.3) * &3.6,
                0.8 * 2.3 * 3.6
            ),
        );
    }

    #[test]
    fn div() {
        let a = &Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        assert_eq!(
            a / &2.3,
            Quaternion::new(&Vector::new(1.3, 0.1, -2.1) / &2.3, -0.8 / 2.3),
        );
        assert_eq!(
            a / &-3.6,
            Quaternion::new(Vector::new(-1.3 / 3.6, -0.1 / 3.6, 2.1 / 3.6), 0.8 / 3.6),
        );
    }

    #[test]
    fn div_assign() {
        let mut a = Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        a /= &2.3;
        assert_eq!(
            a,
            Quaternion::new(&Vector::new(1.3, 0.1, -2.1) / &2.3, -0.8 / 2.3),
        );
        a /= &-3.6;
        assert_eq!(
            a,
            Quaternion::new(
                Vector::new(-1.3 / 2.3 / 3.6, -0.1 / 2.3 / 3.6, 2.1 / 2.3 / 3.6),
                0.8 / 2.3 / 3.6
            ),
        );
    }

    #[test]
    fn abs() {
        let a = Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = Quaternion::new(Vector::new(0.2, -0.4, 31.1), 0.11);
        assert_eq!(a.abs(), (1.3 as f64).hypot(0.1).hypot(2.1).hypot(0.8));
        assert_eq!(b.abs(), (0.2 as f32).hypot(0.4).hypot(31.1).hypot(0.11));
    }

    #[test]
    fn normalized() {
        let a = Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = Quaternion::new(Vector::new(0.2, -0.4, 31.1), 0.11);
        assert_eq!(
            a.normalized(),
            Quaternion::new(
                Vector::new(
                    1.3 / (1.3 as f64).hypot(0.1).hypot(2.1).hypot(0.8),
                    0.1 / (1.3 as f64).hypot(0.1).hypot(2.1).hypot(0.8),
                    -2.1 / (1.3 as f64).hypot(0.1).hypot(2.1).hypot(0.8)
                ),
                -0.8 / (1.3 as f64).hypot(0.1).hypot(2.1).hypot(0.8)
            ),
        );
        assert_eq!(
            b.normalized(),
            Quaternion::new(
                Vector::new(
                    0.2 / (0.2 as f32).hypot(0.4).hypot(31.1).hypot(0.11),
                    -0.4 / (0.2 as f32).hypot(0.4).hypot(31.1).hypot(0.11),
                    31.1 / (0.2 as f32).hypot(0.4).hypot(31.1).hypot(0.11)
                ),
                0.11 / (0.2 as f32).hypot(0.4).hypot(31.1).hypot(0.11)
            ),
        );
    }

    #[test]
    fn conj() {
        let a = Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = Quaternion::new(Vector::new(0.2, -0.4, 31.1), 0.11);
        assert_eq!(
            a.conj(),
            Quaternion::new(Vector::new(-1.3, -0.1, 2.1), -0.8),
        );
        assert_eq!(
            b.conj(),
            Quaternion::new(Vector::new(-0.2, 0.4, -31.1), 0.11),
        );
    }

    #[test]
    fn inv() {
        let a = Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = Quaternion::new(Vector::new(0.2, -0.4, 31.1), 0.11);
        let ta = 1.3 * 1.3 + 0.1 * 0.1 + 2.1 * 2.1 + 0.8 * 0.8;
        let tb = 0.2 * 0.2 + 0.4 * 0.4 + 31.1 * 31.1 + 0.11 * 0.11;
        assert_eq!(
            a.inv(),
            Quaternion::new(Vector::new(-1.3 / ta, -0.1 / ta, 2.1 / ta), -0.8 / ta),
        );
        assert_eq!(
            b.inv(),
            Quaternion::new(Vector::new(-0.2 / tb, 0.4 / tb, -31.1 / tb), 0.11 / tb),
        );
    }

    #[test]
    fn dot() {
        let a = Quaternion::new(Vector::new(1.3, 0.1, -2.1), -0.8);
        let b = &Quaternion::new(Vector::new(0.2, -0.4, 31.1), 0.11);
        assert_eq!(a.dot(b), 1.3 * 0.2 - 0.1 * 0.4 - 2.1 * 31.1 - 0.8 * 0.11);
    }

    #[test]
    fn from_rotation() {
        assert_eq!(
            Quaternion::<f64>::from_rotation(&Vector::new(0.8, 3.2, -1.4)),
            Quaternion::new(
                Vector::new(
                    ((0.8 as f64).hypot(3.2).hypot(1.4) / 2.0).sin()
                        * (0.8 / (0.8 as f64).hypot(3.2).hypot(1.4)),
                    ((0.8 as f64).hypot(3.2).hypot(1.4) / 2.0).sin()
                        * (3.2 / (0.8 as f64).hypot(3.2).hypot(1.4)),
                    -((0.8 as f64).hypot(3.2).hypot(1.4) / 2.0).sin()
                        * (1.4 / (0.8 as f64).hypot(3.2).hypot(1.4)),
                ),
                ((0.8 as f64).hypot(3.2).hypot(1.4) / 2.0).cos()
            )
        );
        assert_eq!(
            Quaternion::<f32>::from_rotation(&Vector::new(0.0, 0.0, 0.0)),
            Quaternion::new(Vector::new(0.0, 0.0, 0.0), 1.0),
        );
        assert_eq!(
            Quaternion::<f32>::from_rotation(&Vector::new(2.2e-218, 1.3e-301, 9.0e-278)),
            Quaternion::new(Vector::new(0.0, 0.0, 0.0), 1.0),
        );
    }

    #[test]
    fn from_translation() {
        assert_eq!(
            Quaternion::from_translation(&Vector::new(0.8, 3.2, -1.4)),
            Quaternion::new(Vector::new(0.8, 3.2, -1.4), 0.0),
        );
    }

    #[test]
    fn default() {
        assert_eq!(
            Quaternion::default(),
            Quaternion::new(Vector::new(0, 0, 0), 0)
        );
        assert_eq!(
            Quaternion::default(),
            Quaternion::new(Vector::new(0.0, 0.0, 0.0), 0.0)
        );
    }
}
