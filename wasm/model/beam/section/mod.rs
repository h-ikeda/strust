use crate::Float;

pub trait Section {
    fn area(&self) -> Float;
    fn centroid(&self) -> [Float; 2];
    fn moment_of_inertia(&self) -> [Float; 2];
    fn product_of_inertia(&self) -> Float;
}

/// Calculates the principal axis direction of the section.
/// Returns the angle of axis in radians.
pub fn principal_axis(section: impl Section) -> Float {
    let [jy, jx] = section.moment_of_inertia();
    (section.product_of_inertia() * -2.0).atan2(jx - jy) * 0.5
}

pub mod circle;
pub mod rectangle;
pub mod translated;

#[cfg(test)]
mod tests {
    use super::*;
    mod centered_rectangle {
        use super::*;
        // Mock a centered rectangle section.
        // width: 4.9; height: 8.1; rotation: 13 degrees;
        struct TestSection {}
        impl Section for TestSection {
            fn area(&self) -> Float {
                4.9 * 8.1
            }
            fn centroid(&self) -> [Float; 2] {
                [0.0, 0.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [
                    (4.9 * 8.1 * 8.1 * 8.1 / 12.0 + 8.1 * 4.9 * 4.9 * 4.9 / 12.0) / 2.0
                        - (4.9 * 8.1 * 8.1 * 8.1 / 12.0 - 8.1 * 4.9 * 4.9 * 4.9 / 12.0) / 2.0
                            * ((13.0 as Float).to_radians() * 2.0).cos(),
                    (4.9 * 8.1 * 8.1 * 8.1 / 12.0 + 8.1 * 4.9 * 4.9 * 4.9 / 12.0) / 2.0
                        + (4.9 * 8.1 * 8.1 * 8.1 / 12.0 - 8.1 * 4.9 * 4.9 * 4.9 / 12.0) / 2.0
                            * ((13.0 as Float).to_radians() * 2.0).cos(),
                ]
            }
            fn product_of_inertia(&self) -> Float {
                -(4.9 * 8.1 * 8.1 * 8.1 / 12.0 - 8.1 * 4.9 * 4.9 * 4.9 / 12.0) / 2.0
                    * ((13.0 as Float).to_radians() * 2.0).sin()
            }
        }
        #[test]
        fn principal_axis() {
            assert_eq!(super::principal_axis(TestSection {}).to_degrees(), 13.0);
        }
    }
    mod first_quadrant_circle {
        use super::*;
        // Mock a translated circle section.
        // radius: 5.1; centroid: [3.4, 9.0];
        struct TestSection {}
        impl Section for TestSection {
            fn area(&self) -> Float {
                5.1 * 5.1 * (180.0 as Float).to_radians()
            }
            fn centroid(&self) -> [Float; 2] {
                [3.4, 9.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [
                    5.1 * 5.1 * 5.1 * 5.1 * 16.0 * (180.0 as Float).to_radians() / 64.0
                        + 3.4 * 3.4 * 5.1 * 5.1 * (180.0 as Float).to_radians(),
                    5.1 * 5.1 * 5.1 * 5.1 * 16.0 * (180.0 as Float).to_radians() / 64.0
                        + 9.0 * 9.0 * 5.1 * 5.1 * (180.0 as Float).to_radians(),
                ]
            }
            fn product_of_inertia(&self) -> Float {
                5.1 * 5.1 * (180.0 as Float).to_radians() * 3.4 * 9.0
            }
        }
        #[test]
        fn principal_axis() {
            assert_eq!(
                super::principal_axis(TestSection {}),
                (-3.4 as Float).atan2(9.0)
            );
        }
    }
    mod second_quadrant_circle {
        use super::*;
        // Mock a translated circle section.
        // radius: 5.1; centroid: [-9.0, 3.4];
        struct TestSection {}
        impl Section for TestSection {
            fn area(&self) -> Float {
                5.1 * 5.1 * (180.0 as Float).to_radians()
            }
            fn centroid(&self) -> [Float; 2] {
                [-9.0, 3.4]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [
                    5.1 * 5.1 * 5.1 * 5.1 * 16.0 * (180.0 as Float).to_radians() / 64.0
                        + 9.0 * 9.0 * 5.1 * 5.1 * (180.0 as Float).to_radians(),
                    5.1 * 5.1 * 5.1 * 5.1 * 16.0 * (180.0 as Float).to_radians() / 64.0
                        + 3.4 * 3.4 * 5.1 * 5.1 * (180.0 as Float).to_radians(),
                ]
            }
            fn product_of_inertia(&self) -> Float {
                -5.1 * 5.1 * (180.0 as Float).to_radians() * 3.4 * 9.0
            }
        }
        #[test]
        fn principal_axis() {
            assert_eq!(
                super::principal_axis(TestSection {}),
                (9.0 as Float).atan2(3.4)
            );
        }
    }
    mod third_quadrant_circle {
        use super::*;
        // Mock a translated circle section.
        // radius: 5.1; centroid: [-3.4, -9.0];
        struct TestSection {}
        impl Section for TestSection {
            fn area(&self) -> Float {
                5.1 * 5.1 * (180.0 as Float).to_radians()
            }
            fn centroid(&self) -> [Float; 2] {
                [-3.4, -9.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [
                    5.1 * 5.1 * 5.1 * 5.1 * 16.0 * (180.0 as Float).to_radians() / 64.0
                        + 3.4 * 3.4 * 5.1 * 5.1 * (180.0 as Float).to_radians(),
                    5.1 * 5.1 * 5.1 * 5.1 * 16.0 * (180.0 as Float).to_radians() / 64.0
                        + 9.0 * 9.0 * 5.1 * 5.1 * (180.0 as Float).to_radians(),
                ]
            }
            fn product_of_inertia(&self) -> Float {
                5.1 * 5.1 * (180.0 as Float).to_radians() * 3.4 * 9.0
            }
        }
        #[test]
        fn principal_axis() {
            assert_eq!(
                super::principal_axis(TestSection {}),
                (-3.4 as Float).atan2(9.0)
            );
        }
    }
    mod fourth_quadrant_circle {
        use super::*;
        // Mock a translated circle section.
        // radius: 5.1; centroid: [9.0, -3.4];
        struct TestSection {}
        impl Section for TestSection {
            fn area(&self) -> Float {
                5.1 * 5.1 * (180.0 as Float).to_radians()
            }
            fn centroid(&self) -> [Float; 2] {
                [9.0, -3.4]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [
                    5.1 * 5.1 * 5.1 * 5.1 * 16.0 * (180.0 as Float).to_radians() / 64.0
                        + 9.0 * 9.0 * 5.1 * 5.1 * (180.0 as Float).to_radians(),
                    5.1 * 5.1 * 5.1 * 5.1 * 16.0 * (180.0 as Float).to_radians() / 64.0
                        + 3.4 * 3.4 * 5.1 * 5.1 * (180.0 as Float).to_radians(),
                ]
            }
            fn product_of_inertia(&self) -> Float {
                -5.1 * 5.1 * (180.0 as Float).to_radians() * 3.4 * 9.0
            }
        }
        #[test]
        fn principal_axis() {
            assert_eq!(
                super::principal_axis(TestSection {}),
                (9.0 as Float).atan2(3.4)
            );
        }
    }
}
