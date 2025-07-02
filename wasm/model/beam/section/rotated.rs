use super::Section;
use crate::Float;
use std::array::from_fn;

pub struct RotatedSection<T: Section> {
    origin: T,
    angle: Float,
}

impl<T: Section> RotatedSection<T> {
    pub const fn new(section: T, angle: Float) -> Self {
        Self {
            origin: section,
            angle,
        }
    }
}

impl<T: Section> Section for RotatedSection<T> {
    fn area(&self) -> Float {
        self.origin.area()
    }
    fn centroid(&self) -> [Float; 2] {
        let cos = self.angle.cos();
        let sin = self.angle.sin();
        let c = self.origin.centroid();
        let mut i = c
            .iter()
            .rev()
            .map(|t| t * cos)
            .zip(c.iter().map(|t| t * sin))
            .enumerate()
            .map(|(n, (a, b))| a + b * [-1.0].iter().cycle().take(n).product::<Float>())
            .rev();
        from_fn(|_| i.next().unwrap())
    }
    fn moment_of_inertia(&self) -> [Float; 2] {
        let a2 = self.angle * -2.0;
        let cos = a2.cos() * 0.5;
        let sin = a2.sin();
        let [jy, jx] = self.origin.moment_of_inertia();
        let jxy = self.origin.product_of_inertia();
        from_fn(|n| {
            let s = [-1.0].iter().cycle().take(n).product::<Float>();
            let mut t = [(jy + jx) * 0.5, (jy - jx) * cos * s, jxy * sin * s];
            t.sort_by(|a, b| a.abs().total_cmp(&b.abs()));
            t.iter().sum()
        })
    }
    fn product_of_inertia(&self) -> Float {
        let a2 = self.angle * -2.0;
        let [jy, jx] = self.origin.moment_of_inertia();
        (jx - jy) * a2.sin() * 0.5 + self.origin.product_of_inertia() * a2.cos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Mock origin section representing a rectangle section.
    // width: 4.9; height: 8.1; centroid: [2.2, 3.1];
    struct Origin {}
    impl Section for Origin {
        fn area(&self) -> Float {
            4.9 * 8.1
        }
        fn centroid(&self) -> [Float; 2] {
            [2.2, 3.1]
        }
        fn moment_of_inertia(&self) -> [Float; 2] {
            [
                8.1 * 4.9 * 4.9 * 4.9 / 12.0 + 2.2 * 2.2 * 4.9 * 8.1,
                4.9 * 8.1 * 8.1 * 8.1 / 12.0 + 3.1 * 3.1 * 4.9 * 8.1,
            ]
        }
        fn product_of_inertia(&self) -> Float {
            4.9 * 8.1 * 2.2 * 3.1
        }
    }
    #[test]
    fn area() {
        let s = RotatedSection::new(Origin {}, 0.72);
        assert_eq!(s.area(), 4.9 * 8.1);
    }
    #[test]
    fn centroid() {
        let s = RotatedSection::new(Origin {}, 0.92);
        let r = (2.2 as Float).hypot(3.1);
        let theta = (3.1 as Float).atan2(2.2) + 0.92;
        assert_eq!(s.centroid(), [r * theta.cos(), r * theta.sin()]);
    }
    #[test]
    fn moment_of_inertia() {
        let s = RotatedSection::new(Origin {}, 0.67);
        let r = (2.2 as Float).hypot(3.1);
        let theta = (3.1 as Float).atan2(2.2) + 0.67;
        let x = r * theta.cos();
        let y = r * theta.sin();
        assert_eq!(
            s.moment_of_inertia(),
            [
                (8.1 * 4.9 * 4.9 * 4.9 + 4.9 * 8.1 * 8.1 * 8.1
                    - (4.9 * 8.1 * 8.1 * 8.1 - 8.1 * 4.9 * 4.9 * 4.9)
                        * (0.67 * 2.0 as Float).cos())
                    / 24.0
                    + x * x * 4.9 * 8.1,
                (8.1 * 4.9 * 4.9 * 4.9
                    + 4.9 * 8.1 * 8.1 * 8.1
                    + (4.9 * 8.1 * 8.1 * 8.1 - 8.1 * 4.9 * 4.9 * 4.9)
                        * (0.67 * 2.0 as Float).cos())
                    / 24.0
                    + y * y * 4.9 * 8.1,
            ]
        );
    }
    #[test]
    fn product_of_inertia() {
        let s = RotatedSection::new(Origin {}, 0.72);
        let r2 = 2.2 * 2.2 + 3.1 * 3.1;
        let theta = (3.1 as Float).atan2(2.2) + 0.72;
        assert_eq!(
            s.product_of_inertia(),
            -(4.9 * 8.1 * 8.1 * 8.1 - 8.1 * 4.9 * 4.9 * 4.9) / 24.0 * (1.44 as Float).sin()
                + 4.9 * 8.1 * r2 * theta.cos() * theta.sin()
        );
    }
}
