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
    let area = section.area();
    let center = section.centroid();
    let ixy = section.product_of_inertia() - center.iter().product::<Float>() * area;
    let mut i = section.moment_of_inertia();
    for (v, c) in i.iter_mut().zip(center) {
        *v -= c * c * area;
    }
    (ixy * -2.0).atan2(i[1] - i[0]) * 0.5
}

pub mod circle;
pub mod rectangle;
pub mod translated;

#[cfg(test)]
mod tests {
    use super::*;
    mod principal_axis {
        use super::*;
        /// width: 4.0
        /// height: 6.0
        /// center: [1.2, 2.0]
        /// axis_rotation: -15 degrees
        struct TiltedPortraitRectangleSection {}
        impl Section for TiltedPortraitRectangleSection {
            fn area(&self) -> Float {
                24.0
            }
            fn centroid(&self) -> [Float; 2] {
                [1.2, 2.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                let r3 = (3.0 as Float).sqrt();
                [52.0 - 10.0 * r3 + 34.56, 52.0 + 10.0 * r3 + 96.0]
            }
            fn product_of_inertia(&self) -> Float {
                10.0 + 57.6
            }
        }
        #[test]
        fn tilted_portrait_rectangle_section() {
            assert_eq!(
                format!(
                    "{:.13?}",
                    principal_axis(TiltedPortraitRectangleSection {}).to_degrees()
                ),
                format!("{:.13?}", -15.0)
            );
        }
        /// width: 4.0
        /// height: 6.0
        /// center: [-2.0, 1.2]
        /// axis_rotation: 75 degrees
        struct RotatedPortraitRectangleSection {}
        impl Section for RotatedPortraitRectangleSection {
            fn area(&self) -> Float {
                24.0
            }
            fn centroid(&self) -> [Float; 2] {
                [-2.0, 1.2]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                let r3 = (3.0 as Float).sqrt();
                [52.0 + 10.0 * r3 + 96.0, 52.0 - 10.0 * r3 + 34.56]
            }
            fn product_of_inertia(&self) -> Float {
                -10.0 - 57.6
            }
        }
        #[test]
        fn rotated_portrait_rectangle_section() {
            assert_eq!(
                format!(
                    "{:.13?}",
                    principal_axis(RotatedPortraitRectangleSection {}).to_degrees()
                ),
                format!("{:.13?}", 75.0)
            );
        }
        /// width: 6.0
        /// height: 4.0
        /// center: [2.0, 1.2]
        /// axis_rotation: 15 degrees
        struct TiltedLandscapeRectangleSection {}
        impl Section for TiltedLandscapeRectangleSection {
            fn area(&self) -> Float {
                24.0
            }
            fn centroid(&self) -> [Float; 2] {
                [2.0, 1.2]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                let r3 = (3.0 as Float).sqrt();
                [52.0 + 10.0 * r3 + 96.0, 52.0 - 10.0 * r3 + 34.56]
            }
            fn product_of_inertia(&self) -> Float {
                10.0 + 57.6
            }
        }
        #[test]
        fn tilted_landscape_rectangle_section() {
            assert_eq!(
                format!(
                    "{:.13?}",
                    principal_axis(TiltedLandscapeRectangleSection {}).to_degrees()
                ),
                format!("{:.13?}", -75.0)
            );
        }
        /// width: 6.0
        /// height: 4.0
        /// center: [-1.2, 2.0]
        /// axis_rotation: -75 degrees
        struct RotatedLandscapeRectangleSection {}
        impl Section for RotatedLandscapeRectangleSection {
            fn area(&self) -> Float {
                24.0
            }
            fn centroid(&self) -> [Float; 2] {
                [-1.2, 2.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                let r3 = (3.0 as Float).sqrt();
                [52.0 - 10.0 * r3 + 34.56, 52.0 + 10.0 * r3 + 96.0]
            }
            fn product_of_inertia(&self) -> Float {
                -10.0 - 57.6
            }
        }
        #[test]
        fn rotated_landscape_rectangle_section() {
            assert_eq!(
                format!(
                    "{:.13?}",
                    principal_axis(RotatedLandscapeRectangleSection {}).to_degrees()
                ),
                format!("{:.13?}", 15.0)
            );
        }
        /// width: 6.0
        /// height: 4.0
        /// center: [0.0, 0.0]
        /// axis_rotation: 0 degrees
        struct UprightPortraitRectangleSection {}
        impl Section for UprightPortraitRectangleSection {
            fn area(&self) -> Float {
                24.0
            }
            fn centroid(&self) -> [Float; 2] {
                [0.0, 0.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [32.0, 72.0]
            }
            fn product_of_inertia(&self) -> Float {
                0.0
            }
        }
        #[test]
        fn upright_portrait_rectangle_section() {
            assert_eq!(
                principal_axis(UprightPortraitRectangleSection {}).to_degrees(),
                0.0
            );
        }
        /// width: 4.0
        /// height: 6.0
        /// center: [0.0, 0.0]
        /// axis_rotation: 0 degrees
        struct UprightLandscapeRectangleSection {}
        impl Section for UprightLandscapeRectangleSection {
            fn area(&self) -> Float {
                24.0
            }
            fn centroid(&self) -> [Float; 2] {
                [0.0, 0.0]
            }
            fn moment_of_inertia(&self) -> [Float; 2] {
                [72.0, 32.0]
            }
            fn product_of_inertia(&self) -> Float {
                0.0
            }
        }
        #[test]
        fn upright_landscape_rectangle_section() {
            assert_eq!(
                principal_axis(UprightLandscapeRectangleSection {}).to_degrees(),
                -90.0
            );
        }
    }
}
