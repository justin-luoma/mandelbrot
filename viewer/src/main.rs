use std::cell::RefCell;
use std::cmp;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;

use fltk::{app, input};
use fltk::app::{App, Scheme, Sender};
use fltk::button::Button;
use fltk::draw::*;
use fltk::draw::Offscreen;
use fltk::enums::{Align, Color, Event, FrameType};
use fltk::frame::Frame;
use fltk::group::Flex;
use fltk::prelude::*;
use fltk::window::{DoubleWindow, Window};
use png::HasParameters;

use mandelbrot::color_scale::ContinuousColorScale;
use mandelbrot::complex_number::ComplexNumber;
use mandelbrot::config::MandelbrotConfig;
use mandelbrot::config::viewport::Viewport;
use mandelbrot::flatten_array;
use mandelbrot::mandelbrot::Mandelbrot;
use mandelbrot::pixel::{Pixel, PixelMath};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
const MENU_HEIGHT: i32 = 50;
const TOOLS_WINDOW_DIMENSIONS: (i32, i32) = (300, 300);

const ITERATIONS: u32 = 1000;

#[derive(Debug, Clone)]
enum AnimationState {
    Running,
    Stopped,
    Selecting,
}

#[derive(Debug, Clone)]
struct Settings {
    a: (f64, f64),
    b: (f64, f64),
}

#[derive(Debug, Clone)]
enum Message {
    Redraw,
    Zoom((i32, i32)),
    Mode,
    Save,
    Reset,
    Animation(AnimationState),
    Center((i32, i32)),
    Frame((ComplexNumber<f64>, ComplexNumber<f64>)),
    Loading(bool),
    SettingsWindow,
    SettingsUpdate(Settings),
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gleam);
    let mut window = Window::default()
        .with_size(WIDTH, HEIGHT + MENU_HEIGHT);
    window.set_color(Color::White);

    let mut col_main = Flex::default_fill()
        .column()
        .with_align(Align::Center);
    let mut row_main = Flex::default()
        .row();
    col_main.fixed(&row_main, HEIGHT);

    let mut mandelbrot_frame = Frame::default()
        .with_size(WIDTH, HEIGHT)
        .with_align(Align::LeftTop);
    mandelbrot_frame.set_color(Color::White);
    mandelbrot_frame.set_frame(FrameType::DownFrame);

    row_main.fixed(&mandelbrot_frame, HEIGHT);
    row_main.end();

    let mut buttons = Flex::default()
        .row()
        .with_align(Align::Center);
    buttons.set_margins(10, 0, 10, 10);
    col_main.fixed(&buttons, MENU_HEIGHT);

    let mut reset_btn = Button::default()
        .with_label("Reset");
    let mut save_btn = Button::default()
        .with_label("Save");
    let mut julia_btn = Button::default()
        .with_label("Switch to Julia set");
    let mut animation_btn = Button::default()
        .with_label("Play Animation");
    let mut settings_btn = Button::default()
        .with_label("Settings");
    buttons.end();

    col_main.end();

    col_main.center_of(&window);

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
        }
    });

    let mandelbrot = Rc::new(RefCell::from(mandelbrot));

    let julia_set = Rc::new(RefCell::new(false));

    let loading = Rc::new(RefCell::new(false));

    let zoom = Rc::new(RefCell::new(1));

    let selecting_center = Rc::new(RefCell::new(false));

    let (sender, receiver) = app::channel::<Message>();

    mandelbrot_frame.handle({
        let sender = sender.clone();
        let selecting_center = selecting_center.clone();
        let loading = loading.clone();
        move |_, event| {
            if !*loading.borrow() {
                match event {
                    Event::Push => {
                        let coords = app::event_coords();
                        if *selecting_center.borrow() {
                            sender.send(Message::Center(coords));
                            sender.send(Message::Animation(AnimationState::Running));
                        } else {
                            sender.send(Message::Loading(true));
                            sender.send(Message::Zoom(coords));
                        }
                        true
                    }
                    _ => {
                        false
                    }
                }
            } else {
                false
            }
        }
    });

    let mut animation_running = false;

    reset_btn.emit(sender.clone(), Message::Reset);
    save_btn.emit(sender.clone(), Message::Save);
    julia_btn.emit(sender.clone(), Message::Mode);
    animation_btn.emit(sender.clone(), Message::Animation(AnimationState::Selecting));
    settings_btn.emit(sender.clone(), Message::SettingsWindow);

    let mandelbrot = mandelbrot.clone();

    let mut settings_window = settings_window(sender.clone());

    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                Message::Redraw => {
                    mandelbrot_frame.redraw();
                    sender.send(Message::Loading(false));
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
                        offs.borrow_mut().end();
                        zoom.replace_with(|&mut old| old + 1);
                        sender.send(Message::Redraw);
                    }
                }
                Message::Mode => {
                    sender.send(Message::Loading(true));
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
                    sender.send(Message::Loading(true));
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
                    sender.send(Message::Loading(false));
                }
                Message::Reset => {
                    sender.send(Message::Loading(true));
                    if animation_running {
                        sender.send(Message::Animation(AnimationState::Stopped))
                    }
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
                Message::Animation(state) => {
                    match state {
                        AnimationState::Running => {
                            animation_running = true;
                            animation_btn.set_label("Pause Animation");
                            animation_btn.emit(sender.clone(), Message::Animation(AnimationState::Stopped));
                        }
                        AnimationState::Stopped => {
                            animation_running = false;
                            animation_btn.set_label("Play Animation");
                            animation_btn.emit(
                                sender.clone(),
                                Message::Animation(AnimationState::Selecting),
                            );
                        }
                        AnimationState::Selecting => {
                            selecting_center.replace(true);
                            animation_btn.set_label("Select Center");
                            animation_btn.emit(sender.clone(), Message::Animation(AnimationState::Stopped));
                        }
                    }
                }
                Message::Center((x, y)) => {
                    selecting_center.replace(false);
                    let center = mandelbrot.borrow().get_xy_complex(x as usize, y as usize).unwrap();
                    let viewport = default_viewport();
                    let width = viewport.bottom_right.r - viewport.top_left.r;
                    let height = viewport.top_left.i - viewport.bottom_right.i;
                    let x1 = center.r - width / 2.;
                    let y1 = center.i + height / 2.;
                    let x2 = center.r + width / 2.;
                    let y2 = center.i - height / 2.;
                    let a = ComplexNumber::new(x1, y1);
                    let b = ComplexNumber::new(x2, y2);
                    sender.send(Message::Frame((a, b)));
                }
                Message::Frame((a, b)) => {
                    if animation_running {
                        sender.send(Message::Loading(true));
                        let mut viewport = default_viewport();
                        viewport.top_left = a;
                        viewport.bottom_right = b;
                        mandelbrot.borrow_mut().update(viewport);
                        if *julia_set.borrow() {
                            mandelbrot.borrow_mut().julia_set(ITERATIONS);
                        } else {
                            mandelbrot.borrow_mut().run(ITERATIONS);
                        }
                        if animation_running {
                            let data = mandelbrot.borrow().get_pixels().clone();
                            let offs = offs.borrow_mut();
                            offs.begin();
                            draw_mandelbrot(&data);
                            offs.end();
                            sender.send(Message::Redraw);
                        }
                        let a = mandelbrot.borrow().get_xy_complex(75, 75).unwrap();
                        let b = mandelbrot.borrow()
                            .get_xy_complex((WIDTH - 75) as usize, (HEIGHT - 75) as usize)
                            .unwrap();
                        if animation_running {
                            sender.send(Message::Frame((a, b)));
                        }
                    }
                }
                Message::Loading(state) => {
                    loading.replace(state);
                    if state {
                        reset_btn.deactivate();
                        save_btn.deactivate();
                        julia_btn.deactivate();
                        if !animation_running {
                            animation_btn.deactivate();
                        }
                    } else {
                        reset_btn.activate();
                        save_btn.activate();
                        julia_btn.activate();
                        animation_btn.activate();
                    }
                }
                Message::SettingsWindow => {
                    settings_window.show();
                }
                Message::SettingsUpdate(settings) => {

                }
            }
        }
    }
}

fn settings_window(sender: Sender<Message>) -> DoubleWindow {
    let mut settings_window = Window::default()
        .with_label("Settings")
        .with_size(TOOLS_WINDOW_DIMENSIONS.0, TOOLS_WINDOW_DIMENSIONS.1);

    let mut col = Flex::default_fill().column().center_of_parent();

    let mut menu_row = Flex::default()
        .row()
        .with_align(Align::Center);
    menu_row.set_margins(10, 0, 10, 0);

    let coords = Flex::default()
        .column()
        .with_align(Align::Left);

    let col_top_left = Flex::default_fill().column();
    let row = Flex::default().row();
    Frame::default().with_label("Top Left");
    row.end();
    let row = Flex::default_fill().row();
    Frame::default().with_label("x").with_align(Align::Inside | Align::Left);
    let mut tl_x_input = input::FloatInput::default();
    Frame::default().with_label("y").with_align(Align::Inside | Align::Left);
    let mut tl_y_input = input::FloatInput::default();
    row.end();
    col_top_left.end();

    let col_bottom_right = Flex::default_fill().column();
    let row = Flex::default().row();
    Frame::default().with_label("Bottom Right");
    row.end();
    let row = Flex::default_fill().row();
    Frame::default().with_label("x").with_align(Align::Inside | Align::Left);
    let mut br_x_input = input::FloatInput::default();
    Frame::default().with_label("y").with_align(Align::Inside | Align::Left);
    let mut br_y_input = input::FloatInput::default();
    row.end();
    col_bottom_right.end();

    coords.end();
    menu_row.end();

    let button_row = Flex::default().row();
    let mut update_btn = Button::default().with_label("Update");
    button_row.end();
    col.fixed(&button_row, MENU_HEIGHT);

    col.end();

    settings_window.end();

    let viewport = default_viewport();
    let a = viewport.top_left;
    let b = viewport.bottom_right;

    tl_x_input.set_value(&format!("{}", a.r));
    tl_y_input.set_value(&format!("{}", a.i));
    br_x_input.set_value(&format!("{}", b.r));
    br_y_input.set_value(&format!("{}", b.i));

    update_btn.set_callback(move |_| {
        let x1 = tl_x_input.value().parse::<f64>();
        let y1 = tl_y_input.value().parse::<f64>();
        let x2 = br_x_input.value().parse::<f64>();
        let y2 = br_y_input.value().parse::<f64>();
        if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (x1, y1, x2, y2) {
            sender.send(Message::SettingsUpdate(Settings {
                a: (x1, y1),
                b: (x2, y2),
            }));
        }
    });

    settings_window
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
        color_fn: ContinuousColorScale::get_color_fn_boxed(200.0, 1.0, 1.0),
    };

    Mandelbrot::new(config)
}

fn default_viewport() -> Viewport<f64> {
    Viewport::default().with_size(WIDTH as f64, HEIGHT as f64)
}

fn get_rect_coords(x1: i32, y1: i32, x2: i32, y2: i32) -> (Coord<i32>, Coord<i32>, Coord<i32>,
                                                           Coord<i32>) {
    let pos1 = Coord(x1, y1);
    let pos2 = Coord(x2, y1);
    let pos3 = Coord(x2, y2);
    let pos4 = Coord(x1, y2);

    (pos1, pos2, pos3, pos4)
}
