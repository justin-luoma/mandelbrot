use std::fs::File;
use std::io::BufWriter;

use png::HasParameters;

use mandelbrot::color_scale::ContinuousColorScale;
use mandelbrot::complex_number::ComplexNumber;
use mandelbrot::flatten_array;
use mandelbrot::mandelbrot::{Mandelbrot, MandelbrotConfig, Viewport};

fn main() {
    let dimensions = (1000, 1000);
    let (w, h) = dimensions;
    let frames = 5000;

    let file = File::create("mandelbrot.png").expect("Failed to create file");
    let buf = BufWriter::new(file);
    let mut encoder = png::Encoder::new(buf, w, h);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let width = (w) as f64;
    let height = (h) as f64;

    let a_x = -2.;
    let a = ComplexNumber::new(a_x, 1.15);
    let b = ComplexNumber::new(0.5, -1.15);

    let viewport = Viewport::<f64> {
        top_left: a,
        bottom_right: b,
        width,
        height,
    };

    let config = MandelbrotConfig::<u8> {
        dimensions,
        viewport,
        color_fn: ContinuousColorScale::get_color_fn_boxed(200.0, 1.0, 1.0),
    };

    let mut mandelbrot = Mandelbrot::new(config);

    mandelbrot.run(frames);

    let data = mandelbrot.get_pixels();

    writer
        .write_image_data(flatten_array(data).as_slice())
        .unwrap();
}
