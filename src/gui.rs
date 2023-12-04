use std::fmt::{Debug, UpperHex};
use std::ops::{Deref, MulAssign};
use std::str::FromStr;

use num_traits::{AsPrimitive, Bounded, Float, Unsigned, Zero};

use fractal_generator_gui::{BoxedPrimitive, Complex, Generator, GeneratorConfigOld, GeneratorSetting, GeneratorSettings, GeneratorValue, RgbaData};

use crate::complex_number::ComplexNumber;
use crate::config::MandelbrotConfig;
use crate::config::viewport::Viewport;
use crate::gui::settings::Iterations;
use crate::mandelbrot::Mandelbrot;
use crate::pixel::Pixel;

impl<P, F> Generator for Mandelbrot<P, F>
    where
        P: 'static + Unsigned + Bounded + UpperHex + Copy + Zero + Send + Sync + Sync + Debug +
        Into<f64>,
        F: 'static + Float + From<f64> + Into<f64> + MulAssign + FromStr + From<u32> +
        From<i32> + Send + Sync + Debug,
        f64: From<P> + From<F> + AsPrimitive<P>
{
    type B = Pixel<P>;
    type T = F;
    // type S = GeneratorSetting;

    type C = MandelbrotConfig<P, F>;

    fn new(config: Self::C, max_iterations: u32) -> Self {
        Mandelbrot::new(config, max_iterations)
    }

    fn data(&self) -> Vec<Self::B> {
        self.get_pixels().into_iter().flat_map(|col| col.into_iter()).collect()
    }

    fn zoom(&mut self, center: (u32, u32), radius: u32) -> (
        Box<dyn Complex<T=F>>,
        Box<dyn Complex<T=F>>
    ) {
        let (tl, br) = self.zoom(center, radius);
        (Box::new(tl), Box::new(br))
    }

    // fn settings(&self) -> Vec<GeneratorSetting> {
    //     let mut settings = Vec::new();
    //
    //     let iterations = Iterations(self.max_iterations);
    //     let viewport = self.config.viewport;
    //
    //     settings.push(iterations.into());
    //     settings.push(viewport.into());
    //
    //     settings
    // }

    fn reset(&mut self) {
        self.reset()
    }

    fn recalculate(&mut self, refresh: bool) {
        self.recalculate(!refresh)
    }

    fn redraw(&mut self) {
        self.redraw()
    }

    fn viewport(&self) -> &dyn fractal_generator_gui::Viewport<T=Self::T> {
        &self.config().viewport
    }
}

impl<P, F> GeneratorSettings for Mandelbrot<P, F>
    where
        P: 'static + Unsigned + Bounded + UpperHex + Copy + Zero + Send + Sync + Sync + Debug +
        Into<f64>,
        F: 'static + Float + From<f64> + Into<f64> + MulAssign + FromStr + From<u32> +
        From<i32> + Send + Sync + Debug,
{
    fn settings(&self) -> Vec<GeneratorSetting> {
        vec![
            Iterations::from(self.max_iterations).into(),
            self.config.viewport.into(),
        ]
    }

    fn update_settings(&mut self, settings: &[GeneratorSetting]) {
        settings
            .iter()
            .for_each(|s| {
                match s.label.as_str() {
                    "iterations" => {
                        if let GeneratorValue::Range((iters, _, _, _)) = &s.value {
                            self.max_iterations = iters.to_u32().unwrap();
                        }
                    }
                    "viewport" => {
                        if let GeneratorValue::Viewport(viewport) = &s.value {
                            self.config.viewport = Viewport::from(viewport);
                        }
                    }
                    _ => unreachable!(),
                }
            });
    }
}

impl<P, F> GeneratorConfigOld for MandelbrotConfig<P, F>
    where
        P: Unsigned + Bounded + UpperHex + Copy + Zero + Send + Sync + 'static,
        F: Float + Send + Sync + 'static,
{
    type C = MandelbrotConfig<P, F>;
}

impl<P> From<Pixel<P>> for [u8; 4]
    where P: 'static + Unsigned + Bounded + Send + Sync + Into<f64> + UpperHex + Copy + Into<f32>
    + Into<u8>,
{
    fn from(value: Pixel<P>) -> Self {
        [
            value.r().into(),
            value.g().into(),
            value.b().into(),
            value.a().into(),
        ]
    }
}

impl<P> RgbaData for Pixel<P>
    where P: 'static + Unsigned + Bounded + Send + Sync + Into<f64> + UpperHex + Copy + Into<f32> + Into<u8>,
{
    type T = P;

    fn r(&self) -> Self::T {
        self.r()
    }

    fn g(&self) -> Self::T {
        self.g()
    }

    fn b(&self) -> Self::T {
        self.b()
    }

    fn a(&self) -> Self::T {
        self.a()
    }
}

impl<T> Complex for ComplexNumber<T> where T: Float + Send + Sync + 'static {
    type T = T;
    fn real(&self) -> T {
        self.r
    }

    fn imaginary(&self) -> T {
        self.i
    }
}

impl<T> fractal_generator_gui::Viewport for Viewport<T> where T: Float + Send + Sync + 'static {
    type T = T;
    fn top_left(&self) -> &dyn Complex<T=T> {
        &self.top_left
    }

    fn bottom_right(&self) -> &dyn Complex<T=T> {
        &self.bottom_right
    }

    fn width(&self) -> T {
        self.width
    }

    fn height(&self) -> T {
        self.height
    }
}

pub mod settings {
    use num_traits::Float;
    use fractal_generator_gui::{GeneratorSetting, GeneratorValue, GeneratorViewport};
    use crate::complex_number::ComplexNumber;
    use crate::config::viewport::Viewport;

    #[derive(Clone)]
    pub struct Iterations {
        value: u32,
        min: u32,
        max: u32,
        step: u32,
    }

    impl Iterations {
        pub fn from(value: u32) -> Self {
            Self {
                value,
                ..Default::default()
            }
        }
    }

    impl Default for Iterations {
        fn default() -> Self {
            Self {
                value: 1000,
                min: 1,
                max: u32::MAX,
                step: 1,
            }
        }
    }

    impl From<Iterations> for GeneratorValue {
        fn from(value: Iterations) -> Self {
            Self::Range((
                Box::new(value.value),
                Box::new(value.min),
                Box::new(value.max),
                Some(Box::new(value.step)),
            ))
        }
    }

    impl From<Iterations> for GeneratorSetting {
        fn from(value: Iterations) -> Self {
            Self::new(
                "iterations".to_string(),
                Iterations::default().into(),
                value.into(),
            )
        }
    }

    impl<F> From<Viewport<F>> for GeneratorValue
        where F: Float + Send + Sync
    {
        fn from(value: Viewport<F>) -> Self {
            Self::Viewport(value.into())
        }
    }

    impl<F> From<Viewport<F>> for GeneratorSetting
        where F: Float + Send + Sync
    {
        fn from(value: Viewport<F>) -> Self {
            Self::new(
                "viewport".to_string(),
                GeneratorValue::Viewport(Viewport::<F>::default().into()),
                GeneratorValue::Viewport(value.into()),
            )
        }
    }

    impl<F> From<Viewport<F>> for GeneratorViewport where F: Float + Send + Sync {
        fn from(value: Viewport<F>) -> Self {
            GeneratorViewport::new(
                (value.top_left.r, value.top_left.i),
                (value.top_left.r, value.top_left.i),
                value.width,
                value.height,
            )
        }
    }

    // impl<F> From<GeneratorViewport> for Viewport<F> where F: Float + Send + Sync {
    //     fn from(value: GeneratorViewport) -> Self {
    //         Self {
    //             top_left: ComplexNumber {},
    //             bottom_right: ComplexNumber {},
    //             width: (),
    //             height: (),
    //         }
    //     }
    // }

    impl<F> Viewport<F> where F: Float + Send + Sync + From<f64> {
        pub fn from(value: &GeneratorViewport) -> Self {
            let tl_a: f64 = value.top_left.0.to_f64().unwrap();
            let tl_b = value.top_left.0.to_f64().unwrap();
            Self {
                top_left: ComplexNumber::new(
                    tl_a.into(),
                    tl_b.into(),
                ),
                bottom_right: ComplexNumber::new(
                    value.bottom_right.0.to_f64().unwrap().into(),
                    value.bottom_right.0.to_f64().unwrap().into(),
                ),
                width: value.width.to_f64().unwrap().into(),
                height: value.height.to_f64().unwrap().into(),
            }
        }
    }
}
