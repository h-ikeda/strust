use super::Section;
use crate::Float;
use std::array::from_fn;

#[derive(Debug, Clone)]
pub struct CircleSection {
    pub radius: Float,
    pub offset: [Float; 2],
}

impl CircleSection {
    pub const fn new(radius: Float, offset: [Float; 2]) -> Self {
        Self { radius, offset }
    }
}

impl Section for CircleSection {
    fn area(&self) -> Float {
        self.radius * self.radius * (180.0 as Float).to_radians()
    }
    fn centroid(&self) -> [Float; 2] {
        self.offset
    }
    fn moment_of_inertia(&self) -> [Float; 2] {
        from_fn(|i| {
            (self.radius * self.radius + self.offset[i] * self.offset[i] * 4.0)
                * self.radius
                * self.radius
                * (45.0 as Float).to_radians()
        })
    }
    fn product_of_inertia(&self) -> Float {
        self.radius
            * self.radius
            * (180.0 as Float).to_radians()
            * self.offset.iter().product::<Float>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod area {
        use super::*;
        #[test]
        fn positive_radius() {
            let s = CircleSection::new(3.2, [0.8, -0.1]);
            assert_eq!(s.area(), 3.2 * 3.2 * (180.0 as Float).to_radians());
        }
        #[test]
        fn negative_radius() {
            let s = CircleSection::new(-3.3, [-0.8, 0.1]);
            assert_eq!(s.area(), 3.3 * 3.3 * (180.0 as Float).to_radians());
        }
    }
    mod centroid {
        use super::*;
        #[test]
        fn positive_radius() {
            let s = CircleSection::new(3.2, [0.8, -0.1]);
            assert_eq!(s.centroid(), [0.8, -0.1]);
        }
        #[test]
        fn negative_radius() {
            let s = CircleSection::new(-3.3, [-0.8, 0.1]);
            assert_eq!(s.centroid(), [-0.8, 0.1]);
        }
    }
    mod moment_of_inertia {
        use super::*;
        #[test]
        fn positive_radius() {
            let s = CircleSection::new(3.2, [0.8, -0.1]);
            assert_eq!(
                format!("{:.13?}", s.moment_of_inertia()),
                format!(
                    "{:.13?}",
                    [
                        6.4 * 6.4 * 6.4 * 6.4 * (180.0 as Float).to_radians() / 64.0
                            + 3.2 * 3.2 * (180.0 as Float).to_radians() * 0.8 * 0.8,
                        6.4 * 6.4 * 6.4 * 6.4 * (180.0 as Float).to_radians() / 64.0
                            + 3.2 * 3.2 * (180.0 as Float).to_radians() * 0.1 * 0.1,
                    ]
                )
            );
        }
        #[test]
        fn negative_radius() {
            let s = CircleSection::new(-3.3, [-0.8, 0.1]);
            assert_eq!(
                format!("{:.12?}", s.moment_of_inertia()),
                format!(
                    "{:.12?}",
                    [
                        6.6 * 6.6 * 6.6 * 6.6 * (180.0 as Float).to_radians() / 64.0
                            + 3.3 * 3.3 * (180.0 as Float).to_radians() * 0.8 * 0.8,
                        6.6 * 6.6 * 6.6 * 6.6 * (180.0 as Float).to_radians() / 64.0
                            + 3.3 * 3.3 * (180.0 as Float).to_radians() * 0.1 * 0.1,
                    ]
                )
            );
        }
    }
    mod product_of_inertia {
        use super::*;
        #[test]
        fn positive_radius() {
            let s = CircleSection::new(3.2, [-1.1, 8.2]);
            assert_eq!(
                s.product_of_inertia(),
                -3.2 * 3.2 * (180.0 as Float).to_radians() * 1.1 * 8.2
            );
        }
        #[test]
        fn negative_radius() {
            let s = CircleSection::new(-3.3, [1.1, -8.2]);
            assert_eq!(
                s.product_of_inertia(),
                -3.3 * 3.3 * (180.0 as Float).to_radians() * 1.1 * 8.2
            );
        }
    }
}
