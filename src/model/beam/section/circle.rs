use super::Section;
use crate::Float;

#[derive(Debug, Clone)]
pub struct CircleSection {
    pub radius: Float,
}

impl CircleSection {
    pub const fn new(radius: Float) -> Self {
        Self { radius }
    }
}

impl Section for CircleSection {
    fn area(&self) -> Float {
        self.radius * self.radius * (180.0 as Float).to_radians()
    }
    fn centroid(&self) -> [Float; 2] {
        [Float::default(); 2]
    }
    fn moment_of_inertia(&self) -> [Float; 2] {
        [self.radius * self.radius * self.radius * self.radius * (45.0 as Float).to_radians(); 2]
    }
    fn product_of_inertia(&self) -> Float {
        Float::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod area {
        use super::*;
        #[test]
        fn positive_radius() {
            let s = CircleSection::new(3.2);
            assert_eq!(s.area(), 3.2 * 3.2 * (180.0 as Float).to_radians());
        }
        #[test]
        fn negative_radius() {
            let s = CircleSection::new(-3.3);
            assert_eq!(s.area(), 3.3 * 3.3 * (180.0 as Float).to_radians());
        }
    }
    mod centroid {
        use super::*;
        #[test]
        fn positive_radius() {
            let s = CircleSection::new(3.2);
            assert_eq!(s.centroid(), [0.0, 0.0]);
        }
        #[test]
        fn negative_radius() {
            let s = CircleSection::new(-3.3);
            assert_eq!(s.centroid(), [0.0, 0.0]);
        }
    }
    mod moment_of_inertia {
        use super::*;
        #[test]
        fn positive_radius() {
            let s = CircleSection::new(3.2);
            assert_eq!(
                s.moment_of_inertia(),
                [
                    6.4 * 6.4 * 6.4 * 6.4 * (180.0 as Float).to_radians() / 64.0,
                    6.4 * 6.4 * 6.4 * 6.4 * (180.0 as Float).to_radians() / 64.0,
                ]
            );
        }
        #[test]
        fn negative_radius() {
            let s = CircleSection::new(-3.3);
            assert_eq!(
                s.moment_of_inertia(),
                [
                    6.6 * 6.6 * 6.6 * 6.6 * (180.0 as Float).to_radians() / 64.0,
                    6.6 * 6.6 * 6.6 * 6.6 * (180.0 as Float).to_radians() / 64.0,
                ]
            );
        }
    }
    mod product_of_inertia {
        use super::*;
        #[test]
        fn positive_radius() {
            let s = CircleSection::new(3.2);
            assert_eq!(s.product_of_inertia(), 0.0);
        }
        #[test]
        fn negative_radius() {
            let s = CircleSection::new(-3.3);
            assert_eq!(s.product_of_inertia(), 0.0);
        }
    }
}
