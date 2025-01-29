#[derive(Debug, Clone, PartialEq)]
enum Command<T, S> {
    MoveTo {
        to: T,
    },
    LineTo {
        to: T,
    },
    CubicBezier {
        cp1: T,
        cp2: T,
        to: T,
    },
    SquareBezier {
        cp: T,
        to: T,
    },
    Arc {
        radius: T,
        axis_rotation: S,
        large_arc_flag: bool,
        sweep_flag: bool,
        to: T,
    },
    ClosePath,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment<'a, T, S> {
    Line {
        from: &'a T,
        to: &'a T,
    },
    CubicBezier {
        from: &'a T,
        cp1: &'a T,
        cp2: &'a T,
        to: &'a T,
    },
    SquareBezier {
        from: &'a T,
        cp: &'a T,
        to: &'a T,
    },
    Arc {
        from: &'a T,
        radius: &'a T,
        axis_rotation: &'a S,
        large_arc_flag: bool,
        sweep_flag: bool,
        to: &'a T,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path<T, S> {
    commands: Vec<Command<T, S>>,
    init_pos: T,
}

impl<T, S> Path<T, S> {
    pub fn move_to(&mut self, to: T) -> &mut Self {
        if self.commands.is_empty() {
            self.init_pos = to;
        } else {
            self.commands.push(Command::MoveTo { to });
        }
        self
    }

    pub fn line_to(&mut self, to: T) -> &mut Self {
        self.commands.push(Command::LineTo { to });
        self
    }

    pub fn cubic_bezier(&mut self, cp1: T, cp2: T, to: T) -> &mut Self {
        self.commands.push(Command::CubicBezier { cp1, cp2, to });
        self
    }

    pub fn square_bezier(&mut self, cp: T, to: T) -> &mut Self {
        self.commands.push(Command::SquareBezier { cp, to });
        self
    }

    pub fn arc(
        &mut self,
        radius: T,
        axis_rotation: S,
        large_arc_flag: bool,
        sweep_flag: bool,
        to: T,
    ) -> &mut Self {
        self.commands.push(Command::Arc {
            radius,
            axis_rotation,
            large_arc_flag,
            sweep_flag,
            to,
        });
        self
    }

    pub fn close_path(&mut self) -> &mut Self {
        self.commands.push(Command::ClosePath);
        self
    }

    pub fn segments(&self) -> impl Iterator<Item = Segment<T, S>> {
        self.commands
            .iter()
            .scan(
                [&self.init_pos, &self.init_pos],
                |state, command| match command {
                    Command::MoveTo { to } => {
                        state[0] = to;
                        state[1] = to;
                        Some(None)
                    }
                    Command::LineTo { to } => {
                        let from = state[1];
                        state[1] = to;
                        Some(Some(Segment::Line { from, to }))
                    }
                    Command::SquareBezier { cp, to } => {
                        let from = state[1];
                        state[1] = to;
                        Some(Some(Segment::SquareBezier { from, cp, to }))
                    }
                    Command::CubicBezier { cp1, cp2, to } => {
                        let from = state[1];
                        state[1] = to;
                        Some(Some(Segment::CubicBezier { from, cp1, cp2, to }))
                    }
                    Command::Arc {
                        radius,
                        axis_rotation,
                        large_arc_flag,
                        sweep_flag,
                        to,
                    } => {
                        let from = state[1];
                        state[1] = to;
                        Some(Some(Segment::Arc {
                            from,
                            radius,
                            axis_rotation,
                            large_arc_flag: *large_arc_flag,
                            sweep_flag: *sweep_flag,
                            to,
                        }))
                    }
                    Command::ClosePath => {
                        let from = state[1];
                        state[1] = state[0];
                        Some(Some(Segment::Line { from, to: state[0] }))
                    }
                },
            )
            .filter_map(|segment| segment)
    }
}

impl<T, S> Path<T, S>
where
    T: Default,
{
    pub fn new() -> Self {
        Self {
            commands: vec![],
            init_pos: T::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::complex::Complex;

    use super::*;

    #[test]
    fn closed_and_opened_path() {
        let mut a = Path::new();
        a.move_to(Complex::new(3, 5));
        a.line_to(Complex::new(8, -2));
        a.line_to(Complex::new(-12, 6));
        a.close_path();
        a.arc(Complex::new(30, 25), 1, false, true, Complex::new(18, 21));
        a.move_to(Complex::new(32, 55));
        a.cubic_bezier(
            Complex::new(61, 32),
            Complex::new(83, 11),
            Complex::new(108, 129),
        );
        a.square_bezier(Complex::new(-21, 30), Complex::new(-71, 91));
        let mut i = a.segments();
        assert_eq!(
            i.next(),
            Some(Segment::Line {
                from: &Complex::new(3, 5),
                to: &Complex::new(8, -2),
            }),
        );
        assert_eq!(
            i.next(),
            Some(Segment::Line {
                from: &Complex::new(8, -2),
                to: &Complex::new(-12, 6),
            }),
        );
        assert_eq!(
            i.next(),
            Some(Segment::Line {
                from: &Complex::new(-12, 6),
                to: &Complex::new(3, 5),
            }),
        );
        assert_eq!(
            i.next(),
            Some(Segment::Arc {
                from: &Complex::new(3, 5),
                radius: &Complex::new(30, 25),
                axis_rotation: &1,
                large_arc_flag: false,
                sweep_flag: true,
                to: &Complex::new(18, 21),
            }),
        );
        assert_eq!(
            i.next(),
            Some(Segment::CubicBezier {
                from: &Complex::new(32, 55),
                cp1: &Complex::new(61, 32),
                cp2: &Complex::new(83, 11),
                to: &Complex::new(108, 129),
            }),
        );
        assert_eq!(
            i.next(),
            Some(Segment::SquareBezier {
                from: &Complex::new(108, 129),
                cp: &Complex::new(-21, 30),
                to: &Complex::new(-71, 91),
            }),
        );
        assert_eq!(i.next(), None);
    }

    #[test]
    fn method_chain() {
        let mut a = Path::new();
        a.move_to(Complex::new(3, 5))
            .line_to(Complex::new(8, -2))
            .line_to(Complex::new(-12, 6))
            .close_path()
            .arc(Complex::new(30, 25), 1, false, true, Complex::new(18, 21))
            .move_to(Complex::new(32, 55))
            .cubic_bezier(
                Complex::new(61, 32),
                Complex::new(83, 11),
                Complex::new(108, 129),
            )
            .square_bezier(Complex::new(-21, 30), Complex::new(-71, 91));
        let mut i = a.segments();
        assert_eq!(
            i.next(),
            Some(Segment::Line {
                from: &Complex::new(3, 5),
                to: &Complex::new(8, -2),
            }),
        );
        assert_eq!(
            i.next(),
            Some(Segment::Line {
                from: &Complex::new(8, -2),
                to: &Complex::new(-12, 6),
            }),
        );
        assert_eq!(
            i.next(),
            Some(Segment::Line {
                from: &Complex::new(-12, 6),
                to: &Complex::new(3, 5),
            }),
        );
        assert_eq!(
            i.next(),
            Some(Segment::Arc {
                from: &Complex::new(3, 5),
                radius: &Complex::new(30, 25),
                axis_rotation: &1,
                large_arc_flag: false,
                sweep_flag: true,
                to: &Complex::new(18, 21),
            }),
        );
        assert_eq!(
            i.next(),
            Some(Segment::CubicBezier {
                from: &Complex::new(32, 55),
                cp1: &Complex::new(61, 32),
                cp2: &Complex::new(83, 11),
                to: &Complex::new(108, 129),
            }),
        );
        assert_eq!(
            i.next(),
            Some(Segment::SquareBezier {
                from: &Complex::new(108, 129),
                cp: &Complex::new(-21, 30),
                to: &Complex::new(-71, 91),
            }),
        );
        assert_eq!(i.next(), None);
    }
}
