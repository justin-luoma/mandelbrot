use std::fs::File;
use gif::Repeat::Infinite;
use itertools_num::linspace;
use mandelbrot::{color_scale::ContinuousColorScale, flatten_array, mandelbrot::{Mandelbrot, MandelbrotConfig, Viewport}};
use mandelbrot::complex_number::ComplexNumber;


fn main() {
    let dimensions = (1000, 1000);
    let (w, h) = dimensions;
    let f = 5;
    let iter = 1000;
    let mut file = File::create("mandelbrot.gif").expect("Failed to create file");

    let mut encoder = gif::Encoder::new(&mut file, w as u16, h as u16, &[]).unwrap();

    encoder.set_repeat(Infinite).unwrap();

    let a_x = -2.;
    let z_x_b = a_x - -0.015;

    let z_y = 0.0025;

    let a = ComplexNumber::new(a_x, 1.15);
    let b = ComplexNumber::new(0.5, -1.15);

    let z_a = ComplexNumber::new(a_x, z_y);
    let z_b = ComplexNumber::new(z_x_b, -z_y);

    let a_re_range: Vec<_> = linspace(a.r, z_a.r, f).collect();
    let a_im_range: Vec<_> = linspace(a.i, z_a.i, f).collect();
    let b_re_range: Vec<_> = linspace(b.r, z_b.r, f).collect();
    let b_im_range: Vec<_> = linspace(b.i, z_b.i, f).collect();

    for i in 0..f {
        let top_left = ComplexNumber::new(a_re_range[i], a_im_range[i]);
        let bottom_right = ComplexNumber::new(b_re_range[i], b_im_range[i]);

        let viewport = Viewport::<f64> {
            top_left,
            bottom_right,
            width: w as f64,
            height: h as f64,
        };

        let config = MandelbrotConfig::<u8> {
            dimensions,
            viewport,
            color_fn: ContinuousColorScale::get_color_fn_boxed(140.0, 1.0, 1.0),
        };

        let mut mandelbrot = Mandelbrot::new(config);

        mandelbrot.run(iter);

        let pixels = mandelbrot.get_pixels();

        let mut frame = gif::Frame::from_rgba(w as u16, h as u16, &mut flatten_array(pixels));
        frame.delay = 50;

        encoder.write_frame(&frame).unwrap();
    }
}
