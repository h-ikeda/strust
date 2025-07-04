use std::vec;

use super::Section;
use crate::Float;

pub struct CombinedSection {
    pub sections: Vec<Box<dyn Section>>,
}

impl CombinedSection {
    pub const fn new() -> Self {
        Self { sections: vec![] }
    }
    pub fn push(&mut self, section: impl Section + 'static) {
        self.sections.push(Box::new(section));
    }
}

impl Section for CombinedSection {
    fn area(&self) -> Float {
        let mut v: Vec<Float> = self.sections.iter().map(|s| s.area()).collect();
        v.sort_by(|a, b| a.abs().total_cmp(&b.abs()));
        v.iter().sum()
    }
    fn centroid(&self) -> [Float; 2] {
        let (mut va, vs): (Vec<Float>, Vec<[Float; 2]>) = self
            .sections
            .iter()
            .map(|s| {
                let t = s.area();
                (t, s.centroid().map(|c| c * t))
            })
            .unzip();
        va.sort_by(|a, b| a.abs().total_cmp(&b.abs()));
        let a: Float = va.iter().sum();
        let s: [Vec<Float>; 2] = vs.iter().map(|&i| i.into()).unzip().into();
        s.map(|mut i| {
            i.sort_by(|a, b| a.abs().total_cmp(&b.abs()));
            i.iter().sum::<Float>() / a
        })
    }
    fn moment_of_inertia(&self) -> [Float; 2] {
        let j: [Vec<Float>; 2] = self
            .sections
            .iter()
            .map(|s| s.moment_of_inertia().into())
            .unzip()
            .into();
        j.map(|mut i| {
            i.sort_by(|a, b| a.abs().total_cmp(&b.abs()));
            i.iter().sum()
        })
    }
    fn product_of_inertia(&self) -> Float {
        let mut v: Vec<Float> = self
            .sections
            .iter()
            .map(|s| s.product_of_inertia())
            .collect();
        v.sort_by(|a, b| a.abs().total_cmp(&b.abs()));
        v.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod combined_rectangle {
        use super::*;
        struct TestSectionA {}
        impl Section for TestSectionA {
            fn area(&self) -> Float {
                30.0
            }
            fn centroid(&self) -> [Float; 2] {
                [2.5, 3.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [
                    6.0 * 5.0 * 5.0 * 5.0 / 12.0 + 2.5 * 2.5 * 30.0,
                    5.0 * 6.0 * 6.0 * 6.0 / 12.0 + 3.0 * 3.0 * 30.0,
                ]
            }
            fn product_of_inertia(&self) -> Float {
                30.0 * 2.5 * 3.0
            }
        }
        struct TestSectionB {}
        impl Section for TestSectionB {
            fn area(&self) -> Float {
                30.0
            }
            fn centroid(&self) -> [Float; 2] {
                [-2.5, 3.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [
                    6.0 * 5.0 * 5.0 * 5.0 / 12.0 + 2.5 * 2.5 * 30.0,
                    5.0 * 6.0 * 6.0 * 6.0 / 12.0 + 3.0 * 3.0 * 30.0,
                ]
            }
            fn product_of_inertia(&self) -> Float {
                -30.0 * 2.5 * 3.0
            }
        }
        struct TestSectionC {}
        impl Section for TestSectionC {
            fn area(&self) -> Float {
                30.0
            }
            fn centroid(&self) -> [Float; 2] {
                [-2.5, -3.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [
                    6.0 * 5.0 * 5.0 * 5.0 / 12.0 + 2.5 * 2.5 * 30.0,
                    5.0 * 6.0 * 6.0 * 6.0 / 12.0 + 3.0 * 3.0 * 30.0,
                ]
            }
            fn product_of_inertia(&self) -> Float {
                30.0 * 2.5 * 3.0
            }
        }
        struct TestSectionD {}
        impl Section for TestSectionD {
            fn area(&self) -> Float {
                30.0
            }
            fn centroid(&self) -> [Float; 2] {
                [2.5, -3.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [
                    6.0 * 5.0 * 5.0 * 5.0 / 12.0 + 2.5 * 2.5 * 30.0,
                    5.0 * 6.0 * 6.0 * 6.0 / 12.0 + 3.0 * 3.0 * 30.0,
                ]
            }
            fn product_of_inertia(&self) -> Float {
                -30.0 * 2.5 * 3.0
            }
        }
        #[test]
        fn area() {
            let mut c = CombinedSection::new();
            c.push(TestSectionA {});
            c.push(TestSectionB {});
            c.push(TestSectionC {});
            c.push(TestSectionD {});
            assert_eq!(c.area(), 120.0);
        }
        #[test]
        fn centroid() {
            let mut c = CombinedSection::new();
            c.push(TestSectionA {});
            c.push(TestSectionB {});
            c.push(TestSectionC {});
            c.push(TestSectionD {});
            assert_eq!(c.centroid(), [0.0, 0.0]);
        }
        #[test]
        fn moment_of_inertia() {
            let mut c = CombinedSection::new();
            c.push(TestSectionA {});
            c.push(TestSectionB {});
            c.push(TestSectionC {});
            c.push(TestSectionD {});
            assert_eq!(
                c.moment_of_inertia(),
                [
                    12.0 * 10.0 * 10.0 * 10.0 / 12.0,
                    10.0 * 12.0 * 12.0 * 12.0 / 12.0
                ]
            );
        }
        #[test]
        fn product_of_inertia() {
            let mut c = CombinedSection::new();
            c.push(TestSectionA {});
            c.push(TestSectionB {});
            c.push(TestSectionC {});
            c.push(TestSectionD {});
            assert_eq!(c.product_of_inertia(), 0.0);
        }
    }
}
