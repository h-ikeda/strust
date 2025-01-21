pub trait Sin {
    fn sin(&self) -> Self;
}

impl Sin for f32 {
    fn sin(&self) -> Self {
        (*self).sin()
    }
}

impl Sin for f64 {
    fn sin(&self) -> Self {
        (*self).sin()
    }
}

pub trait Cos {
    fn cos(&self) -> Self;
}

impl Cos for f32 {
    fn cos(&self) -> Self {
        (*self).cos()
    }
}

impl Cos for f64 {
    fn cos(&self) -> Self {
        (*self).cos()
    }
}

pub trait Hypot {
    fn hypot(&self, other: &Self) -> Self;
}

impl Hypot for f32 {
    fn hypot(&self, other: &Self) -> Self {
        (*self).hypot(*other)
    }
}

impl Hypot for f64 {
    fn hypot(&self, other: &Self) -> Self {
        (*self).hypot(*other)
    }
}
