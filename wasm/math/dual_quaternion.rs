use super::{
    quaternion::Quaternion,
    traits::{Cos, Sin},
    vector::Vector,
};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq)]
pub struct DualQuaternion<T> {
    pub p: Quaternion<T>,
    pub q: Quaternion<T>,
}

impl<T> DualQuaternion<T> {
    pub const fn new(p: Quaternion<T>, q: Quaternion<T>) -> Self {
        Self { p, q }
    }
}

impl<T> DualQuaternion<T>
where
    T: From<u8> + Clone + Sin + Cos,
    for<'a> &'a T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    /// This function needs explicit type specification to be called because of a compiler bug.
    pub fn from_translation_and_rotation(
        translation: &Vector<T>,
        rotation_axis: &Vector<T>,
        rotation_theta: &T,
    ) -> Self {
        let r = Quaternion::from_rotation(rotation_axis, rotation_theta);
        let rt = &r * &Quaternion::from_translation(translation);
        Self {
            p: r.into(),
            q: &rt / &2.into(),
        }
    }

    /// This function needs explicit type specification to be called because of a compiler bug.
    pub fn from_rotation_and_translation(
        rotation_axis: &Vector<T>,
        rotation_theta: &T,
        translation: &Vector<T>,
    ) -> Self {
        let r = Quaternion::from_rotation(rotation_axis, rotation_theta);
        let tr = &Quaternion::from_translation(translation) * &r;
        Self {
            p: r,
            q: &tr / &2.into(),
        }
    }
}

impl<T> DualQuaternion<T>
where
    for<'a> &'a T: Neg<Output = T>,
    T: Clone,
{
    pub fn conj_from_dual_number(&self) -> Self {
        Self {
            p: self.p.clone(),
            q: -&self.q,
        }
    }

    pub fn conj_from_quaternion(&self) -> Self {
        Self {
            p: self.p.conj(),
            q: self.q.conj(),
        }
    }

    pub fn conj_from_dual_number_and_quaternion(&self) -> Self {
        Self {
            p: self.p.conj(),
            q: -&self.q.conj(),
        }
    }
}

impl<T> DualQuaternion<T>
where
    for<'a> &'a T: Mul<Output = T> + Add<Output = T>,
{
    pub fn dot(&self, other: &Self) -> T {
        &self.p.dot(&other.p) + &self.q.dot(&other.q)
    }
}

impl<T> Add for &DualQuaternion<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = DualQuaternion<T>;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            p: &self.p + &other.p,
            q: &self.q + &other.q,
        }
    }
}

impl<T> AddAssign<&DualQuaternion<T>> for DualQuaternion<T>
where
    for<'a> T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, other: &DualQuaternion<T>) {
        self.p += &other.p;
        self.q += &other.q;
    }
}

impl<T> Sub for &DualQuaternion<T>
where
    for<'a> &'a T: Sub<Output = T>,
{
    type Output = DualQuaternion<T>;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            p: &self.p - &other.p,
            q: &self.q - &other.q,
        }
    }
}

impl<T> SubAssign<&DualQuaternion<T>> for DualQuaternion<T>
where
    for<'a> T: SubAssign<&'a T>,
{
    fn sub_assign(&mut self, other: &DualQuaternion<T>) {
        self.p -= &other.p;
        self.q -= &other.q;
    }
}

impl<T> Mul for &DualQuaternion<T>
where
    for<'a> &'a T: Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type Output = DualQuaternion<T>;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            p: &self.p * &other.p,
            q: &(&self.p * &other.q) + &(&other.p * &self.q),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = &DualQuaternion::new(
            Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
            Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3),
        );
        let b = &DualQuaternion::new(
            Quaternion::new(Vector::new(5.3, 3.2, -10.98), 41.2),
            Quaternion::new(Vector::new(3.3, 4.9, -6.13), -9.34),
        );
        let c = &DualQuaternion::new(
            Quaternion::new(Vector::new(-6.23, -663.2, 1.1), -901.2),
            Quaternion::new(Vector::new(-943.1, 0.0, 3.4), -65.2),
        );
        assert_eq!(
            a + b,
            DualQuaternion::new(
                Quaternion::new(
                    Vector::new(3.8 + 5.3, -9.9 + 3.2, -0.84 - 10.98),
                    3.27 + 41.2,
                ),
                Quaternion::new(Vector::new(-1.2 + 3.3, -2.2 + 4.9, 64.3 - 6.13), 3.3 - 9.34),
            ),
        );
        assert_eq!(
            a + c,
            DualQuaternion::new(
                Quaternion::new(
                    Vector::new(3.8 - 6.23, -9.9 - 663.2, -0.84 + 1.1),
                    3.27 - 901.2,
                ),
                Quaternion::new(Vector::new(-1.2 - 943.1, -2.2, 64.3 + 3.4), 3.3 - 65.2),
            ),
        );
        assert_eq!(
            b + c,
            DualQuaternion::new(
                Quaternion::new(
                    Vector::new(5.3 - 6.23, 3.2 - 663.2, -10.98 + 1.1),
                    41.2 - 901.2,
                ),
                Quaternion::new(Vector::new(3.3 - 943.1, 4.9, -6.13 + 3.4), -9.34 - 65.2),
            ),
        );
    }

    #[test]
    fn add_assign() {
        let mut a = DualQuaternion::new(
            Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
            Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3),
        );
        a += &DualQuaternion::new(
            Quaternion::new(Vector::new(5.3, 3.2, -10.98), 41.2),
            Quaternion::new(Vector::new(3.3, 4.9, -6.13), -9.34),
        );
        assert_eq!(
            a,
            DualQuaternion::new(
                Quaternion::new(
                    Vector::new(3.8 + 5.3, -9.9 + 3.2, -0.84 - 10.98),
                    3.27 + 41.2,
                ),
                Quaternion::new(Vector::new(-1.2 + 3.3, -2.2 + 4.9, 64.3 - 6.13), 3.3 - 9.34),
            ),
        );
        a += &DualQuaternion::new(
            Quaternion::new(Vector::new(-6.23, -663.2, 1.1), -901.2),
            Quaternion::new(Vector::new(-943.1, 0.0, 3.4), -65.2),
        );
        assert_eq!(
            a,
            DualQuaternion::new(
                Quaternion::new(
                    Vector::new(3.8 + 5.3 - 6.23, -9.9 + 3.2 - 663.2, -0.84 - 10.98 + 1.1),
                    3.27 + 41.2 - 901.2,
                ),
                Quaternion::new(
                    Vector::new(-1.2 + 3.3 - 943.1, -2.2 + 4.9, 64.3 - 6.13 + 3.4),
                    3.3 - 9.34 - 65.2,
                ),
            ),
        );
    }

    #[test]
    fn sub() {
        let a = &DualQuaternion::new(
            Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
            Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3),
        );
        let b = &DualQuaternion::new(
            Quaternion::new(Vector::new(5.3, 3.2, -10.98), 41.2),
            Quaternion::new(Vector::new(3.3, 4.9, -6.13), -9.34),
        );
        let c = &DualQuaternion::new(
            Quaternion::new(Vector::new(-6.23, -663.2, 1.1), -901.2),
            Quaternion::new(Vector::new(-943.1, 0.0, 3.4), -65.2),
        );
        assert_eq!(
            a - b,
            DualQuaternion::new(
                Quaternion::new(
                    Vector::new(3.8 - 5.3, -9.9 - 3.2, -0.84 + 10.98),
                    3.27 - 41.2,
                ),
                Quaternion::new(Vector::new(-1.2 - 3.3, -2.2 - 4.9, 64.3 + 6.13), 3.3 + 9.34),
            ),
        );
        assert_eq!(
            a - c,
            DualQuaternion::new(
                Quaternion::new(
                    Vector::new(3.8 + 6.23, -9.9 + 663.2, -0.84 - 1.1),
                    3.27 + 901.2,
                ),
                Quaternion::new(Vector::new(-1.2 + 943.1, -2.2, 64.3 - 3.4), 3.3 + 65.2),
            ),
        );
        assert_eq!(
            b - c,
            DualQuaternion::new(
                Quaternion::new(
                    Vector::new(5.3 + 6.23, 3.2 + 663.2, -10.98 - 1.1),
                    41.2 + 901.2,
                ),
                Quaternion::new(Vector::new(3.3 + 943.1, 4.9, -6.13 - 3.4), -9.34 + 65.2),
            ),
        );
    }

    #[test]
    fn sub_assign() {
        let mut a = DualQuaternion::new(
            Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
            Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3),
        );
        a -= &DualQuaternion::new(
            Quaternion::new(Vector::new(5.3, 3.2, -10.98), 41.2),
            Quaternion::new(Vector::new(3.3, 4.9, -6.13), -9.34),
        );
        assert_eq!(
            a,
            DualQuaternion::new(
                Quaternion::new(
                    Vector::new(3.8 - 5.3, -9.9 - 3.2, -0.84 + 10.98),
                    3.27 - 41.2,
                ),
                Quaternion::new(Vector::new(-1.2 - 3.3, -2.2 - 4.9, 64.3 + 6.13), 3.3 + 9.34),
            ),
        );
        a -= &DualQuaternion::new(
            Quaternion::new(Vector::new(-6.23, -663.2, 1.1), -901.2),
            Quaternion::new(Vector::new(-943.1, 0.0, 3.4), -65.2),
        );
        assert_eq!(
            a,
            DualQuaternion::new(
                Quaternion::new(
                    Vector::new(3.8 - 5.3 + 6.23, -9.9 - 3.2 + 663.2, -0.84 + 10.98 - 1.1),
                    3.27 - 41.2 + 901.2,
                ),
                Quaternion::new(
                    Vector::new(-1.2 - 3.3 + 943.1, -2.2 - 4.9, 64.3 + 6.13 - 3.4),
                    3.3 + 9.34 + 65.2,
                ),
            ),
        );
    }

    #[test]
    fn mul() {
        let a = &DualQuaternion::new(
            Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
            Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3),
        );
        let b = &DualQuaternion::new(
            Quaternion::new(Vector::new(5.3, 3.2, -10.98), 41.2),
            Quaternion::new(Vector::new(3.3, 4.9, -6.13), -9.34),
        );
        assert_eq!(
            a * b,
            DualQuaternion::new(
                &Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27)
                    * &Quaternion::new(Vector::new(5.3, 3.2, -10.98), 41.2),
                &(&Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27)
                    * &Quaternion::new(Vector::new(3.3, 4.9, -6.13), -9.34))
                    + &(&Quaternion::new(Vector::new(5.3, 3.2, -10.98), 41.2)
                        * &Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3)),
            ),
        );
        assert_eq!(
            b * a,
            DualQuaternion::new(
                &Quaternion::new(Vector::new(5.3, 3.2, -10.98), 41.2)
                    * &Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
                &(&Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27)
                    * &Quaternion::new(Vector::new(3.3, 4.9, -6.13), -9.34))
                    + &(&Quaternion::new(Vector::new(5.3, 3.2, -10.98), 41.2)
                        * &Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3)),
            ),
        );
    }

    #[test]
    fn conj() {
        let a = DualQuaternion::new(
            Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
            Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3),
        );
        assert_eq!(
            a.conj_from_dual_number(),
            DualQuaternion::new(
                Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
                Quaternion::new(Vector::new(1.2, 2.2, -64.3), -3.3),
            )
        );
        assert_eq!(
            a.conj_from_quaternion(),
            DualQuaternion::new(
                Quaternion::new(Vector::new(-3.8, 9.9, 0.84), 3.27),
                Quaternion::new(Vector::new(1.2, 2.2, -64.3), 3.3),
            )
        );
        assert_eq!(
            a.conj_from_dual_number_and_quaternion(),
            DualQuaternion::new(
                Quaternion::new(Vector::new(-3.8, 9.9, 0.84), 3.27),
                Quaternion::new(Vector::new(-1.2, -2.2, 64.3), -3.3),
            )
        );
    }

    #[test]
    fn dot() {
        let a = DualQuaternion::new(
            Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
            Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3),
        );
        let b = &DualQuaternion::new(
            Quaternion::new(Vector::new(5.3, 3.2, -10.98), 41.2),
            Quaternion::new(Vector::new(3.3, 4.9, -6.13), -9.34),
        );
        assert_eq!(
            a.dot(b),
            3.8 * 5.3 - 9.9 * 3.2 + 0.84 * 10.98 + 3.27 * 41.2
                - 1.2 * 3.3
                - 2.2 * 4.9
                - 64.3 * 6.13
                - 3.3 * 9.34
        );
    }

    #[test]
    fn from_translation_and_rotation() {
        assert_eq!(
            DualQuaternion::<f64>::from_translation_and_rotation(
                &Vector::new(4.2, 3.1, -10.6),
                &Vector::new(0.3, -0.4, 0.866),
                &-0.13,
            ),
            DualQuaternion::new(
                Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
                Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3),
            )
        );
    }

    #[test]
    fn from_rotation_and_translation() {
        assert_eq!(
            DualQuaternion::<f64>::from_rotation_and_translation(
                &Vector::new(0.3, -0.4, 0.866),
                &-0.13,
                &Vector::new(4.2, 3.1, -10.6),
            ),
            DualQuaternion::new(
                Quaternion::new(Vector::new(3.8, -9.9, -0.84), 3.27),
                Quaternion::new(Vector::new(-1.2, -2.2, 64.3), 3.3),
            )
        );
    }
}