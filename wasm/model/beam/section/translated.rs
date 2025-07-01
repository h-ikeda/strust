use super::Section;
use crate::Float;
use std::array::from_fn;

pub struct TranslatedSection<T: Section> {
    pub origin: T,
    pub offset: [Float; 2],
}

impl<T: Section> TranslatedSection<T> {
    pub const fn new(section: T, offset: [Float; 2]) -> Self {
        Self {
            origin: section,
            offset,
        }
    }
}

impl<T: Section> Section for TranslatedSection<T> {
    fn area(&self) -> Float {
        self.origin.area()
    }
    fn centroid(&self) -> [Float; 2] {
        let c = self.origin.centroid();
        from_fn(|i| c[i] + self.offset[i])
    }
    fn moment_of_inertia(&self) -> [Float; 2] {
        let a = self.origin.area();
        let c = self.origin.centroid().map(|v| v * 2.0);
        let j = self.origin.moment_of_inertia();
        from_fn(|i| j[i] + (self.offset[i] + c[i]) * self.offset[i] * a)
    }
    fn product_of_inertia(&self) -> Float {
        let c = self.origin.centroid();
        let mut i = c
            .iter()
            .rev()
            .zip(self.offset)
            .map(|(a, b)| a * b)
            .chain([self.offset.iter().product()]);
        let mut t: [Float; 3] = from_fn(|_| i.next().unwrap());
        t.sort_by(|a, b| a.abs().total_cmp(&b.abs()));
        self.origin.product_of_inertia() + t.iter().sum::<Float>() * self.origin.area()
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
        let s = TranslatedSection::new(Origin {}, [-3.3, -1.2]);
        assert_eq!(s.area(), 4.9 * 8.1);
    }
    #[test]
    fn centroid() {
        let s = TranslatedSection::new(Origin {}, [-3.3, -1.2]);
        assert_eq!(s.centroid(), [2.2 - 3.3, 3.1 - 1.2]);
    }
    #[test]
    fn moment_of_inertia() {
        let s = TranslatedSection::new(Origin {}, [-3.4, -1.3]);
        assert_eq!(
            s.moment_of_inertia(),
            [
                8.1 * 4.9 * 4.9 * 4.9 / 12.0 + 1.2 * 1.2 * 4.9 * 8.1,
                4.9 * 8.1 * 8.1 * 8.1 / 12.0 + 1.8 * 1.8 * 4.9 * 8.1,
            ]
        );
    }
    #[test]
    fn product_of_inertia() {
        let s = TranslatedSection::new(Origin {}, [-3.5, -1.4]);
        assert_eq!(s.product_of_inertia(), -4.9 * 8.1 * 1.3 * 1.7);
    }
}
