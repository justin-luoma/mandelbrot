mod complex;

use std::f32::consts::TAU;
use crate::complex::Complex;

fn mandelbrot(x: f32, y: f32) -> f32 {
    let mut z = Complex::new(0., 0.);
    let c = Complex::new(x, y);
    let max = 256;
    let mut i = 0;
    while i < max && z.magnitude() < 32. {
        z = z * z + c;
        i += 1;
    }
    (i as f32 - z.magnitude().log2().log2()) / (max as f32)
}

fn color(t: f32) -> [u8; 3] {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (TAU * (c.0 * t + d.0)).cos() + a.0;
    let g = b.1 * (TAU * (c.1 * t + d.1)).cos() + a.1;
    let b = b.2 * (TAU * (c.2 * t + d.2)).cos() + a.2;

    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}