pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(&self) -> Self {
        (*self).sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(&self) -> Self {
        (*self).sqrt()
    }
}

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
