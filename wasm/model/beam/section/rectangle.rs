use super::Section;
use crate::Float;
use std::array::from_fn;

#[derive(Debug, Clone)]
pub struct RectangleSection {
    pub size: [Float; 2],
    pub offset: [Float; 2],
}

impl RectangleSection {
    pub const fn new(size: [Float; 2], offset: [Float; 2]) -> Self {
        Self { size, offset }
    }
}

impl Section for RectangleSection {
    fn area(&self) -> Float {
        self.size.iter().product::<Float>().abs()
    }
    fn centroid(&self) -> [Float; 2] {
        from_fn(|i| self.offset[i] + self.size[i] * 0.5)
    }
    fn moment_of_inertia(&self) -> [Float; 2] {
        let a: Float = self.size.iter().product();
        from_fn(|i| {
            self.size[i] * self.size[i] / 3.0 + (self.size[i] + self.offset[i]) * self.offset[i]
        })
        .map(|v| (v * a).abs())
    }
    fn product_of_inertia(&self) -> Float {
        self.size
            .iter()
            .enumerate()
            .map(|(i, v)| v.abs() * (v * 0.5 + self.offset[i]))
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod area {
        use super::*;
        #[test]
        fn positive_width_positive_height() {
            let s = RectangleSection::new([3.2, 1.1], [0.8, -0.1]);
            assert_eq!(s.area(), 3.2 * 1.1);
        }
        #[test]
        fn positive_width_negative_height() {
            let s = RectangleSection::new([3.2, -1.1], [0.8, -0.1]);
            assert_eq!(s.area(), 3.2 * 1.1);
        }
        #[test]
        fn negative_width_positive_height() {
            let s = RectangleSection::new([-3.2, 1.1], [0.8, -0.1]);
            assert_eq!(s.area(), 3.2 * 1.1);
        }
        #[test]
        fn negative_width_negative_height() {
            let s = RectangleSection::new([-3.2, -1.1], [0.8, -0.1]);
            assert_eq!(s.area(), 3.2 * 1.1);
        }
    }
    mod centroid {
        use super::*;
        #[test]
        fn offset_forward() {
            let s = RectangleSection::new([3.2, -1.1], [0.8, -0.1]);
            assert_eq!(s.centroid(), [3.2 * 0.5 + 0.8, -1.1 * 0.5 - 0.1]);
        }
        #[test]
        fn offset_backward() {
            let s = RectangleSection::new([3.2, -1.1], [-0.8, 0.1]);
            assert_eq!(s.centroid(), [3.2 * 0.5 - 0.8, -1.1 * 0.5 + 0.1]);
        }
    }
    mod moment_of_inertia {
        use super::*;
        #[test]
        fn positive_width_positive_height() {
            let s = RectangleSection::new([3.2, 1.1], [0.8, -0.1]);
            assert_eq!(
                format!("{:.14?}", s.moment_of_inertia()),
                format!(
                    "{:.14?}",
                    [
                        1.1 * 3.2 * 3.2 * 3.2 / 12.0
                            + 3.2 * 1.1 * (3.2 * 0.5 + 0.8) * (3.2 * 0.5 + 0.8),
                        3.2 * 1.1 * 1.1 * 1.1 / 12.0
                            + 3.2 * 1.1 * (1.1 * 0.5 - 0.1) * (1.1 * 0.5 - 0.1),
                    ]
                )
            );
        }
        #[test]
        fn positive_width_negative_height() {
            let s = RectangleSection::new([3.2, -1.1], [-0.8, -0.1]);
            assert_eq!(
                format!("{:.15?}", s.moment_of_inertia()),
                format!(
                    "{:.15?}",
                    [
                        1.1 * 3.2 * 3.2 * 3.2 / 12.0
                            + 3.2 * 1.1 * (3.2 * 0.5 - 0.8) * (3.2 * 0.5 - 0.8),
                        3.2 * 1.1 * 1.1 * 1.1 / 12.0
                            + 3.2 * 1.1 * (1.1 * 0.5 + 0.1) * (1.1 * 0.5 + 0.1),
                    ]
                )
            );
        }
        #[test]
        fn negative_width_positive_height() {
            let s = RectangleSection::new([-3.2, 1.1], [0.8, -0.1]);
            assert_eq!(
                format!("{:.14?}", s.moment_of_inertia()),
                format!(
                    "{:.14?}",
                    [
                        1.1 * 3.2 * 3.2 * 3.2 / 12.0
                            + 3.2 * 1.1 * (3.2 * 0.5 - 0.8) * (3.2 * 0.5 - 0.8),
                        3.2 * 1.1 * 1.1 * 1.1 / 12.0
                            + 3.2 * 1.1 * (1.1 * 0.5 - 0.1) * (1.1 * 0.5 - 0.1),
                    ]
                )
            );
        }
        #[test]
        fn negative_width_negative_height() {
            let s = RectangleSection::new([-3.2, -1.1], [-0.8, 0.1]);
            assert_eq!(
                format!("{:.14?}", s.moment_of_inertia()),
                format!(
                    "{:.14?}",
                    [
                        1.1 * 3.2 * 3.2 * 3.2 / 12.0
                            + 3.2 * 1.1 * (3.2 * 0.5 + 0.8) * (3.2 * 0.5 + 0.8),
                        3.2 * 1.1 * 1.1 * 1.1 / 12.0
                            + 3.2 * 1.1 * (1.1 * 0.5 - 0.1) * (1.1 * 0.5 - 0.1),
                    ]
                )
            );
        }
    }
    mod product_of_inertia {
        use super::*;
        #[test]
        fn positive_width_positive_height() {
            let s = RectangleSection::new([3.3, 4.5], [-1.1, 8.2]);
            assert_eq!(
                s.product_of_inertia(),
                3.3 * 4.5 * (3.3 * 0.5 - 1.1) * (4.5 * 0.5 + 8.2)
            );
        }
        #[test]
        fn positive_width_negative_height() {
            let s = RectangleSection::new([3.3, -4.5], [-1.1, -8.2]);
            assert_eq!(
                s.product_of_inertia(),
                3.3 * 4.5 * (3.3 * 0.5 - 1.1) * (-4.5 * 0.5 - 8.2)
            );
        }
        #[test]
        fn negative_width_positive_height() {
            let s = RectangleSection::new([-3.3, 4.5], [1.1, -8.2]);
            assert_eq!(
                s.product_of_inertia(),
                3.3 * 4.5 * (-3.3 * 0.5 + 1.1) * (4.5 * 0.5 - 8.2)
            );
        }
        #[test]
        fn negative_width_negative_height() {
            let s = RectangleSection::new([-3.3, -4.5], [-1.1, 8.2]);
            assert_eq!(
                s.product_of_inertia(),
                3.3 * 4.5 * (-3.3 * 0.5 - 1.1) * (-4.5 * 0.5 + 8.2)
            );
        }
    }
}
