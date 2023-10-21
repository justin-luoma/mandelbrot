use std::thread;
use std::time::Duration;

use bevy::{prelude::{*}, window::PrimaryWindow};
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::mouse::MouseButtonInput;
use bevy::window::WindowResolution;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};
use bevy_egui::egui::{PointerButton, pos2, Pos2, Response, Sense, Ui};
use bevy_egui::egui::emath::Numeric;
use bevy_egui::egui::load::SizedTexture;
use futures_lite::future;
use image::RgbaImage;
use num_traits::Num;
use rayon::prelude::*;

use gui::{ComputeTask, ImageRes, Images, LoadingEvent, TriangleGenerator};
use gui::message::{GeneratorRxSender, GeneratorUiReceiver};
use mandelbrot::flatten_array;
use mandelbrot::mandelbrot::{Mandelbrot, MandelbrotConfig, Viewport};
use mandelbrot::pixel::Pixel;
use fractal_generator_gui::{GeneratorRxMessage, GeneratorSettings, GeneratorTxMessage, RgbaData};
use sierpinski_triangle::SierpinskiTriangle;


const WIDTH: f32 = 1000.;

const SIDEBAR_WIDTH: f32 = 200.;
const BOTTOM_PANEL_HEIGHT: f32 = 25.;

const HEIGHT: f32 = 1000.;

const ITERATIONS: u32 = 1000;

const TOTAL_WIDTH: f32 = WIDTH + SIDEBAR_WIDTH;

const TOTAL_HEIGHT: f32 = HEIGHT + BOTTOM_PANEL_HEIGHT;

const OFFSET_RE: f32 = -(TOTAL_WIDTH / 2.);

const OFFSET_IM: f32 = -(TOTAL_HEIGHT / 2.);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa::Sample4)
        .init_resource::<UiState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::from((TOTAL_WIDTH, TOTAL_HEIGHT))
                        .with_scale_factor_override(1.),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            EguiPlugin
        ))
        .add_event::<LoadingEvent>()
        .init_resource::<Images>()
        // .init_resource::<EntityGrid>()
        .add_systems(
            Startup,
            (
                configure_visuals_system,
                configure_ui_state_system,
                setup_mandelbrot,
                setup_camera_system,
            ),
        )
        .insert_resource(TriangleGenerator(SierpinskiTriangle::new(
            400.,
            100,
            ([255, 255, 255, 255], [0, 0, 0, 255]),
        )))
        .add_systems(
            PostStartup,
            (generator_run_system),
        )
        // .add_systems(PostStartup, mandelbrot_system)
        .add_systems(
            Update,
            (
                update_ui_scale_factor_system,
                ui_example_system,
                generator_image_system,
                draw,
            ),
        )
        .add_systems(
            Update,
            (cursor_events)
                .run_if(input_just_pressed(MouseButton::Left)),
        )
        .run();
}

fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gizmos: Gizmos,
) {
    // let iterations = 3;
    // let vertices = koch_snowflake(iterations);
    // for v in vertices.windows(2) {
    //     let v1 = v[0];
    //     let v2 = v[1];
    //     // gizmos.line_2d(
    //     //     Vec2::new(
    //     //         v1.0 as f32 * 500.,
    //     //         v1.1 as f32 * 500.,
    //     //     ), Vec2::new(
    //     //         v2.0 as f32 * 500.,
    //     //         v2.1 as f32 * 500.,
    //     //     ),
    //     //     Color::WHITE);
    // }
    // let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    // let points = vec![[0., 0., 1., 1.]];
    // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    // gizmos.line_2d(Vec2::new(0., 0.), Vec2::new(10., 10.), Color::WHITE);
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from()).into(),
    //     ..default()
    // });
}

fn koch_snowflake(iterations: u32) -> Vec<(f64, f64)> {
    {
        let mut current = vec![
            (0., 1.),
            (3f64.sqrt() / 2., -0.5),
            (-(3f64).sqrt() / 2., -0.5),
        ];
        for _ in 0..iterations {
            current = snowflake_iter(&current[..]);
        }
        let first = current[0];
        current.push(first);
        current
    }
}

fn snowflake_iter(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut r = vec![];
    for i in 0..points.len() {
        let (start, end) = (points[i], points[(i + 1) % points.len()]);
        let t = ((end.0 - start.0) / 3.0, (end.1 - start.1) / 3.0);
        let s = (
            t.0 * 0.5 - t.1 * (0.75f64).sqrt(),
            t.1 * 0.5 + (0.75f64).sqrt() * t.0,
        );
        r.push(start);
        r.push((start.0 + t.0, start.1 + t.1));
        r.push((start.0 + t.0 + s.0, start.1 + t.1 + s.1));
        r.push((start.0 + t.0 * 2., start.1 + t.1 * 2.));
    }
    r
}

fn cursor_events(
    mut event: EventReader<MouseButtonInput>,
    window: Query<&Window, With<PrimaryWindow>>,
    tx: Res<GeneratorRxSender>,
) {
    for _ in event.iter() {
        if let Some(pos) = window.single().cursor_position() {
            // dbg!(&pos);
            if pos.x < WIDTH && pos.y < HEIGHT {
                // tx.0.send(GeneratorMessage::Zoom((pos.x as u32, pos.y as u32))).unwrap();
            }
        }
    }
}

fn setup_camera_system(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         custom_size: Some(Vec2::splat(10.)),
    //         color: Color::WHITE,
    //         ..Default::default()
    //     },
    //     texture: render::texture::DEFAULT_IMAGE_HANDLE.typed(),
    //     transform: Transform::from_xyz(offset_x(500.), offset_y(500.), 100.),
    //     ..Default::default()
    // });
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::RegularPolygon::new(50., 3).into())
    //         .into(),
    //     material: materials.add(ColorMaterial::from(Color::WHITE)),
    //     transform: Transform::from_xyz(0., 0., 1.).with_rotation
    //     (Quat::from_rotation_x(60.)),
    //     ..Default::default()
    // });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::RegularPolygon::new(2., 3).into()),
    //     material: materials.add(Color::WHITE.into()),
    //     transform: Transform::from_xyz(0., 0., 0.),
    //     ..Default::default()
    // });
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 9000.,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4., 8., 4.),
    //     ..default()
    // });
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0., 0., 10.).looking_at(Vec3::ZERO, Vec3::Z),
    //     ..Default::default()
    // });
}

fn setup_mandelbrot(
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
) {
    let viewport = Viewport::default()
        .with_size(WIDTH as f64, HEIGHT as f64);

    let mandelbrot =
        Mandelbrot::<u8, f64>::new(
            MandelbrotConfig::default()
                .with_dimensions((WIDTH as u32, HEIGHT as u32))
                .with_viewport(viewport)
        );

    ui_state.viewport = viewport;

    let (generator_tx, ui_rx) = crossbeam_channel::unbounded();
    let (ui_tx, generator_rx) = crossbeam_channel::unbounded();
    thread::spawn(move || {
        let mut mandelbrot = mandelbrot;
        mandelbrot.run(ITERATIONS);
        let pixels = mandelbrot.get_pixels();

        generator_tx.send(GeneratorTxMessage::Image(pixels)).unwrap();
        for msg in generator_rx {
            match msg {
                GeneratorRxMessage::Zoom((x, y)) => {
                    mandelbrot.zoom((x, y), 200);
                    generator_tx.send(GeneratorTxMessage::Loading(true)).unwrap();
                    mandelbrot.run(ITERATIONS);
                    let pixels = mandelbrot.get_pixels();
                    generator_tx.send(GeneratorTxMessage::Image(pixels)).unwrap();
                    generator_tx.send(GeneratorTxMessage::Loading(false)).unwrap();
                }
                GeneratorRxMessage::Settings(settings) => {
                    mandelbrot.update_settings(&settings);
                    if settings.exponent.is_some() {
                        generator_tx.send(GeneratorTxMessage::Loading(true)).unwrap();
                        mandelbrot.recalculate(false);
                    }
                    if settings.x1.is_some() || settings.y1.is_some() || settings.x2.is_some() ||
                        settings.y2.is_some() {
                        mandelbrot.recalculate(false);
                    }
                    mandelbrot.redraw();
                    let pixels = mandelbrot.get_pixels();
                    generator_tx.send(GeneratorTxMessage::Image(pixels)).unwrap();
                    generator_tx.send(GeneratorTxMessage::Loading(false)).unwrap();
                }
                GeneratorRxMessage::Reset => {
                    mandelbrot.reset();
                    mandelbrot.update_config(MandelbrotConfig::default()
                        .with_dimensions((WIDTH as u32, HEIGHT as u32))
                        .with_viewport(viewport));
                    generator_tx.send(GeneratorTxMessage::Loading(true)).unwrap();
                    mandelbrot.run(ITERATIONS);
                    let pixels = mandelbrot.get_pixels();
                    generator_tx.send(GeneratorTxMessage::Image(pixels)).unwrap();
                    generator_tx.send(GeneratorTxMessage::Loading(false)).unwrap();
                }
            }
        }
    });

    commands.insert_resource(GeneratorUiReceiver(ui_rx));
    commands.insert_resource(GeneratorRxSender(ui_tx));
}

fn generator_image_system(
    mut commands: Commands,
    rx: Res<GeneratorUiReceiver<Pixel<u8>>>,
    mut ui_state: ResMut<UiState>,
    mut asset: ResMut<Assets<Image>>,
    // mut is_initialized: Local<bool>,
) {
    for msg in rx.0.try_iter() {
        match msg {
            GeneratorTxMessage::Loading(is_loading) => {
                ui_state.loading = is_loading;
            }
            GeneratorTxMessage::Image(rgba_grid) => {
                let image: Vec<_> = rgba_grid
                    .into_iter()
                    .flat_map(|row| row
                        .into_iter()
                        .flat_map(|pixel| pixel.data().map(|v| v as u8)))
                    .collect();

                // for (y, row) in rgba_grid.into_iter().enumerate() {
                //     for (x, pixel) in row.into_iter().enumerate() {
                //         let x = offset_x(x as f32);
                //         let y = offset_y(y as f32);
                //         let entity = commands.spawn(SpriteBundle {
                //             sprite: Sprite {
                //                 custom_size: Some(Vec2::splat(0.5)),
                //                 color: Color::from(pixel.data()),
                //                 ..default()
                //             },
                //             transform: Transform::from_xyz(
                //                 x,
                //                 y
                //                 , 1.
                //             ),
                //             ..default()
                //         })
                //             .id();
                //         entity_grid.0.insert((x as i32, y as i32), entity);
                //     }
                // }
                let image = RgbaImage::from_raw(WIDTH as u32, HEIGHT as u32, image).unwrap();
                let image = image::DynamicImage::from(image);
                let handle = asset.add(Image::from_dynamic(image, false));
                commands.insert_resource(ImageRes(handle));
            }
        }
    }
}

fn generator_run_system(
    tx: Res<GeneratorRxSender>,
) {
    let tx = tx.0.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000));
            // let viewport = Viewport::default();
            // tx.send(viewport).unwrap();
        }
    });
}

fn handle_tasks(
    mut commands: Commands,
    mut tasks: Query<&mut ComputeTask>,
    mut assets: ResMut<Assets<Image>>,
    mandelbrot: ResMut<Mandelbrot<u8, f64>>,
) {
    for mut task in &mut tasks {
        if let Some(task) = future::block_on(future::poll_once(&mut task.0)) {
            if task {
                let pixels = mandelbrot.get_pixels();
                let image = RgbaImage::from_raw(WIDTH as u32, HEIGHT as u32, flatten_array(pixels))
                    .expect("image creation");
                let image = image::DynamicImage::from(image);
                let handle = assets.add(Image::from_dynamic(image, false));
                commands.insert_resource(ImageRes(handle));
            }
        }
    }
}

#[derive(Default, Resource)]
struct UiState {
    label: String,
    value: f32,
    painting: Painting,
    inverted: bool,
    egui_texture_handle: Option<egui::TextureHandle>,
    is_window_open: bool,
    viewport: Viewport<f64>,
    settings: GeneratorSettings,
    loading: bool,
}

fn configure_visuals_system(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn configure_ui_state_system(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}

fn update_ui_scale_factor_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut toggle_scale_factor: Local<Option<bool>>,
    mut egui_settings: ResMut<EguiSettings>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
        *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        if let Ok(window) = windows.get_single() {
            let scale_factor = if toggle_scale_factor.unwrap() {
                1.0
            } else {
                1.0 / window.scale_factor()
            };
            egui_settings.scale_factor = scale_factor;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn ui_example_system(
    mut ui_state: ResMut<UiState>,
    mut triangle: ResMut<TriangleGenerator>,
    // You are not required to store Egui texture ids in systems. We store this one here just to
    // demonstrate that rendering by using a texture id of a removed image is handled without
    // making bevy_egui panic.
    mut rendered_texture_id: Local<egui::TextureId>,
    generator_sender: Res<GeneratorRxSender>,
    mut is_initialized: Local<bool>,
    mut updated: Local<bool>,
    mut iterations: Local<u32>,
    // If you need to access the ids from multiple systems, you can also initialize the `Images`
    // resource while building the app and use `Res<Images>` instead.
    // images: Res<Images>,
    image: Option<Res<ImageRes>>,
    mut contexts: EguiContexts,
) {
    // run_example(&mut ui_state, &mut rendered_texture_id, &mut is_initialized, &images, contexts);

    if *iterations == 0 {
        *iterations = 1;
    }

    if let Some(image) = image {
        if !*is_initialized {
            *is_initialized = true;
        }
        *rendered_texture_id = contexts.add_image(image.0.clone_weak());
    }

    let is_loading = ui_state.loading;

    let ctx = contexts.ctx_mut();

    let x1 = ui_state.viewport.top_left.r;
    let y1 = ui_state.viewport.top_left.i;
    let x2 = ui_state.viewport.bottom_right.r;
    let y2 = ui_state.viewport.bottom_right.i;
    let mut x1 = ui_state.settings.x1.clone().unwrap_or(x1.to_string());
    let mut y1 = ui_state.settings.y1.clone().unwrap_or(y1.to_string());
    let mut x2 = ui_state.settings.x2.clone().unwrap_or(x2.to_string());
    let mut y2 = ui_state.settings.y2.clone().unwrap_or(y2.to_string());
    let mut hue = ui_state.settings.hue.unwrap_or(200.);
    let mut exponent = ui_state.settings.exponent.unwrap_or(2);

    let vertices = koch_snowflake(*iterations);

    egui::SidePanel::right("right-panel")
        .exact_width(SIDEBAR_WIDTH)
        .resizable(false)
        .show(ctx, |ui| {
            egui::Grid::new("right-panel-grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Viewport");
                    ui.end_row();

                    if create_viewport_ui(
                        "X1:",
                        &mut x1,
                        ui,
                    ).changed() {
                        let _ = ui_state.settings.x1.insert(x1);
                        *updated = true;
                    }

                    if create_viewport_ui(
                        "Y1:",
                        &mut y1,
                        ui,
                    ).changed() {
                        let _ = ui_state.settings.y1.insert(y1);
                        *updated = true;
                    }

                    if create_viewport_ui(
                        "X2:",
                        &mut x2,
                        ui,
                    ).changed() {
                        let _ = ui_state.settings.x2.insert(x2);
                        *updated = true;
                    }

                    if create_viewport_ui(
                        "Y2:",
                        &mut y2,
                        ui,
                    ).changed() {
                        let _ = ui_state.settings.y2.insert(y2);
                        *updated = true;
                    }

                    ui.add_space(25.);
                    ui.end_row();

                    if add_settings_editor(
                        "Color Hue",
                        (0f64, 255f64),
                        &mut hue,
                        1.,
                        ui,
                    ).changed() {
                        let _ = ui_state.settings.hue.insert(hue);
                        *updated = true;
                    }

                    if add_settings_editor(
                        "Exponent",
                        (2, 10),
                        &mut exponent,
                        1.,
                        ui,
                    ).changed() {
                        let _ = ui_state.settings.exponent.insert(exponent);
                        *updated = true;
                    }

                    ui.add_space(25.);
                    ui.end_row();

                    if ui.add_enabled(
                        *is_initialized && !is_loading && *updated,
                        egui::Button::new("Update"),
                    ).clicked() {
                        *updated = false;
                        generator_sender
                            .send(GeneratorRxMessage::Settings(ui_state.settings.clone()))
                            .unwrap();
                    }

                    ui.end_row();

                    if ui
                        .add_enabled(*is_initialized && !is_loading, egui::Button::new("Reset"))
                        .clicked() {
                        generator_sender
                            .send(GeneratorRxMessage::Reset)
                            .unwrap();
                    }

                    ui.end_row();

                    if ui
                        .add(egui::Button::new("Iterate"))
                        .clicked() {
                        // triangle.0.iterate();
                        *iterations += 1;
                    }
                });
        });

    let image_area = egui::Area::new("main")
        .fixed_pos(pos2(0., 0.))
        .enabled(*is_initialized && !is_loading)
        .show(ctx, |ui| {
            if *is_initialized {
                ui.add(egui::widgets::Image::from_texture(SizedTexture::new(
                    *rendered_texture_id,
                    [1000., 1000.],
                )));
            } else {
                ui.horizontal_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(WIDTH / 2.);
                        ui.label("Loading...");
                    });
                });
            }

            let lines: Vec<_> = vertices
                .par_windows(2)
                .map(|v| (v[0], v[1]))
                .map(|(p1, p2)| {
                    egui::Shape::line_segment(
                        [
                            Pos2::new(p1.0 as f32 * 500. + WIDTH / 2., p1.1 as f32 * 500. +
                                HEIGHT /
                                2.),
                            Pos2::new(p2.0 as f32 * 500. + WIDTH / 2., p2.1 as f32 * 500. + HEIGHT /
                                2.)
                        ],
                        egui::Stroke::new(
                            1.,
                            egui::Color32::WHITE,
                        ),
                    )
                }).collect();
            // ui.painter().extend(lines);


            // let mut triangle = SierpinskiTriangle::new(
            //     400.,
            //     100,
            //     ([255, 255, 255, 255], [0, 0, 0, 255]),
            // );
            // for _ in 0..10 {
            //     triangle.iterate();
            // }
            let pixels = triangle.0.pixels();
            let triangles = pixels.iter()
                .map(|(color, (a, b, c))| (
                    color,
                    vec![
                        Pos2::new(a.0 as f32 + WIDTH / 2., a.1 as f32 * -1. + HEIGHT / 2.),
                        Pos2::new(b.0 as f32 + WIDTH / 2., b.1 as f32 * -1. + HEIGHT / 2.),
                        Pos2::new(c.0 as f32 + WIDTH / 2., c.1 as f32 * -1. + HEIGHT / 2.),
                    ])
                )
                .map(|(color, points)| {
                    egui::Shape::convex_polygon(points, egui::Color32::from_rgba_unmultiplied
                        (color[0], color[1], color[2], color[3]),
                                                egui::Stroke::new(1., egui::Color32::WHITE))
                });
            // ui.painter().extend(triangles);
        })
        .response
        .interact(Sense::click());

    if image_area.clicked_by(PointerButton::Primary) {
        if let Some(pos) = image_area.interact_pointer_pos() {
            if pos.x < WIDTH && pos.y < HEIGHT {
                generator_sender
                    .send(GeneratorRxMessage::Zoom((pos.x as u32, pos.y as u32)))
                    .unwrap();
            }
        }
    }

    egui::TopBottomPanel::bottom("bottom-panel")
        .resizable(false)
        .exact_height(BOTTOM_PANEL_HEIGHT)
        .show(ctx, |ui| {
            if !*is_initialized || is_loading {
                ui.add(egui::Spinner::new());
            }
        });

    *updated = false;
}

fn create_viewport_ui(
    label: &str,
    state: &mut String,
    ui: &mut Ui,
) -> Response {
    ui.label(label);
    let widget = ui.add(egui::TextEdit::singleline(state));
    if widget.changed() {
        *state = state
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
            .collect();
    }
    ui.end_row();
    widget
}

fn add_settings_editor<T: Num + Numeric>(
    name: &str,
    range: (T, T),
    value: &mut T,
    speed: f64,
    ui: &mut Ui,
) -> Response
    where f64: From<T> {
    ui.label(name);
    let widget = ui.add(egui::DragValue::new(value)
        .speed(speed)
        .clamp_range((range.0)..=(range.1)));
    ui.end_row();
    widget
}

fn offset_x(x: f32) -> f32 {
    x + OFFSET_RE
}

fn offset_y(y: f32) -> f32 {
    y + OFFSET_IM
}

// fn run_example(mut ui_state: &mut ResMut<UiState>, rendered_texture_id: &mut Local<TextureId>, is_initialized: &mut Local<bool>, images: &Local<Images>, mut contexts: EguiContexts) {
//     let egui_texture_handle = ui_state
//         .egui_texture_handle
//         .get_or_insert_with(|| {
//             contexts.ctx_mut().load_texture(
//                 "example-image",
//                 egui::ColorImage::example(),
//                 Default::default(),
//             )
//         })
//         .clone();
//
//     let mut load = false;
//     let mut remove = false;
//     let mut invert = false;
//
//     if !*is_initialized {
//         *is_initialized = true;
//         *rendered_texture_id = contexts.add_image(images.mandelbrot.clone_weak());
//     }
//
//     let ctx = contexts.ctx_mut();
//
//     egui::SidePanel::left("side_panel")
//         .default_width(200.0)
//         .show(ctx, |ui| {
//             ui.heading("Side Panel");
//
//             ui.horizontal(|ui| {
//                 ui.label("Write something: ");
//                 ui.text_edit_singleline(&mut ui_state.label);
//             });
//
//             ui.add(egui::widgets::Image::from_texture(SizedTexture::new(
//                 egui_texture_handle.id(),
//                 egui_texture_handle.size_vec2(),
//             )));
//
//             ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
//             if ui.button("Increment").clicked() {
//                 ui_state.value += 1.0;
//             }
//
//             ui.allocate_space(egui::Vec2::new(1.0, 100.0));
//             ui.horizontal(|ui| {
//                 load = ui.button("Load").clicked();
//                 invert = ui.button("Invert").clicked();
//                 remove = ui.button("Remove").clicked();
//             });
//
//             ui.add(egui::widgets::Image::from_texture(SizedTexture::new(
//                 *rendered_texture_id,
//                 [256.0, 256.0],
//             )));
//
//             ui.allocate_space(egui::Vec2::new(1.0, 10.0));
//             ui.checkbox(&mut ui_state.is_window_open, "Window Is Open");
//
//             ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
//                 ui.add(egui::Hyperlink::from_label_and_url(
//                     "powered by egui",
//                     "https://github.com/emilk/egui/",
//                 ));
//             });
//         });
//
//     egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
//         // The top panel is often a good place for a menu bar:
//         egui::menu::bar(ui, |ui| {
//             egui::menu::menu_button(ui, "File", |ui| {
//                 if ui.button("Quit").clicked() {
//                     std::process::exit(0);
//                 }
//             });
//         });
//     });
//
//     egui::CentralPanel::default().show(ctx, |ui| {
//         ui.heading("Egui Template");
//         ui.hyperlink("https://github.com/emilk/egui_template");
//         ui.add(egui::github_link_file_line!(
//             "https://github.com/mvlabat/bevy_egui/blob/main/",
//             "Direct link to source code."
//         ));
//         egui::warn_if_debug_build(ui);
//
//         ui.separator();
//
//         ui.heading("Central Panel");
//         ui.label("The central panel the region left after adding TopPanel's and SidePanel's");
//         ui.label("It is often a great place for big things, like drawings:");
//
//         ui.heading("Draw with your mouse to paint:");
//         ui_state.painting.ui_control(ui);
//         egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
//             ui_state.painting.ui_content(ui);
//         });
//     });
//
//     egui::Window::new("Window")
//         .vscroll(true)
//         .open(&mut ui_state.is_window_open)
//         .show(ctx, |ui| {
//             ui.label("Windows can be moved by dragging them.");
//             ui.label("They are automatically sized based on contents.");
//             ui.label("You can turn on resizing and scrolling if you like.");
//             ui.label("You would normally chose either panels OR windows.");
//         });
//
//     if invert {
//         ui_state.inverted = !ui_state.inverted;
//     }
//     if load || invert {
//         // If an image is already added to the context, it'll return an existing texture id.
//         if ui_state.inverted {
//             *rendered_texture_id = contexts.add_image(images.bevy_icon_inverted.clone_weak());
//         } else {
//             *rendered_texture_id = contexts.add_image(images.bevy_icon.clone_weak());
//         };
//     }
//     if remove {
//         contexts.remove_image(&images.bevy_icon);
//         contexts.remove_image(&images.bevy_icon_inverted);
//     }
// }

struct Painting {
    lines: Vec<Vec<egui::Vec2>>,
    stroke: egui::Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.stroke, "Stroke");
            ui.separator();
            if ui.button("Clear Painting").clicked() {
                self.lines.clear();
            }
        })
            .response
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());
        let rect = response.rect;

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = pointer_pos - rect.min;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
        }

        for line in &self.lines {
            if line.len() >= 2 {
                let points: Vec<egui::Pos2> = line.iter().map(|p| rect.min + *p).collect();
                painter.add(egui::Shape::line(points, self.stroke));
            }
        }
    }
}
