use num_traits::{AsPrimitive, Bounded, One, Unsigned, Zero};
use std::{fmt::UpperHex, marker::Sized};

#[derive(Clone)]
pub struct Pixel<T: Unsigned + Bounded> {
    r: T,
    g: T,
    b: T,
    a: T,
}

pub trait PixelMath<T: 'static + Unsigned + Bounded + Copy> {
    fn default() -> Self;
    fn from_hsb(hue: f64, saturation: f64, brightness: f64) -> Result<Self, String>
    where
        Self: Sized,
        f64: From<T> + AsPrimitive<T>,
        T: Into<f64>;

    fn new(r: T, g: T, b: T) -> Self;
    fn new_rgba(r: T, g: T, b: T, a: T) -> Self;

    fn set_alpha(&mut self, a: T) -> &Self;
    fn set_r(&mut self, r: T) -> &Self;
    fn set_g(&mut self, g: T) -> &Self;
    fn set_b(&mut self, b: T) -> &Self;

    fn set_rgb(&mut self, r: T, g: T, b: T) -> &Self;
    fn set_rgba(&mut self, r: T, g: T, b: T, a: T) -> &Self;

    fn get_tuple(&self) -> (T, T, T, T);
    fn get_vector(&self) -> Vec<T>;
    fn get_slice(&self) -> [T; 4];

    fn to_hex(&self) -> String;
    fn to_hsv(&self) -> (T, T, T);
}

impl<T: 'static + Unsigned + Bounded + UpperHex + Zero + One + Copy + Into<f64>> PixelMath<T>
    for Pixel<T>
{
    fn default() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    /// hue is in degrees, saturation and brightness are between 0 and 1
    fn from_hsb(hue: f64, saturation: f64, brightness: f64) -> Result<Self, String>
    where
        f64: From<T> + AsPrimitive<T>,
        T: Into<f64>,
    {
        if saturation > 1f64 || brightness > 1f64 {
            return Err(format!(
                "Invalid HSB values: {} {} {}",
                hue, saturation, brightness
            ));
        }

        let hh: f64;
        let pp: f64;
        let qq: f64;
        let tt: f64;
        let ff: f64;
        let vv: f64;
        let ii: u64;

        let (r, g, b) = if saturation <= 0f64 {
            (brightness, brightness, brightness)
        } else {
            hh = (hue % 360f64) / 60f64;
            ii = hh as u64;
            ff = hh - hh.floor();
            pp = brightness * (1f64 - saturation);
            qq = brightness * (1f64 - (saturation * ff));
            tt = brightness * (1f64 - (saturation * (1f64 - ff)));
            vv = brightness;

            match ii {
                0 => (vv, tt, pp),
                1 => (qq, vv, pp),
                2 => (pp, vv, tt),
                3 => (pp, qq, vv),
                4 => (tt, pp, vv),
                _ => (vv, pp, qq),
            }
        };

        let max = T::max_value().into();

        Ok(Self::new(
            (r * max).round().as_(),
            (g * max).round().as_(),
            (b * max).round().as_(),
        ))
    }

    fn new(r: T, g: T, b: T) -> Self {
        Self::new_rgba(r, g, b, T::max_value())
    }

    fn new_rgba(r: T, g: T, b: T, a: T) -> Self {
        Pixel { r, g, b, a }
    }

    fn set_alpha(&mut self, a: T) -> &Self {
        self.a = a;
        self
    }

    fn set_r(&mut self, r: T) -> &Self {
        self.r = r;
        self
    }

    fn set_g(&mut self, g: T) -> &Self {
        self.g = g;
        self
    }

    fn set_b(&mut self, b: T) -> &Self {
        self.b = b;
        self
    }

    fn set_rgb(&mut self, r: T, g: T, b: T) -> &Self {
        self.r = r;
        self.g = g;
        self.b = b;

        self
    }

    fn set_rgba(&mut self, r: T, g: T, b: T, a: T) -> &Self {
        self.set_rgb(r, g, b);
        self.a = a;

        self
    }

    fn get_tuple(&self) -> (T, T, T, T) {
        (self.r, self.g, self.b, self.a)
    }

    fn get_vector(&self) -> Vec<T> {
        vec![self.r, self.g, self.b, self.a]
    }

    fn get_slice(&self) -> [T; 4] {
        [self.r, self.g, self.b, self.a]
    }

    fn to_hex(&self) -> String {
        format!("{:#X}{:X}{:X}{:X}", self.r, self.g, self.b, self.a)
    }

    fn to_hsv(&self) -> (T, T, T) {
        unimplemented!();
    }
}

pub struct IntoPixel<'a, T: 'a + Unsigned + Bounded> {
    px: &'a Pixel<T>,
    remaining: u8,
}

impl<'a, T: Unsigned + Bounded> IntoPixel<'a, T> {
    pub fn new(px: &'a Pixel<T>) -> Self {
        IntoPixel {
            px: &px,
            remaining: 5,
        }
    }
}

impl<'a, T: Unsigned + Bounded + Copy> Iterator for IntoPixel<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.remaining -= 1;

        match self.remaining {
            r @ 1..=4 => Some(match r {
                4 => self.px.r,
                3 => self.px.g,
                2 => self.px.b,
                _ => self.px.a,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{IntoPixel, Pixel, PixelMath};

    #[test]
    fn pixel_iterator() {
        let px = Pixel::new(9u8, 234, 5);

        let iter = IntoPixel::new(&px);

        for i in iter {
            println!("{:?}", i);
        }
    }

    fn test_hsb_to_rgb(h: f64, s: f64, v: f64, r: u8, g: u8, b: u8) {
        assert_eq!(
            (r, g, b, 255u8),
            Pixel::<u8>::from_hsb(h, s, v).unwrap().get_tuple()
        )
    }

    #[test]
    fn from_hsb() {
        test_hsb_to_rgb(360.0, 1.0, 1.0, 255, 0, 0);
        test_hsb_to_rgb(250.0, 1.0, 1.0, 43, 0, 255);
        test_hsb_to_rgb(360.0, 0.5, 0.5, 128, 64, 64);
        test_hsb_to_rgb(200.0, 0.5, 0.5, 64, 106, 128);
        test_hsb_to_rgb(0.0, 0.5, 0.5, 128, 64, 64);
        test_hsb_to_rgb(0.0, 0.0, 0.5, 128, 128, 128);
        test_hsb_to_rgb(0.0, 0.5, 0.0, 0, 0, 0);
        test_hsb_to_rgb(0.0, 0.0, 0.0, 0, 0, 0);
    }
}
