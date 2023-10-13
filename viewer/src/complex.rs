#[derive(Debug, Clone, Copy)]
pub(crate) struct Complex {
    pub(crate) a: f32,
    pub(crate) b: f32,
}

impl Complex {
    pub(crate) fn new(a: f32, b: f32) -> Self {
        Self {
            a, b
        }
    }
    pub(crate) fn magnitude(self) -> f32 {
        self.a * self.a + self.b * self.b
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a * rhs.a - self.b * rhs.b,
            b: self.a * rhs.b + self.b * rhs.a,
        }
    }
}