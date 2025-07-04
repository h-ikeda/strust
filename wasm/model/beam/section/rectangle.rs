use super::Section;
use crate::Float;

#[derive(Debug, Clone)]
pub struct RectangleSection {
    pub size: [Float; 2],
}

impl RectangleSection {
    pub const fn new(size: [Float; 2]) -> Self {
        Self { size }
    }
}

impl Section for RectangleSection {
    fn area(&self) -> Float {
        self.size.iter().product::<Float>().abs()
    }
    fn centroid(&self) -> [Float; 2] {
        self.size.map(|v| v * 0.5)
    }
    fn moment_of_inertia(&self) -> [Float; 2] {
        let a = self.size.iter().product::<Float>().abs() / 3.0;
        self.size.map(|v| v * v * a)
    }
    fn product_of_inertia(&self) -> Float {
        let t = self.size.iter().product::<Float>();
        t.abs() * t * 0.25
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod area {
        use super::*;
        #[test]
        fn positive_width_positive_height() {
            let s = RectangleSection::new([3.2, 1.1]);
            assert_eq!(s.area(), 3.2 * 1.1);
        }
        #[test]
        fn positive_width_negative_height() {
            let s = RectangleSection::new([3.2, -1.1]);
            assert_eq!(s.area(), 3.2 * 1.1);
        }
        #[test]
        fn negative_width_positive_height() {
            let s = RectangleSection::new([-3.2, 1.1]);
            assert_eq!(s.area(), 3.2 * 1.1);
        }
        #[test]
        fn negative_width_negative_height() {
            let s = RectangleSection::new([-3.2, -1.1]);
            assert_eq!(s.area(), 3.2 * 1.1);
        }
    }
    mod centroid {
        use super::*;
        #[test]
        fn positive_width_positive_height() {
            let s = RectangleSection::new([3.2, 1.1]);
            assert_eq!(s.centroid(), [3.2 * 0.5, 1.1 * 0.5]);
        }
        #[test]
        fn positive_width_negative_height() {
            let s = RectangleSection::new([3.2, -1.1]);
            assert_eq!(s.centroid(), [3.2 * 0.5, -1.1 * 0.5]);
        }
        #[test]
        fn negative_width_positive_height() {
            let s = RectangleSection::new([-3.2, 1.1]);
            assert_eq!(s.centroid(), [-3.2 * 0.5, 1.1 * 0.5]);
        }
        #[test]
        fn negative_width_negative_height() {
            let s = RectangleSection::new([-3.2, -1.1]);
            assert_eq!(s.centroid(), [-3.2 * 0.5, -1.1 * 0.5]);
        }
    }
    mod moment_of_inertia {
        use super::*;
        #[test]
        fn positive_width_positive_height() {
            let s = RectangleSection::new([3.3, 1.1]);
            assert_eq!(
                s.moment_of_inertia(),
                [
                    1.1 * 3.3 * 3.3 * 3.3 / 12.0 + 3.3 * 1.1 * 3.3 * 0.5 * 3.3 * 0.5,
                    3.3 * 1.1 * 1.1 * 1.1 / 12.0 + 3.3 * 1.1 * 1.1 * 0.5 * 1.1 * 0.5,
                ]
            );
        }
        #[test]
        fn positive_width_negative_height() {
            let s = RectangleSection::new([3.3, -1.1]);
            assert_eq!(
                s.moment_of_inertia(),
                [
                    1.1 * 3.3 * 3.3 * 3.3 / 12.0 + 3.3 * 1.1 * 3.3 * 0.5 * 3.3 * 0.5,
                    3.3 * 1.1 * 1.1 * 1.1 / 12.0 + 3.3 * 1.1 * 1.1 * 0.5 * 1.1 * 0.5,
                ]
            );
        }
        #[test]
        fn negative_width_positive_height() {
            let s = RectangleSection::new([-3.2, 1.1]);
            assert_eq!(
                format!("{:.14?}", s.moment_of_inertia()),
                format!(
                    "{:.14?}",
                    [
                        1.1 * 3.2 * 3.2 * 3.2 / 12.0 + 3.2 * 1.1 * 3.2 * 0.5 * 3.2 * 0.5,
                        3.2 * 1.1 * 1.1 * 1.1 / 12.0 + 3.2 * 1.1 * 1.1 * 0.5 * 1.1 * 0.5,
                    ]
                )
            );
        }
        #[test]
        fn negative_width_negative_height() {
            let s = RectangleSection::new([-3.2, -1.1]);
            assert_eq!(
                format!("{:.14?}", s.moment_of_inertia()),
                format!(
                    "{:.14?}",
                    [
                        1.1 * 3.2 * 3.2 * 3.2 / 12.0 + 3.2 * 1.1 * 3.2 * 0.5 * 3.2 * 0.5,
                        3.2 * 1.1 * 1.1 * 1.1 / 12.0 + 3.2 * 1.1 * 1.1 * 0.5 * 1.1 * 0.5,
                    ]
                )
            );
        }
    }
    mod product_of_inertia {
        use super::*;
        #[test]
        fn positive_width_positive_height() {
            let s = RectangleSection::new([3.3, 4.5]);
            assert_eq!(s.product_of_inertia(), 3.3 * 4.5 * 3.3 * 0.5 * 4.5 * 0.5);
        }
        #[test]
        fn positive_width_negative_height() {
            let s = RectangleSection::new([3.3, -4.5]);
            assert_eq!(s.product_of_inertia(), -3.3 * 4.5 * 3.3 * 0.5 * 4.5 * 0.5);
        }
        #[test]
        fn negative_width_positive_height() {
            let s = RectangleSection::new([-3.3, 4.5]);
            assert_eq!(s.product_of_inertia(), -3.3 * 4.5 * 3.3 * 0.5 * 4.5 * 0.5);
        }
        #[test]
        fn negative_width_negative_height() {
            let s = RectangleSection::new([-3.3, -4.5]);
            assert_eq!(s.product_of_inertia(), 3.3 * 4.5 * 3.3 * 0.5 * 4.5 * 0.5);
        }
    }
}
