use num_traits::{AsPrimitive, Bounded, Float, Unsigned};
use crate::complex_number::ComplexNumber;
use crate::pixel::{Pixel, PixelMath};
use std::fmt::UpperHex;

pub trait ColorScale {
    fn pixel_color<P: 'static + Unsigned + Float + Bounded + Copy + UpperHex + Send + Sync +
    Into<f64>, T:
    Float + Send + Sync + Into<f64>>(
        iters_to_escape: u32,
        ending_point: ComplexNumber<T>,
        num_iterations: u32,
    ) -> Pixel<P>
        where
            f64: From<P> + AsPrimitive<P> + From<T>,
            T: From<f64>;
}

pub struct ContinuousColorScale {}

impl ColorScale for ContinuousColorScale {
    fn pixel_color<P: 'static + Unsigned + Float + Bounded + Copy + UpperHex + Send + Sync +
    Into<f64>, T:
    Float + Send + Sync + Into<f64>>(
        iters_to_escape: u32,
        ending_point: ComplexNumber<T>,
        num_iterations: u32,
    ) -> Pixel<P>
        where
            f64: From<P> + AsPrimitive<P> + From<T>,
            T: Into<f64> + From<f64>
    {
        ContinuousColorScale::pixel_color_gen(
            iters_to_escape,
            ending_point,
            num_iterations,
            200.95,
            0.8,
            1.0,
            10.0,
        )
    }
}

impl ContinuousColorScale {
    pub fn pixel_color_gen<P: 'static + Unsigned + Bounded + Copy + UpperHex + Into<f64>, T:
    Float + Send + Sync + Into<f64>>(
        iters_to_escape: u32,
        ending_point: ComplexNumber<T>,
        num_iterations: u32,
        hue: f64,
        sat: f64,
        val: f64,
        scale: f64,
    ) -> Pixel<P>
        where
            f64: From<P> + AsPrimitive<P> + Into<T>,
            P: Send + Sync,
    {
        if iters_to_escape == num_iterations {
            return Pixel::new(P::zero(), P::zero(), P::zero());
        }

        let smooth: f64 = iters_to_escape.into();
        let smooth: f64 = (smooth.into() + (1.0).into() - ending_point.abs().log((10.0).into()).log(
            (2.0).into())).into();

        Pixel::from_hsb(hue + scale * smooth, sat, val).unwrap()
    }

    pub fn get_color_fn<P: 'static + Unsigned + Bounded + Copy + UpperHex + Into<f64>>(
        hue: f64,
        sat: f64,
        val: f64,
    ) -> impl Fn(u32, ComplexNumber<f64>, u32) -> Pixel<P>
        where
            f64: From<P> + AsPrimitive<P>,
            P: Send + Sync,
    {
        move |iters_to_escape: u32,
              ending_point: ComplexNumber<f64>,
              num_iterations: u32|
              -> Pixel<P> {
            ContinuousColorScale::pixel_color_gen(
                iters_to_escape,
                ending_point,
                num_iterations,
                hue,
                sat,
                val,
                10.0,
            )
        }
    }

    pub fn get_color_fn_boxed<P: 'static + Unsigned + Bounded + Copy + UpperHex + Send + Sync +
    Into<f64>, T: Float + Send + Sync>(
        hue: f64,
        sat: f64,
        val: f64,
    ) -> Box<dyn Fn(u32, ComplexNumber<T>, u32) -> Pixel<P> + Send + Sync>
        where
            f64: From<P> + AsPrimitive<P> + From<T>,
            T: Send + Sync + From<f64>,
    {
        Box::new(
            move |iters_to_escape: u32,
                  ending_point: ComplexNumber<T>,
                  num_iterations: u32|
                  -> Pixel<P> {
                ContinuousColorScale::pixel_color_gen(
                    iters_to_escape,
                    ending_point,
                    num_iterations,
                    hue,
                    sat,
                    val,
                    10.0,
                )
            },
        )
    }
}

pub struct DiscreteColorScale {}

impl ColorScale for DiscreteColorScale {
    fn pixel_color<P: 'static + Unsigned + Bounded + Copy + UpperHex + Send + Sync + Into<f64>, T:
    Float + Send + Sync>(
        iters_to_escape: u32,
        _ending_point: ComplexNumber<T>,
        max_iterations: u32,
    ) -> Pixel<P> {
        match f64::from(iters_to_escape) / f64::from(max_iterations) {
            p if p < 0.15 => Pixel::new(P::max_value(), P::min_value(), P::min_value()),
            p if p < 0.30 => Pixel::new(P::max_value(), P::max_value(), P::min_value()),
            p if p < 0.45 => Pixel::new(P::min_value(), P::max_value(), P::min_value()),
            p if p < 0.60 => Pixel::new(P::max_value(), P::max_value(), P::max_value()),
            p if p < 0.75 => Pixel::new(P::max_value(), P::min_value(), P::max_value()),
            p if p < 0.80 => Pixel::new(P::max_value(), P::min_value(), P::max_value()),
            p if p < 0.95 => Pixel::new(P::max_value(), P::max_value(), P::max_value()),
            _ => Pixel::new(P::min_value(), P::min_value(), P::min_value()),
        }
    }
}

pub struct SimpleColorScale {}

impl ColorScale for SimpleColorScale {
    fn pixel_color<P: 'static + Unsigned + Bounded + Copy + UpperHex + Send + Sync + Into<f64>, T:
    Float + Send + Sync>(
        iters_to_escape: u32,
        _ending_point: ComplexNumber<T>,
        max_iterations: u32,
    ) -> Pixel<P> {
        if iters_to_escape == max_iterations {
            Pixel::new(P::max_value(), P::min_value(), P::min_value())
        } else {
            Pixel::new(P::min_value(), P::min_value(), P::min_value())
        }
    }
}
