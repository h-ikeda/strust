use crate::Float;

pub trait Section {
    fn area(&self) -> Float;
    fn centroid(&self) -> [Float; 2];
    fn moment_of_inertia(&self) -> [Float; 2];
    fn product_of_inertia(&self) -> Float;
}

pub mod rectangle;
