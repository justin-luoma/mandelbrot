use num::complex::Complex64;

pub type Rgba = [u8; 4];

pub type TriangleVertexes = ((f64, f64), (f64, f64), (f64, f64));

#[derive(Debug)]
pub struct SierpinskiTriangle {
    pub triangles: Vec<Vec<Triangle>>,
    depth: usize,
    max_depth: usize,
    radius: f64,
    colors: (Rgba, Rgba),
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Complex64,
    pub b: Complex64,
    pub c: Complex64,
    pub midpoints: (Complex64, Complex64, Complex64),
}

impl Triangle {
    pub fn side_length(radius: f64) -> f64 {
        radius * 3f64.sqrt()
    }

    pub fn altitude(side_length: f64) -> f64 {
        (3f64.sqrt() / 2.) * side_length
    }

    fn new(
        a: Complex64,
        b: Complex64,
        c: Complex64,
        midpoints: (Complex64, Complex64, Complex64),
    ) -> Self {
        Self {
            a,
            b,
            c,
            midpoints,
        }
    }

    fn top(&self) -> Self {
        let a = self.a;
        let b = self.midpoints.0;
        let c = self.midpoints.2;
        Self {
            a,
            b,
            c,
            midpoints: (midpoint(a, b), midpoint(b, c), midpoint(a, c)),
        }
    }

    fn left(&self) -> Self {
        let a = self.midpoints.2;
        let b = self.midpoints.1;
        let c = self.c;
        Self {
            a,
            b,
            c,
            midpoints: (midpoint(a, b), midpoint(b, c), midpoint(a, c)),
        }
    }

    fn right(&self) -> Self {
        let a = self.midpoints.0;
        let b = self.b;
        let c = self.midpoints.1;
        Self {
            a,
            b,
            c,
            midpoints: (midpoint(a, b), midpoint(b, c), midpoint(a, c)),
        }
    }

    fn divide(&self) -> [Triangle; 3] {
        [
            self.top(),
            self.left(),
            self.right(),
        ]
    }

    fn pos(&self) -> TriangleVertexes {
        (
            (self.a.re, self.a.im),
            (self.b.re, self.b.im),
            (self.c.re, self.c.im),
        )
    }

    fn midpoints(&self) -> TriangleVertexes {
        let (ab, bc, ac) = self.midpoints;
        (
            (ab.re, ab.im),
            (bc.re, bc.im),
            (ac.re, ac.im),
        )
    }
}

impl SierpinskiTriangle {
    /// Creates a new `SierpinskiTriangle` with the given `radius` (center to top vertex) and
    /// `max_depth`
    pub fn new(radius: f64, max_depth: usize, colors: (Rgba, Rgba)) -> Self {
        let mut triangles = Vec::with_capacity(max_depth);
        let a = Complex64::new(0., radius);
        let length = Triangle::side_length(radius);
        let altitude = Triangle::altitude(length);
        let b = Complex64::new(length / 2., radius - altitude);
        let c = Complex64::new(-(length / 2.), radius - altitude);
        let midpoints = {
            (
                midpoint(a, b),
                midpoint(b, c),
                midpoint(a, c),
            )
        };
        triangles.push(Triangle::new(a, b, c, midpoints));
        Self {
            triangles: vec![triangles],
            depth: 0,
            max_depth,
            radius,
            colors,
        }
    }

    pub fn iterate(&mut self) {
        if self.depth == 0 {
            self.depth += 1;
            return;
        }
        self.next();
    }

    pub fn iterate_and_pixels(&mut self) -> Vec<(Rgba, TriangleVertexes)> {
        self
            .next()
            .iter()
            .map(|tri| (self.colors.1, tri.midpoints()))
            .collect()
    }

    fn next(&mut self) -> Vec<Triangle> {
        let depth = self.depth;
        if depth > self.max_depth {
            return Vec::new();
        }
        let triangles = &self.triangles[depth - 1];
        let next: Vec<_> = triangles
            .iter()
            .flat_map(|tri| tri.divide())
            .collect();
        self.triangles.push(next.clone());
        self.depth += 1;

        next
    }

    pub fn pixels(&self) -> Vec<(Rgba, TriangleVertexes)> {
        let mut shapes = Vec::with_capacity(self.depth + 1);
        let mut iter = self.triangles.iter();
        if let Some(triangles) = iter.next() {
            shapes.extend(
                triangles
                    .iter()
                    .flat_map(|tri| {
                        if self.depth > 0 {
                            vec![
                                (self.colors.0, tri.pos()),
                                (self.colors.1, tri.midpoints()),
                            ]
                        } else {
                            vec![(self.colors.0, tri.pos())]
                        }
                    })
            )
        }
        shapes.extend(
            iter
                .flat_map(|v| v
                    .iter()
                    .map(|tri| (self.colors.1, tri.midpoints())))
        );
        shapes
    }
}

fn midpoint(a: Complex64, b: Complex64) -> Complex64 {
    Complex64::new(
        (a.re + b.re) / 2.,
        (a.im + b.im) / 2.,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midpoint() {
        let a = Complex64::new(-3., 5.);
        let b = Complex64::new(8., -1.);

        let expected = Complex64::new(2.5, 2.);
        assert_eq!(expected, midpoint(a, b));
    }
}
