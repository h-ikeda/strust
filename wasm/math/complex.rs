use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::traits::{Atan2, Cos, Exp, Hypot, Ln, Sin};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

impl<T> From<T> for Complex<T>
where
    T: Default,
{
    fn from(value: T) -> Self {
        Self {
            re: value,
            im: T::default(),
        }
    }
}

impl<T> Complex<T>
where
    T: Hypot,
{
    pub fn abs(&self) -> T {
        self.re.hypot(&self.im)
    }
}

impl<T> Complex<T>
where
    for<'a> &'a T: Neg<Output = T>,
    T: Clone,
{
    pub fn conj(&self) -> Self {
        Self {
            re: self.re.clone(),
            im: -&self.im,
        }
    }
}

impl<T> Complex<T>
where
    T: Cos + Sin + Exp,
    for<'a> &'a T: Mul<Output = T>,
{
    pub fn exp(&self) -> Self {
        Self {
            re: &self.re.exp() * &self.im.cos(),
            im: &self.re.exp() * &self.im.sin(),
        }
    }
}

impl<T> Complex<T>
where
    T: Atan2,
{
    pub fn arg(&self) -> T {
        self.im.atan2(&self.re)
    }
}

impl<T> Complex<T>
where
    T: Ln + Atan2 + Hypot,
{
    pub fn ln(&self) -> Self {
        Self {
            re: self.abs().ln(),
            im: self.arg(),
        }
    }
}

impl<T> Complex<T>
where
    T: Sin + Cos + Exp + Ln + Atan2 + Hypot,
    for<'a> &'a T: Mul<Output = T> + Sub<Output = T> + Add<Output = T>,
{
    pub fn pow(&self, rhs: &Self) -> Self {
        (rhs * &self.ln()).exp()
    }
}

impl<T> Add for &Complex<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            re: &self.re + &rhs.re,
            im: &self.im + &rhs.im,
        }
    }
}

impl<T> Add<&T> for &Complex<T>
where
    for<'a> &'a T: Add<Output = T>,
    T: Clone,
{
    type Output = Complex<T>;
    fn add(self, rhs: &T) -> Self::Output {
        Self::Output {
            re: &self.re + rhs,
            im: self.im.clone(),
        }
    }
}

impl<T> AddAssign<&Complex<T>> for Complex<T>
where
    for<'a> T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, rhs: &Complex<T>) {
        self.re += &rhs.re;
        self.im += &rhs.im;
    }
}

impl<T> AddAssign<&T> for Complex<T>
where
    for<'a> T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, rhs: &T) {
        self.re += rhs;
    }
}

impl<T> Sub for &Complex<T>
where
    for<'a> &'a T: Sub<Output = T>,
{
    type Output = Complex<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            re: &self.re - &rhs.re,
            im: &self.im - &rhs.im,
        }
    }
}

impl<T> Sub<&T> for &Complex<T>
where
    for<'a> &'a T: Sub<Output = T>,
    T: Clone,
{
    type Output = Complex<T>;
    fn sub(self, rhs: &T) -> Self::Output {
        Self::Output {
            re: &self.re - rhs,
            im: self.im.clone(),
        }
    }
}

impl<T> SubAssign<&Complex<T>> for Complex<T>
where
    for<'a> T: SubAssign<&'a T>,
{
    fn sub_assign(&mut self, rhs: &Complex<T>) {
        self.re -= &rhs.re;
        self.im -= &rhs.im;
    }
}

impl<T> SubAssign<&T> for Complex<T>
where
    for<'a> T: SubAssign<&'a T>,
{
    fn sub_assign(&mut self, rhs: &T) {
        self.re -= rhs;
    }
}

impl<T> Mul for &Complex<T>
where
    for<'a> &'a T: Mul<Output = T> + Sub<Output = T> + Add<Output = T>,
{
    type Output = Complex<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            re: &(&self.re * &rhs.re) - &(&self.im * &rhs.im),
            im: &(&self.re * &rhs.im) + &(&self.im * &rhs.re),
        }
    }
}

impl<T> Mul<&T> for &Complex<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Complex<T>;
    fn mul(self, rhs: &T) -> Self::Output {
        Self::Output {
            re: &self.re * rhs,
            im: &self.im * rhs,
        }
    }
}

impl<T> MulAssign<&Complex<T>> for Complex<T>
where
    for<'a> T: MulAssign<&'a T> + SubAssign<&'a T> + AddAssign<&'a T>,
    for<'a> &'a T: Mul<Output = T>,
{
    fn mul_assign(&mut self, rhs: &Complex<T>) {
        // (a + bi)(c + di) = (ac - bd) + (bc + ad)i
        let ad = &self.re * &rhs.im;
        self.re *= &rhs.re;
        self.re -= &(&self.im * &rhs.im);
        self.im *= &rhs.re;
        self.im += &ad;
    }
}

impl<T> MulAssign<&T> for Complex<T>
where
    for<'a> T: MulAssign<&'a T>,
{
    fn mul_assign(&mut self, rhs: &T) {
        self.re *= rhs;
        self.im *= rhs;
    }
}

impl<T> Div for &Complex<T>
where
    for<'a> &'a T: Div<Output = T> + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type Output = Complex<T>;
    fn div(self, rhs: Self) -> Self::Output {
        let denominator = &(&rhs.re * &rhs.re) + &(&rhs.im * &rhs.im);
        Self::Output {
            re: &(&(&self.re * &rhs.re) + &(&self.im * &rhs.im)) / &denominator,
            im: &(&(&self.im * &rhs.re) - &(&self.re * &rhs.im)) / &denominator,
        }
    }
}

impl<T> Div<&T> for &Complex<T>
where
    for<'a> &'a T: Div<Output = T>,
{
    type Output = Complex<T>;
    fn div(self, rhs: &T) -> Self::Output {
        Self::Output {
            re: &self.re / rhs,
            im: &self.im / rhs,
        }
    }
}

impl<T> DivAssign<&Complex<T>> for Complex<T>
where
    for<'a> &'a T: Add<Output = T> + Mul<Output = T>,
    for<'a> T: MulAssign<&'a T> + AddAssign<&'a T> + SubAssign<&'a T> + DivAssign<&'a T>,
{
    fn div_assign(&mut self, rhs: &Complex<T>) {
        // (a + bi)/(c + di) = (ac + bd)/(c^2 + d^2) + (bc - ad)/(c^2 + d^2)
        let denominator = &(&rhs.re * &rhs.re) + &(&rhs.im * &rhs.im);
        let ad = &self.re * &rhs.im;
        self.re *= &rhs.re;
        self.re += &(&self.im * &rhs.im);
        self.re /= &denominator;
        self.im *= &rhs.re;
        self.im -= &ad;
        self.im /= &denominator;
    }
}

impl<T> DivAssign<&T> for Complex<T>
where
    for<'a> T: DivAssign<&'a T>,
{
    fn div_assign(&mut self, rhs: &T) {
        self.re /= rhs;
        self.im /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use core::{f32, f64};

    use super::*;

    #[test]
    fn from() {
        assert_eq!(Complex::from(-3), Complex::new(-3, 0));
        assert_eq!(Complex::from(3.3), Complex::new(3.3, 0.0));
    }

    #[test]
    fn abs() {
        let a = Complex::new(8.1, -6.2);
        let b = Complex::new(4.1, 1.2);
        assert_eq!(a.abs(), (8.1 as f64).hypot(6.2));
        assert_eq!(b.abs(), (4.1 as f64).hypot(1.2));
    }

    #[test]
    fn conj() {
        let a = Complex::new(83.61, 24.83);
        let b = Complex::new(34, -11);
        assert_eq!(a.conj(), Complex::new(83.61, -24.83));
        assert_eq!(b.conj(), Complex::new(34, 11));
    }

    #[test]
    fn add_complex() {
        let a = Complex::new(8, 41);
        let b = Complex::new(-32, 12);
        assert_eq!(&a + &b, Complex::new(-24, 53));
        let c = Complex::new(56.3, -33.8);
        let d = Complex::new(-3.98, 12.6);
        assert_eq!(&c + &d, Complex::new(56.3 - 3.98, -33.8 + 12.6));
    }

    #[test]
    fn add_scalar() {
        let a = Complex::new(8, 41);
        assert_eq!(&a + &9, Complex::new(17, 41));
        let b = Complex::new(56.3, -33.8);
        assert_eq!(&b + &22.8, Complex::new(56.3 + 22.8, -33.8));
    }

    #[test]
    fn add_assign_complex() {
        let mut a = Complex::new(8, 41);
        let b = Complex::new(-32, 12);
        a += &b;
        assert_eq!(a, Complex::new(-24, 53));
        a += &b;
        assert_eq!(a, Complex::new(-56, 65));
        let mut c = Complex::new(56.3, -33.8);
        let d = Complex::new(-3.98, 12.6);
        c += &d;
        assert_eq!(c, Complex::new(56.3 - 3.98, -33.8 + 12.6));
        c += &d;
        assert_eq!(c, Complex::new(56.3 - 3.98 - 3.98, -33.8 + 12.6 + 12.6));
    }

    #[test]
    fn add_assign_scalar() {
        let mut a = Complex::new(8, 41);
        a += &9;
        assert_eq!(a, Complex::new(17, 41));
        a += &16;
        assert_eq!(a, Complex::new(33, 41));
        let mut b = Complex::new(56.3, -33.8);
        b += &22.8;
        assert_eq!(b, Complex::new(56.3 + 22.8, -33.8));
        b += &6.18;
        assert_eq!(b, Complex::new(56.3 + 22.8 + 6.18, -33.8));
    }

    #[test]
    fn sub_complex() {
        let a = Complex::new(8, 41);
        let b = Complex::new(-32, 12);
        assert_eq!(&a - &b, Complex::new(40, 29));
        let c = Complex::new(56.3, -33.8);
        let d = Complex::new(-3.98, 12.6);
        assert_eq!(&c - &d, Complex::new(56.3 + 3.98, -33.8 - 12.6));
    }

    #[test]
    fn sub_scalar() {
        let a = Complex::new(8, 41);
        assert_eq!(&a - &9, Complex::new(-1, 41));
        let b = Complex::new(56.3, -33.8);
        assert_eq!(&b - &22.8, Complex::new(56.3 - 22.8, -33.8));
    }

    #[test]
    fn sub_assign_complex() {
        let mut a = Complex::new(8, 41);
        let b = Complex::new(-32, 12);
        a -= &b;
        assert_eq!(a, Complex::new(40, 29));
        a -= &b;
        assert_eq!(a, Complex::new(72, 17));
        let mut c = Complex::new(56.3, -33.8);
        let d = Complex::new(-3.98, 12.6);
        c -= &d;
        assert_eq!(c, Complex::new(56.3 + 3.98, -33.8 - 12.6));
        c -= &d;
        assert_eq!(c, Complex::new(56.3 + 3.98 + 3.98, -33.8 - 12.6 - 12.6));
    }

    #[test]
    fn sub_assign_scalar() {
        let mut a = Complex::new(8, 41);
        a -= &9;
        assert_eq!(a, Complex::new(-1, 41));
        a -= &16;
        assert_eq!(a, Complex::new(-17, 41));
        let mut b = Complex::new(56.3, -33.8);
        b -= &22.8;
        assert_eq!(b, Complex::new(56.3 - 22.8, -33.8));
        b -= &6.18;
        assert_eq!(b, Complex::new(56.3 - 22.8 - 6.18, -33.8));
    }

    #[test]
    fn mul_complex() {
        let a = Complex::new(8, 41);
        let b = Complex::new(-32, 12);
        assert_eq!(&a * &b, Complex::new(-748, -1216));
        let c = Complex::new(56.3, -33.8);
        let d = Complex::new(-3.98, 12.6);
        assert_eq!(
            &c * &d,
            Complex::new(-56.3 * 3.98 + 33.8 * 12.6, 33.8 * 3.98 + 56.3 * 12.6),
        );
    }

    #[test]
    fn mul_scalar() {
        let a = Complex::new(8, 41);
        assert_eq!(&a * &9, Complex::new(72, 369));
        let b = Complex::new(56.3, -33.8);
        assert_eq!(&b * &22.8, Complex::new(56.3 * 22.8, -33.8 * 22.8));
    }

    #[test]
    fn mul_assign_complex() {
        let mut a = Complex::new(8, 41);
        let b = Complex::new(-32, 12);
        a *= &b;
        assert_eq!(a, Complex::new(-748, -1216));
        a *= &b;
        assert_eq!(a, Complex::new(38528, 29936));
        let mut c = Complex::new(56.3, -33.8);
        let d = Complex::new(-3.98, 12.6);
        c *= &d;
        assert_eq!(
            c,
            Complex::new(-56.3 * 3.98 + 33.8 * 12.6, 33.8 * 3.98 + 56.3 * 12.6),
        );
        c *= &d;
        assert_eq!(
            c,
            Complex::new(
                (56.3 * 3.98 - 33.8 * 12.6) * 3.98 - (33.8 * 3.98 + 56.3 * 12.6) * 12.6,
                (33.8 * 3.98 + 56.3 * 12.6) * -3.98 + (-56.3 * 3.98 + 33.8 * 12.6) * 12.6,
            ),
        );
    }

    #[test]
    fn mul_assign_scalar() {
        let mut a = Complex::new(8, 41);
        a *= &9;
        assert_eq!(a, Complex::new(72, 369));
        a *= &11;
        assert_eq!(a, Complex::new(792, 4059));
        let mut b = Complex::new(56.3, -33.8);
        b *= &22.8;
        assert_eq!(b, Complex::new(56.3 * 22.8, -33.8 * 22.8));
        b *= &-10.9;
        assert_eq!(b, Complex::new(56.3 * 22.8 * -10.9, -33.8 * 22.8 * -10.9));
    }

    #[test]
    fn div_by_complex() {
        let a = Complex::new(-748, -1216);
        let b = Complex::new(-32, 12);
        assert_eq!(&a / &b, Complex::new(8, 41));
        let c = Complex::new(201.8, 843.9);
        let d = Complex::new(-3.98, 12.6);
        assert_eq!(
            &c / &d,
            Complex::new(
                (-201.8 * 3.98 + 843.9 * 12.6) / (3.98 * 3.98 + 12.6 * 12.6),
                (-843.9 * 3.98 - 201.8 * 12.6) / (3.98 * 3.98 + 12.6 * 12.6),
            ),
        );
    }

    #[test]
    fn div_by_scalar() {
        let a = Complex::new(72, 369);
        assert_eq!(&a / &9, Complex::new(8, 41));
        let b = Complex::new(56.3, -33.8);
        assert_eq!(&b / &22.8, Complex::new(56.3 / 22.8, -33.8 / 22.8));
    }

    #[test]
    fn div_assign_by_complex() {
        let mut a = Complex::new(-748, -1216);
        let b = Complex::new(-32, 12);
        a /= &b;
        assert_eq!(a, Complex::new(8, 41));
        let mut c = Complex::new(201.8, 843.9);
        let d = Complex::new(-3.98, 12.6);
        c /= &d;
        assert_eq!(
            c,
            Complex::new(
                (-201.8 * 3.98 + 843.9 * 12.6) / (3.98 * 3.98 + 12.6 * 12.6),
                (-843.9 * 3.98 - 201.8 * 12.6) / (3.98 * 3.98 + 12.6 * 12.6),
            ),
        );
    }

    #[test]
    fn div_assign_by_scalar() {
        let mut a = Complex::new(72, 369);
        a /= &9;
        assert_eq!(a, Complex::new(8, 41));
        let mut b = Complex::new(56.3, -33.8);
        b /= &22.8;
        assert_eq!(b, Complex::new(56.3 / 22.8, -33.8 / 22.8));
    }

    #[test]
    fn exp() {
        let a = Complex::new(56.3, -33.8);
        assert_eq!(
            a.exp(),
            Complex::new(56.3.exp() * 33.8.cos(), -56.3.exp() * 33.8.sin()),
        );
    }

    #[test]
    fn arg() {
        let a = Complex::new(56.3, -33.8);
        assert_eq!(a.arg(), (-33.8 as f32).atan2(56.3));
        let b = Complex::new(56.3, 0.0);
        assert_eq!(b.arg(), 0.0);
        let c = Complex::new(0.0, -33.8);
        assert_eq!(c.arg(), -f64::consts::PI / 2.0);
        let d = Complex::new(-56.3, 33.8);
        assert_eq!(d.arg(), (-33.8 as f32).atan2(56.3) + f32::consts::PI);
        let e = Complex::new(-56.3, 0.0);
        assert_eq!(e.arg(), f32::consts::PI);
        let f = Complex::new(0.0, 0.0);
        assert_eq!(f.arg(), 0.0);
    }

    #[test]
    fn ln() {
        let a = Complex::new(56.3, -33.8);
        assert_eq!(
            a.ln(),
            Complex::new((56.3 as f32).hypot(33.8).ln(), -(33.8 as f32).atan2(56.3)),
        );
        let b = Complex::new(1.0, 0.0);
        assert_eq!(b.ln(), Complex::new(0.0, 0.0));
    }

    #[test]
    fn pow() {
        let a = Complex::new(56.3, -33.8);
        let b = Complex::new(-3.1, 4.41);
        assert_eq!(
            a.pow(&b),
            Complex::new(
                (4.41 * (33.8 as f64).atan2(56.3) - 3.1 * (56.3 as f64).hypot(33.8).ln()).exp()
                    * (4.41 * (56.3 as f64).hypot(33.8).ln() + 3.1 * (33.8 as f64).atan2(56.3))
                        .cos(),
                (4.41 * (33.8 as f64).atan2(56.3) - 3.1 * (56.3 as f64).hypot(33.8).ln()).exp()
                    * (4.41 * (56.3 as f64).hypot(33.8).ln() + 3.1 * (33.8 as f64).atan2(56.3))
                        .sin(),
            ),
        );
    }

    #[test]
    fn default() {
        assert_eq!(Complex::default(), Complex::new(0.0, 0.0));
        assert_eq!(Complex::default(), Complex::new(0, 0));
    }
}
