use std::cmp;
use std::convert::From;
use std::fmt::{Debug, UpperHex};
use std::ops::MulAssign;
use std::str::FromStr;

use itertools_num::linspace;
use num_traits::{AsPrimitive, Bounded, Float, Num, sign::Unsigned, Zero};
use rayon::prelude::*;

#[cfg(feature = "gui")]
use {bevy_ecs::prelude::Resource, fractal_generator_gui::GeneratorSettingsOld};

use crate::color_scale::ContinuousColorScale;
use crate::complex_number::ComplexNumber;
use crate::config::MandelbrotConfig;
use crate::config::viewport::Viewport;
use crate::pixel::{Pixel, PixelMath};

#[cfg_attr(feature = "gui", derive(Resource))]
pub struct Mandelbrot<P: Unsigned + Bounded + UpperHex + Copy + Zero + Send + Sync + Sync, F:
Float + Send + Sync + 'static> {
    pub(crate) config: MandelbrotConfig<P, F>,
    pixels: Vec<Vec<Pixel<P>>>,
    coords: (Vec<F>, Vec<F>),
    values: Vec<Vec<(u32, ComplexNumber<F>)>>,
    steps: (ComplexNumber<F>, ComplexNumber<F>),
    iterations: u32,
    pub(crate) max_iterations: u32,
}

impl<P: 'static + Unsigned + Bounded + Debug + UpperHex + Copy + Zero + Send + Sync +
Into<f64>, F: Num + Float + Debug + Send + Sync>
Mandelbrot<P, F>
    where f64: From<P> + AsPrimitive<P> + Into<F> + From<F>,
          F: From<f64> + Into<f64> + MulAssign + FromStr + From<u32> + From<i32>
{
    pub fn new(config: MandelbrotConfig<P, F>, max_iterations: u32) -> Mandelbrot<P, F> {
        let (w, h) = config.dimensions;

        let top_left = config.viewport.top_left;
        let bottom_right = config.viewport.bottom_right;
        let w_c = ComplexNumber::new(bottom_right.r, 0.0.into());
        let h_c = ComplexNumber::new(0.0.into(), bottom_right.i);

        let re_range = linspace(top_left.r, bottom_right.r, w as usize).collect();
        let im_range = linspace(top_left.i, bottom_right.i, h as usize).collect();

        Mandelbrot {
            config,
            pixels: vec![vec![Pixel::<P>::default(); w as usize]; h as usize],
            values: vec![vec![(0, ComplexNumber::new(0.0.into(), 0.0.into())); w as usize]; h as
                usize],
            steps: (w_c / w, h_c / h),
            iterations: max_iterations,
            max_iterations: 0,
            coords: (re_range, im_range),
        }
    }

    /// Returns a reference to the current state of the Pixels in the
    /// Mandelbrot Set
    pub fn get_pixels(&self) -> Vec<Vec<Pixel<P>>> {
        self.pixels.clone()
    }

    pub fn julia_set(&mut self, iterations: u32) {
        for (y, im) in self.coords.1.iter().enumerate() {
            for (x, re) in self.coords.0.iter().enumerate() {
                let (iters, z) = Self::julia(*re, *im, iterations);
                self.pixels[y][x] = (self.config.color_fn)(iters, z, iterations);
            }
        }
    }

    fn julia(x: F, y: F, iterations: u32) -> (u32, ComplexNumber<F>)
        where f64: Into<F> {
        let mut z = ComplexNumber::new(x, y);
        let c = ComplexNumber::new((0.38).into(), (0.28).into());
        let mut i = 0;
        while i < iterations && z.norm_sqr() < (32.).into() {
            z = z * z + c;
            i += 1;
        }

        (i, z)
    }

    pub fn get_xy_complex(&self, x: usize, y: usize) -> Option<ComplexNumber<f64>> {
        // self.values.get(y).and_then(|row| row.get(x).map(|v| v.1))
        match (self.coords.0.get(x), self.coords.1.get(y)) {
            (Some(x), Some(y)) => Some(ComplexNumber::new((*x).into(), (*y).into())),
            _ => None
        }
    }

    pub fn run(&mut self, iters: u32) {
        self.iterations = iters;

        self.recalculate(true);

        self.redraw();
    }

    pub fn config(&self) -> &MandelbrotConfig<P, F> {
        &self.config
    }

    pub fn zoom(&mut self, center: (u32, u32), radius: u32) -> (ComplexNumber<F>,
                                                                ComplexNumber<F>) {
        let width = self.config.dimensions.0;
        let height = self.config.dimensions.1;

        let x1 = center.0 as i32 - radius as i32 - 1;
        let y1 = center.1 as i32 - radius as i32 - 1;
        let x2 = center.0 + radius - 1;
        let y2 = center.1 + radius - 1;

        let mut viewport = self.config.viewport;

        let width = width as i32;
        let height = height as i32;
        // match (x1, y1, x2 as i32, y2 as i32) {
        //     (x_range, y_range, x_range, y_range) => {
        //         viewport.top_left = top_left.unwrap();
        //         viewport.bottom_right = bottom_right.unwrap();
        //     }
        //     (x_range, y_range, width.., y_range) => {
        //         let neighbor1 = self.coords.0[width as usize - 2];
        //         let neighbor2 = self.coords.0[width as usize - 1];
        //         let re_diff = Self::xy_diff(neighbor1, neighbor2);
        //         let re = re_diff * (x2 as f64 - width as f64) + self.coords.0[width as usize - 1];
        //         let im = self.coords.1[y2 as usize];
        //         viewport.top_left = top_left.unwrap();
        //         viewport.bottom_right = ComplexNumber::new(re, im);
        //     }
        //     (x_range, y_range, x_range, height..) => {
        //         let neighbor1 = self.coords.1[height as usize - 2];
        //         let neighbor2 = self.coords.1[height as usize - 1];
        //         let im_diff = Self::xy_diff(neighbor1, neighbor2);
        //         let im = im_diff * (y2 as f64 - height as f64) + self.coords.1[height as usize - 1];
        //         let re = self.coords.0[x2 as usize];
        //         viewport.top_left = top_left.unwrap();
        //         viewport.bottom_right = ComplexNumber::new(re, im);
        //     }
        //     (x_range, y_range, width.., height..) => {}
        //     (i32::MIN..=-1, y_range, x_range, y_range) => {}
        //     (x_range, i32::MIN..=-1, x_range, y_range) => {}
        //     (i32::MIN..=-1, i32::MIN..=-1, x_range, y_range) => {}
        //     (i32::MIN..=-1, y_range, width.., y_range) => {}
        //     (x_range, i32::MIN..=-1, x_range, height..) => {}
        //     (i32::MIN..=-1, i32::MIN..=-1, width.., height..) => {}
        // }

        let re1 = if x1 < 0 {
            let neighbor1 = self.coords.0[1];
            let neighbor2 = self.coords.0[0];
            let re_diff = Self::xy_diff(neighbor1, neighbor2);
            re_diff * (x1.abs()).into() + self.coords.0[0]
        } else if x1 < width {
            self.coords.0[x1 as usize]
        } else {
            let neighbor1 = self.coords.0[width as usize - 2];
            let neighbor2 = self.coords.0[width as usize - 1];
            let re_diff = Self::xy_diff(neighbor1, neighbor2);
            re_diff * (x1 - width).into() + self.coords.0[width as usize - 1]
        };

        let im1 = if y1 < 0 {
            let neighbor1 = self.coords.1[0];
            let neighbor2 = self.coords.1[1];
            let im_diff = Self::xy_diff(neighbor1, neighbor2);
            im_diff * y1.abs().into() + self.coords.1[0]
        } else if y1 < height {
            self.coords.1[y1 as usize]
        } else {
            let neighbor1 = self.coords.1[height as usize - 2];
            let neighbor2 = self.coords.1[height as usize - 1];
            let im_diff = Self::xy_diff(neighbor1, neighbor2);
            im_diff * (y1 - height).into() + self.coords.0[height as usize - 1]
        };

        let re2 = if x2 < width as u32 {
            self.coords.0[x2 as usize]
        } else if x2 >= width as u32 {
            let neighbor1 = self.coords.0[width as usize - 2];
            let neighbor2 = self.coords.0[width as usize - 1];
            let re_diff = Self::xy_diff(neighbor1, neighbor2);
            re_diff * (x2 as i32 - width).into() + self.coords.0[width as usize - 1]
        } else {
            unreachable!()
        };

        let im2 = if y2 < height as u32 {
            self.coords.1[y2 as usize]
        } else if y2 >= height as u32 {
            let neighbor1 = self.coords.1[height as usize - 2];
            let neighbor2 = self.coords.1[height as usize - 1];
            let im_diff = Self::xy_diff(neighbor1, neighbor2);
            im_diff * (y2 as i32 - height).into() + self.coords.1[height as usize - 1]
        } else {
            unreachable!()
        };

        let top_left = ComplexNumber::new(re1, im1);
        viewport.top_left = top_left;
        let bottom_right = ComplexNumber::new(re2, im2);
        viewport.bottom_right = bottom_right;

        self.update(viewport);

        (top_left, bottom_right)
        // if let (Some(tl), Some(br)) = (top_left, bottom_right) {
        //     viewport.top_left = tl;
        //     viewport.bottom_right = br;
        //     self.update(viewport);
        // } else if let Some(tl) = top_left {
        //     let re = if x2 > max_x {
        //         let neighbor1 = self.coords.0[max_x as usize - 2];
        //         let neighbor2 = self.coords.0[max_x as usize - 1];
        //         let re_diff = Self::xy_diff(neighbor1, neighbor2);
        //         // let re_diff = Self::extend_plane(x2 as i32, o1.r, o2.r);
        //         dbg!(&re_diff);
        //         re_diff * (x2 as f64 - max_x as f64) + self.coords.0[max_x as usize - 1]
        //     } else {
        //         self.coords.0[x2 as usize]
        //     };
        //     let im = if y2 > height {
        //         let neighbor1 = self.coords.1[height as usize - 2];
        //         let neighbor2 = self.coords.1[height as usize - 1];
        //         let im_diff = Self::xy_diff(neighbor1, neighbor2);
        //         dbg!(&im_diff);
        //         im_diff * (y2 as f64 - height as f64) + self.coords.1[height as usize - 1]
        //     } else {
        //         self.coords.1[y2 as usize]
        //     };
        //     viewport.top_left = tl;
        //     viewport.bottom_right = ComplexNumber::new(re, im);
        //     self.update(viewport);
        //     return;
        // } else if let Some(br) = bottom_right {
        //     let re = if x1 < 0 {
        //         let neighbor1 = self.coords.0[0];
        //         let neighbor2 = self.coords.0[1];
        //         let re_diff = Self::xy_diff(neighbor1, neighbor2);
        //         dbg!(&re_diff);
        //         re_diff * (x1.abs()) as f64 + self.coords.0[0]
        //     } else {
        //         self.coords.0[x1 as usize]
        //     };
        //     let im = if y1 < 0 {
        //         let neighbor1 = self.coords.1[0];
        //         let neighbor2 = self.coords.1[1];
        //         let im_diff = Self::xy_diff(neighbor1, neighbor2);
        //         dbg!(&im_diff);
        //         im_diff * (y1.abs()) as f64 + self.coords.1[0]
        //     } else {
        //         self.coords.1[y1 as usize]
        //     };
        //     viewport.top_left = ComplexNumber::new(re, im);
        //     viewport.bottom_right = br;
        //     self.update(viewport);
        //     return;
        // } else {
        //
        // }

        // if x1 >= 0 && y1 >= 0 {
        //     let top_left = self.get_xy_complex(x1 as usize, y1 as usize);
        //     let bottom_right = self.get_xy_complex(x2 as usize, y2 as usize);
        //
        //     if let (Some(tl), Some(br)) = (top_left, bottom_right) {
        //         viewport.top_left = tl;
        //         viewport.bottom_right = br;
        //         self.update(viewport);
        //     } else if let Some(tl) = top_left {
        //         let o1 = self.get_xy_complex(width as usize - 2, height as usize - 2).unwrap();
        //         let o2 = self.get_xy_complex(width as usize - 1, height as usize - 1).unwrap();
        //         let re1 = o1.r;
        //         let im1 = o1.i;
        //         let re2 = o2.r;
        //         let im2 = o2.i;
        //         let re_diff = if re1 > re2 {
        //             re1 - re2
        //         } else {
        //             re2 - re1
        //         };
        //         let im_diff = if im1 > im2 {
        //             im1 - im2
        //         } else {
        //             im2 - im1
        //         };
        //         if x2 > width && y2 > height {
        //             let x_diff = x2 - width;
        //             let y_diff = y2 - height;
        //             let mut re = re_diff * x_diff as f64;
        //             let mut im = im_diff * y_diff as f64;
        //             if re1 < 0. || re2 < 0. {
        //                 re = -re;
        //             };
        //             re += self.coords.0[width as usize - 1];
        //             if im1 < 0. || im2 < 0. {
        //                 im = -im;
        //             };
        //             im += self.coords.1[height as usize - 1];
        //             viewport.top_left = tl;
        //             viewport.bottom_right = ComplexNumber::new(re, im);
        //             self.update(viewport);
        //         } else if x2 > width {
        //             let x_diff = x2 - width;
        //             let mut re = re_diff * x_diff as f64;
        //             if re1 < 0. || re2 < 0. {
        //                 re = -re;
        //             };
        //             re += self.coords.0[width as usize - 1];
        //             let im = self.coords.1[y2 as usize];
        //             viewport.top_left = tl;
        //             viewport.bottom_right = ComplexNumber::new(re, im);
        //             self.update(viewport);
        //         } else if y2 > height {
        //             let y_diff = y2 - height;
        //             let mut im = im_diff * y_diff as f64;
        //             if im1 < 0. || im2 < 0. {
        //                 im = -im;
        //             };
        //             im += self.coords.1[height as usize - 1];
        //             let re = self.coords.0[x2 as usize];
        //             viewport.top_left = tl;
        //             viewport.bottom_right = ComplexNumber::new(re, im);
        //             self.update(viewport);
        //         }
        //     }
        // } else {
        //     let o1 = self.get_xy_complex(0, 0).unwrap();
        //     let o2 = self.get_xy_complex(1, 1).unwrap();
        //     if x1 < 0 && y1 >= 0 {
        //         let re_diff = Self::extend_plane(x1, o1.r, o2.r);
        //         dbg!((&re_diff));
        //         let re = re_diff + self.coords.0[0];
        //         let im = self.coords.1[y1 as usize];
        //
        //         viewport.top_left = ComplexNumber::new(re, im);
        //     } else if y1 < 0 && x1 >= 0 {
        //         let im_diff = Self::extend_plane(y1, o1.i, o2.i);
        //         dbg!((&im_diff));
        //         let im = im_diff + self.coords.1[0];
        //         let re = self.coords.0[x1 as usize];
        //
        //         viewport.top_left = ComplexNumber::new(re, im);
        //     } else {
        //         let re_diff = Self::extend_plane(x1, o1.r, o2.r);
        //         let im_diff = Self::extend_plane(y1, o1.i, o2.i);
        //         dbg!((&re_diff, &im_diff));
        //         let re = re_diff + self.coords.0[0];
        //         let im = im_diff + self.coords.1[0];
        //
        //         viewport.top_left = ComplexNumber::new(re, im);
        //     }
        //     dbg!(&viewport.top_left);
        //     let bottom_right = self.get_xy_complex(x2 as usize, y2 as usize).unwrap();
        //     viewport.bottom_right = bottom_right;
        //     self.update(viewport);
        // }


        // let re_range: Vec<_> = linspace(tl.r, br.r, width as usize).collect();
        // let im_range: Vec<_> = linspace(tl.i, br.i, height as usize).collect();


        // for (y, im) in new_im_range.iter().enumerate() {
        //     for (x, re) in new_re_range.iter().enumerate() {
        //         if y <= y2 as usize && y >= y1 as usize && x <= x2 as usize && x >= x1 as usize {
        //             self.coords.0[x] = ;
        //             self.coords.1[y] = *im;
        //             self.pixels[y][x] = Pixel::<P>::default();
        //             self.values[y][x] = (0, ComplexNumber::new(0., 0.));
        //         }
        //         self.coords.0[x] = *re;
        //         self.coords.1[y] = *im;
        //         self.pixels[y][x] = Pixel::<P>::default();
        //         self.values[y][x] = (0, ComplexNumber::new(0., 0.));
        //     }
        // }


        // let filter_fn = |axis_max| {
        //     move |(i, v)| {
        //         let i = i as i32;
        //         let radius = radius as i32;
        //         if i - radius < 0 || i + radius > axis_max {
        //             None
        //         } else {
        //             Some(v)
        //         }
        //     }
        // };

        // self.coords.0 = self.coords.0
        //     .iter()
        //     .cloned()
        //     .enumerate()
        //     .filter_map(filter_fn(width as i32))
        //     .collect();
        //
        // self.coords.1 = self.coords.1
        //     .iter()
        //     .cloned()
        //     .enumerate()
        //     .filter_map(filter_fn(height as i32))
        //     .collect();
    }

    fn xy_diff(neighbor1: F, neighbor2: F) -> F {
        let mut re_diff = if neighbor1 > neighbor2 {
            neighbor1 - neighbor2
        } else {
            neighbor2 - neighbor1
        };
        if neighbor1 < (0.).into() || neighbor2 < (0.).into() {
            re_diff *= (-1.).into();
        }
        re_diff
    }

    fn extend_plane(extended: i32, neighbor1: f64, neighbor2: f64) -> f64 {
        let xy_diff = extended.abs();
        let step = if neighbor1 > neighbor2 {
            neighbor1 - neighbor2
        } else {
            neighbor2 - neighbor1
        };
        let mut xy = step * xy_diff as f64;
        if neighbor1 < 0. || neighbor2 < 0. {
            xy = -xy;
        }
        xy
    }

    pub fn update(&mut self, viewport: Viewport<F>) {
        dbg!(&viewport);
        let (w, h) = self.config.dimensions;

        let top_left = viewport.top_left;
        let bottom_right = viewport.bottom_right;
        let w_c = ComplexNumber::new(bottom_right.r, 0.0.into());
        let h_c = ComplexNumber::new(0.0.into(), bottom_right.i);
        let re_range = linspace(top_left.r, bottom_right.r, w as usize).collect();
        let im_range = linspace(top_left.i, bottom_right.i, h as usize).collect();

        self.config.viewport = viewport;
        self.steps = (w_c / w, h_c / h);
        self.pixels = vec![vec![Pixel::<P>::default(); w as usize]; h as usize];
        self.values = vec![vec![(0, ComplexNumber::new((0.0).into(), (0.0).into())); w as usize]; h as
            usize];
        self.coords = (re_range, im_range);
    }

    pub fn update_config(&mut self, config: MandelbrotConfig<P, F>) {
        let viewport = config.viewport;
        self.config = config;
        self.update(viewport);
    }

    #[cfg(feature = "gui")]
    pub fn update_settings(&mut self, settings: &GeneratorSettingsOld) {
        dbg!(&settings);
        if let Some(hue) = settings.hue {
            self.config.color_fn = ContinuousColorScale::get_color_fn_boxed(hue, 1., 1.);
        }
        if let Some(exponent) = settings.exponent {
            self.config.exponent = exponent;
        }
        if settings.x1.is_some() || settings.y1.is_some() || settings.x2.is_some() || settings.y2.is_some() {
            let mut viewport = self.config.viewport;
            if let Some(Ok(x1)) = settings.x1.clone().map(|v| v.parse()) {
                viewport.top_left.r = x1;
            }
            if let Some(Ok(y1)) = settings.y1.clone().map(|v| v.parse()) {
                viewport.top_left.i = y1;
            }
            if let Some(Ok(x2)) = settings.x2.clone().map(|v| v.parse()) {
                viewport.bottom_right.r = x2;
            }
            if let Some(Ok(y2)) = settings.y2.clone().map(|v| v.parse()) {
                viewport.bottom_right.i = y2;
            }
            self.config.viewport = viewport;
            self.update(viewport);
        }
        if let Some(Ok(iterations)) = settings.iterations.clone().map(|v| v.parse()) {
            self.iterations = iterations;
        }
    }

    pub fn recalculate(&mut self, use_self: bool) {
        let iterations = self.iterations;
        let updates = self.coords.1.par_iter().enumerate().map(|(y, im)| {
            self.coords.0.par_iter().enumerate().map(|(x, re)| {
                (
                    (x, y),
                    if use_self {
                        self.iterate_coordinate(
                            self.values[y][x],
                            ComplexNumber::new(*re, *im),
                            iterations,
                        )
                    } else {
                        self.iterate_coordinate(
                            (0, ComplexNumber::new((0.).into(), (0.).into())),
                            ComplexNumber::new(*re, *im),
                            iterations,
                        )
                    }
                )
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        updates.into_iter().for_each(|v| {
            v.into_iter()
                .for_each(|((x, y), (i, z))| {
                    self.values[y][x] = (i, z);
                    self.max_iterations = cmp::max(self.max_iterations, i);
                });
        });
        // for (y, im) in self.coords.1.iter().enumerate() {
        //     for (x, re) in self.coords.0.iter().enumerate() {
        //         let coordinate = self.iterate_coordinate(
        //             if use_self {
        //                 self.values[y][x]
        //             } else {
        //                 (0, ComplexNumber::new((0.).into(), (0.).into()))
        //             },
        //             // top_left + d_w * im + d_h * re,
        //             ComplexNumber::new((*re).into(), (*im).into()),
        //             iterations,
        //         );
        //         self.values[y][x] = coordinate;
        //
        //         self.max_iterations = cmp::max(self.max_iterations, self.values[y][x].0);
        //     }
        // }
    }

    pub fn redraw(&mut self) {
        for (r, row) in self.values.iter().enumerate() {
            for (c, (iters, zn)) in row.iter().enumerate() {
                self.pixels[r][c] = (self.config.color_fn)(*iters, *zn, self.max_iterations);
            }
        }
    }

    pub fn reset(&mut self) {
        self.pixels
            .iter_mut()
            .map(|col| col.iter_mut().map(|px| *px = Pixel::<P>::default()))
            .count();
        self.values
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|coor| *coor = (0, ComplexNumber::new((0.0).into(), (0.0).into())))
            })
            .count();
        self.iterations = 0;
        self.max_iterations = 0;
    }

    fn iterate_coordinate(
        &self,
        current_coord: (u32, ComplexNumber<F>),
        c: ComplexNumber<F>,
        limit: u32,
    ) -> (u32, ComplexNumber<F>)
        where
            f64: Into<F> + From<F>,
    {
        let mut count = 0;
        let (finished_iters, z) = current_coord;

        let mut z = if finished_iters == 0 {
            c
        } else {
            z
        };

        while z.norm_sqr() <= (4.).into() && count < limit {
            z = c + z.pow(self.config.exponent);
            count += 1;
        }

        (count + finished_iters, z)
    }
}
