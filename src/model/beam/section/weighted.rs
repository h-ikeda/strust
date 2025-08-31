use super::Section;
use crate::Float;

pub struct WeightedSection<T: Section> {
    weight: Float,
    section: T,
}

impl<T: Section> WeightedSection<T> {
    pub const fn new(section: T, weight: Float) -> Self {
        Self { weight, section }
    }
}

impl<T: Section> Section for WeightedSection<T> {
    fn area(&self) -> Float {
        self.section.area() * self.weight
    }
    fn centroid(&self) -> [Float; 2] {
        self.section.centroid()
    }
    fn moment_of_inertia(&self) -> [Float; 2] {
        self.section.moment_of_inertia().map(|v| v * self.weight)
    }
    fn product_of_inertia(&self) -> Float {
        self.section.product_of_inertia() * self.weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestSection {}
    impl Section for TestSection {
        fn area(&self) -> Float {
            15.0
        }
        fn centroid(&self) -> [Float; 2] {
            [0.5, 1.0]
        }
        fn moment_of_inertia(&self) -> [Float; 2] {
            [
                5.0 * 3.0 * 3.0 * 3.0 / 12.0 + 0.5 * 0.5 * 15.0,
                3.0 * 5.0 * 5.0 * 5.0 / 12.0 + 15.0,
            ]
        }
        fn product_of_inertia(&self) -> Float {
            15.0 * 0.5 * 1.0
        }
    }
    #[test]
    fn area() {
        let w = WeightedSection::new(TestSection {}, -1.5);
        assert_eq!(w.area(), -15.0 * 1.5);
    }
    #[test]
    fn centroid() {
        let w = WeightedSection::new(TestSection {}, -1.5);
        assert_eq!(w.centroid(), [0.5, 1.0]);
    }
    #[test]
    fn moment_of_inertia() {
        let w = WeightedSection::new(TestSection {}, -1.5);
        assert_eq!(
            w.moment_of_inertia(),
            [
                -1.5 * 5.0 * 3.0 * 3.0 * 3.0 / 12.0 - 1.5 * 0.5 * 0.5 * 15.0,
                -1.5 * 3.0 * 5.0 * 5.0 * 5.0 / 12.0 - 1.5 * 15.0,
            ]
        );
    }
    #[test]
    fn product_of_inertia() {
        let w = WeightedSection::new(TestSection {}, -1.5);
        assert_eq!(w.product_of_inertia(), -15.0 * 0.5 * 1.0 * 1.5)
    }
}
