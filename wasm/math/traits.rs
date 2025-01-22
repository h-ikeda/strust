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

pub trait Exp {
    fn exp(&self) -> Self;
}

impl Exp for f32 {
    fn exp(&self) -> Self {
        (*self).exp()
    }
}

impl Exp for f64 {
    fn exp(&self) -> Self {
        (*self).exp()
    }
}

pub trait Atan2 {
    fn atan2(&self, other: &Self) -> Self;
}

impl Atan2 for f32 {
    fn atan2(&self, other: &Self) -> Self {
        (*self).atan2(*other)
    }
}

impl Atan2 for f64 {
    fn atan2(&self, other: &Self) -> Self {
        (*self).atan2(*other)
    }
}

pub trait Ln {
    fn ln(&self) -> Self;
}

impl Ln for f32 {
    fn ln(&self) -> Self {
        (*self).ln()
    }
}

impl Ln for f64 {
    fn ln(&self) -> Self {
        (*self).ln()
    }
}
