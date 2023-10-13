use std::cell::RefCell;
use std::cmp;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;
use fltk::app;
use fltk::app::{App, Scheme};
use fltk::button::Button;
use fltk::draw::Offscreen;
use fltk::enums::{Align, Color, Event, FrameType};
use fltk::frame::Frame;
use fltk::prelude::*;
use fltk::window::Window;
use fltk::draw::*;
use fltk::group::Flex;
use png::HasParameters;
use mandelbrot::color_scale::ContinuousColorScale;
use mandelbrot::complex_number::ComplexNumber;
use mandelbrot::flatten_array;
use mandelbrot::mandelbrot::{Mandelbrot, MandelbrotConfig, Viewport};
use mandelbrot::pixel::{Pixel, PixelMath};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
const MENU_HEIGHT: i32 = 50;

const ITERATIONS: u32 = 1000;

#[derive(Debug, Clone)]
enum Message {
    Redraw,
    Zoom((i32, i32)),
    Mode,
    Save,
    Reset,
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gleam);
    let mut window = Window::default()
        .with_size(WIDTH, HEIGHT + MENU_HEIGHT);
    window.set_color(Color::White);

    let mut col = Flex::default_fill()
        .column()
        .with_align(Align::Center);
    let mut main = Flex::default()
        .row();
    col.fixed(&main, HEIGHT);

    let mut mandelbrot_frame = Frame::default()
        .with_size(WIDTH, HEIGHT)
        .with_align(Align::LeftTop);
    // .center_x(&window);
    mandelbrot_frame.set_color(Color::White);
    mandelbrot_frame.set_frame(FrameType::DownFrame);

    main.fixed(&mandelbrot_frame, HEIGHT);
    main.end();

    let mut buttons = Flex::default()
        .row()
        .with_align(Align::Center);
    buttons.set_margins(10, 0, 10, 10);
    col.fixed(&buttons, MENU_HEIGHT);

    let mut reset_btn = Button::default()
        .with_label("Reset");

    buttons.add(&reset_btn);

    let mut save_btn = Button::default()
        .with_label("Save");

    buttons.add(&save_btn);

    let mut julia_btn = Button::default()
        .with_label("Switch to Julia set");

    buttons.add(&julia_btn);

    buttons.end();

    col.end();

    col.center_of(&window);

    window.end();
    window.show();

    let mut mandelbrot = setup_mandelbrot();

    mandelbrot.run(ITERATIONS);

    let data = mandelbrot.get_pixels();

    let offs = Offscreen::new(mandelbrot_frame.width(), mandelbrot_frame.height()).unwrap();
    offs.begin();
    draw_rect_fill(0, 0, WIDTH, HEIGHT, Color::White);
    draw_mandelbrot(data);
    offs.end();
    let offs = Rc::from(RefCell::from(offs));

    mandelbrot_frame.draw({
        let offs = offs.clone();
        move |_| {
            let mut offs = offs.borrow_mut();
            if offs.is_valid() {
                offs.rescale();
                offs.copy(0, 0, WIDTH, HEIGHT, 0, 0);
            } else {
                offs.begin();
                draw_rect_fill(0, 0, WIDTH, HEIGHT, Color::White);
                offs.copy(0, 0, WIDTH, HEIGHT, 0, 0);
                offs.end();
            }
            // set_draw_color(Color::Black.inactive());
            // draw_pie(f.w() / 2, f.h() / 2, 100, 100, 0., 360.);
        }
    });

    let mandelbrot = Rc::new(RefCell::from(mandelbrot));

    let julia_set = Rc::new(RefCell::new(false));

    let zoom = Rc::new(RefCell::new(1));

    let (sender, receiver) = app::channel::<Message>();

    mandelbrot_frame.handle({
        // let zoom = zoom.clone();
        // let mandelbrot = mandelbrot.clone();
        // let offs = offs.clone();
        let sender = sender.clone();
        // let mut x = 0;
        // let mut y = 0;
        move |_, event| {
            // let offs = offs.borrow_mut();
            match event {
                Event::Push => {
                    // offs.begin();
                    // set_draw_color(Color::Red);
                    // set_line_style(LineStyle::Solid, 3);
                    let coords = app::event_coords();
                    sender.send(Message::Zoom(coords));
                    // x = coords.0;
                    // y = coords.1;
                    // let x1 = coords.0 - 200;
                    // let y1 = coords.1 - 200;
                    // let x2 = coords.0 + 200;
                    // let y2 = coords.1 + 200;
                    //
                    // let tl = mandelbrot.borrow().get_xy_complex(
                    //     cmp::max(0, x1) as usize,
                    //     cmp::max(0, y1) as usize,
                    // );
                    // let br = mandelbrot.borrow().get_xy_complex(
                    //     cmp::min(WIDTH - 1, x2) as usize,
                    //     cmp::min(HEIGHT - 1, y2) as usize,
                    // );
                    //
                    // if let (Some(tl), Some(br)) = (tl, br) {
                    //     offs.begin();
                    //     draw_rect_fill(0, 0, WIDTH, HEIGHT, Color::White);
                    //     let viewport = Viewport::<f64> {
                    //         top_left: tl,
                    //         bottom_right: br,
                    //         width: WIDTH as f64,
                    //         height: HEIGHT as f64,
                    //     };
                    //     mandelbrot.borrow_mut().update(viewport);
                    //     mandelbrot.borrow_mut().run(ITERATIONS * *zoom.borrow());
                    //     let data = mandelbrot.borrow().get_pixels().clone();
                    //     draw_mandelbrot(&data);
                    //     // set_draw_color(Color::Black);
                    //     // set_line_style(LineStyle::Solid, 3);
                    //     // let (p1, p2, p3, p4) = get_rect_coords(x, y, x2, y2);
                    //     // draw_loop3(p1, p2, p3, p4);
                    //     offs.end();
                    //     f.redraw();
                    //     zoom.replace_with(|&mut old| old + 1);
                    //     return true;
                    // }

                    // draw_point(x, y);
                    // offs.end();
                    // f.redraw();
                    // set_line_style(LineStyle::Solid, 0);
                    true
                }
                // Event::Drag => {
                //     offs.begin();
                //     set_draw_color(Color::Red);
                //     set_line_style(LineStyle::Solid, 3);
                //     let coords = app::event_coords();
                //     draw_line(x, y, coords.0, coords.1);
                //     x = coords.0;
                //     y = coords.1;
                //     offs.end();
                //     f.redraw();
                //     set_line_style(LineStyle::Solid, 0);
                //     true
                // }
                // Event::Released => {
                //     let coords = app::event_coords();
                //     let x2 = coords.0;
                //     let y2 = coords.1;
                //     let tl = mandelbrot.get_xy_complex(x as usize, y as usize);
                //     let br = mandelbrot.get_xy_complex(x2 as usize, y2 as usize);
                //     if tl.is_none() || br.is_none() {
                //         false
                //     } else {
                //         offs.begin();
                //         draw_rect_fill(0, 0, WIDTH, HEIGHT, Color::White);
                //         let viewport = Viewport::<f64> {
                //             top_left: tl.unwrap(),
                //             bottom_right: br.unwrap(),
                //             width: WIDTH as f64,
                //             height: HEIGHT as f64,
                //         };
                //         mandelbrot.update(viewport);
                //         mandelbrot.run(ITERATIONS);
                //         let data = mandelbrot.get_pixels();
                //         for y in 0..HEIGHT as usize {
                //             for x in 0..WIDTH as usize {
                //                 let pixel = data[y][x].clone();
                //                 set_draw_color(Color::from_rgba_tuple(pixel.get_tuple()));
                //                 draw_point(x as i32, y as i32);
                //             }
                //         }
                //         // set_draw_color(Color::Black);
                //         // set_line_style(LineStyle::Solid, 3);
                //         // let (p1, p2, p3, p4) = get_rect_coords(x, y, x2, y2);
                //         // draw_loop3(p1, p2, p3, p4);
                //         offs.end();
                //         f.redraw();
                //
                //         true
                //     }
                // }
                // Event::Move => {
                //     let coords = app::event_coords();
                //     dbg!(&coords);
                // offs.begin();
                // set_draw_color(Color::DarkBlue);
                // draw_pie(coords.0, coords.1, 100, 100, 0., 360.);
                // offs.end();
                // f.redraw();
                // false
                // }
                _event => {
                    // dbg!(&event);
                    false
                }
            }
        }
    });

    // reset_btn.set_callback({
    //     let mandelbrot = mandelbrot.clone();
    //     let zoom = zoom.clone();
    //     move |_| {
    //         mandelbrot.borrow_mut().update(default_viewport());
    //         mandelbrot.borrow_mut().run(ITERATIONS);
    //         let data = mandelbrot.borrow().get_pixels().clone();
    //         let offs = offs.borrow_mut();
    //         offs.begin();
    //         draw_mandelbrot(&data);
    //         offs.end();
    //         zoom.replace(1);
    //     }
    // });

    // save_btn.set_callback({
    //     let mandelbrot = mandelbrot.clone();
    //     move |_| {
    //         let a = mandelbrot.borrow().get_xy_complex(0, 0).unwrap();
    //         let b = mandelbrot.borrow().get_xy_complex((WIDTH - 1) as usize, (HEIGHT - 1) as usize)
    //             .unwrap();
    //         let path = format!("mandelbrot_{:.3}x{:.3}-{:.3}x{:.3}.png", a.r, a.i, b.r, b.i);
    //         let file = File::create(path).expect("Failed to create file");
    //         let buf = BufWriter::new(file);
    //         let mut encoder = png::Encoder::new(buf, WIDTH as u32, HEIGHT as u32);
    //         encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    //         let mut writer = encoder.write_header().unwrap();
    //         let data = mandelbrot.borrow().get_pixels().clone();
    //         writer
    //             .write_image_data(flatten_array(&data).as_slice())
    //             .unwrap();
    //     }
    // });

    reset_btn.emit(sender.clone(), Message::Reset);
    save_btn.emit(sender.clone(), Message::Save);
    julia_btn.emit(sender.clone(), Message::Mode);

    // julia_btn.set_callback({
    //     let mandelbrot = mandelbrot.clone();
    //     let zoom = zoom.clone();
    //     move |b| {
    //         if julia_set.replace_with(|&mut val| !val) {
    //             mandelbrot.borrow_mut().julia_set(ITERATIONS);
    //             b.set_label("Switch to mandelbrot set");
    //         } else {
    //             mandelbrot.borrow_mut().run(ITERATIONS * *zoom.borrow());
    //             b.set_label("Switch to Julia set");
    //         }
    //     }
    // });

    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                Message::Redraw => {
                    mandelbrot_frame.redraw();
                }
                Message::Zoom((x, y)) => {
                    let x1 = x - 200;
                    let y1 = y - 200;
                    let x2 = x + 200;
                    let y2 = y + 200;

                    let tl = mandelbrot.borrow().get_xy_complex(
                        cmp::max(0, x1) as usize,
                        cmp::max(0, y1) as usize,
                    );
                    let br = mandelbrot.borrow().get_xy_complex(
                        cmp::min(WIDTH - 1, x2) as usize,
                        cmp::min(HEIGHT - 1, y2) as usize,
                    );

                    if let (Some(tl), Some(br)) = (tl, br) {
                        offs.borrow_mut().begin();
                        draw_rect_fill(0, 0, WIDTH, HEIGHT, Color::White);
                        let viewport = Viewport::<f64> {
                            top_left: tl,
                            bottom_right: br,
                            width: WIDTH as f64,
                            height: HEIGHT as f64,
                        };
                        mandelbrot.borrow_mut().update(viewport);
                        if *julia_set.borrow() {
                            mandelbrot.borrow_mut().julia_set(ITERATIONS * *zoom.borrow());
                        } else {
                            mandelbrot.borrow_mut().run(ITERATIONS * *zoom.borrow());
                        }
                        let data = mandelbrot.borrow().get_pixels().clone();
                        draw_mandelbrot(&data);
                        // set_draw_color(Color::Black);
                        // set_line_style(LineStyle::Solid, 3);
                        // let (p1, p2, p3, p4) = get_rect_coords(x, y, x2, y2);
                        // draw_loop3(p1, p2, p3, p4);
                        offs.borrow_mut().end();
                        zoom.replace_with(|&mut old| old + 1);
                        sender.send(Message::Redraw);
                    }
                }
                Message::Mode => {
                    if julia_set.replace_with(|&mut val| !val) {
                        mandelbrot.borrow_mut().run(ITERATIONS * *zoom.borrow());
                        julia_btn.set_label("Switch to julia set");
                    } else {
                        mandelbrot.borrow_mut().julia_set(ITERATIONS * *zoom.borrow());
                        julia_btn.set_label("Switch to mandelbrot set");
                    }
                    offs.borrow_mut().begin();
                    draw_rect_fill(0, 0, WIDTH, HEIGHT, Color::White);
                    let data = mandelbrot.borrow().get_pixels().clone();
                    draw_mandelbrot(&data);
                    offs.borrow_mut().end();
                    sender.send(Message::Redraw);
                }
                Message::Save => {
                    let a = mandelbrot.borrow().get_xy_complex(0, 0).unwrap();
                    let b = mandelbrot.borrow().get_xy_complex((WIDTH - 1) as usize, (HEIGHT - 1) as usize)
                        .unwrap();
                    let path = format!("mandelbrot_{:.3}x{:.3}-{:.3}x{:.3}.png", a.r, a.i, b.r, b.i);
                    let file = File::create(path).expect("Failed to create file");
                    let buf = BufWriter::new(file);
                    let mut encoder = png::Encoder::new(buf, WIDTH as u32, HEIGHT as u32);
                    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
                    let mut writer = encoder.write_header().unwrap();
                    let data = mandelbrot.borrow().get_pixels().clone();
                    writer
                        .write_image_data(flatten_array(&data).as_slice())
                        .unwrap();
                }
                Message::Reset => {
                    mandelbrot.borrow_mut().update(default_viewport());
                    if *julia_set.borrow() {
                        mandelbrot.borrow_mut().julia_set(ITERATIONS);
                    } else {
                        mandelbrot.borrow_mut().run(ITERATIONS);
                    }
                    let data = mandelbrot.borrow().get_pixels().clone();
                    let offs = offs.borrow_mut();
                    offs.begin();
                    draw_mandelbrot(&data);
                    offs.end();
                    zoom.replace(1);
                    sender.send(Message::Redraw);
                }
            }
        }
    }
}

fn draw_mandelbrot(data: &[Vec<Pixel<u8>>]) {
    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            let pixel = data[y][x].clone();
            set_draw_color(Color::from_rgba_tuple(pixel.get_tuple()));
            draw_point(x as i32, y as i32);
        }
    }
}

fn setup_mandelbrot() -> Mandelbrot<u8> {
    let viewport = default_viewport();

    let config = MandelbrotConfig::<u8> {
        dimensions: (WIDTH as u32, HEIGHT as u32),
        viewport,
        // color_fn: ContinuousColorScale::get_color_fn(20.0, 0.8, 1.0),
        color_fn: ContinuousColorScale::get_color_fn_boxed(200.0, 1.0, 1.0),
        // color_fn: Box::new(SimpleColorScale::pixel_color),
    };

    Mandelbrot::new(config)
}

fn default_viewport() -> Viewport<f64> {
    let a_x = -2.;
    // let z_x_b = a_x - -0.015;

    // let z_y = 0.5;

    let a = ComplexNumber::new(a_x, 1.15);
    let b = ComplexNumber::new(0.5, -1.15);

    // let z_a = ComplexNumber::new(a_x, z_y);
    // let z_b = ComplexNumber::new(z_x_b, -z_y);

    Viewport::<f64> {
        top_left: a,
        bottom_right: b,
        width: WIDTH as f64,
        height: HEIGHT as f64,
    }
}

fn get_rect_coords(x1: i32, y1: i32, x2: i32, y2: i32) -> (Coord<i32>, Coord<i32>, Coord<i32>,
                                                           Coord<i32>) {
    let pos1 = Coord(x1, y1);
    let pos2 = Coord(x2, y1);
    let pos3 = Coord(x2, y2);
    let pos4 = Coord(x1, y2);

    (pos1, pos2, pos3, pos4)
}
