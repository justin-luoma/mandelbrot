use std::collections::HashMap;
use std::marker::PhantomData;

use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{PointerButton, Pos2, Sense, Ui};
use bevy_egui::egui::ecolor::Hsva;
use image::{DynamicImage, RgbaImage};
use rayon::prelude::*;

use fractal_generator_gui::{Generator, GeneratorCommandMessage, GeneratorConfigOld, GeneratorOutputMessage, RgbaData, Viewport};

use crate::event::{GeneratorEvent, UiEvent};
use crate::hilbert_curve::i_to_xy;
use crate::l_system::{LStr, LSystem};
use crate::plugin::state::GeneratorState;
use crate::resource::{GeneratorResource, GeneratorStateResource, GeneratorWindowSettings, ImageRes, SettingsResource};
use crate::turtle::Turtle;
use crate::turtle_l_system::{Rules, TurtleLSystem};

pub(crate) mod state;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    Config,
    Run,
}

pub struct GeneratorPlugin<G, B, V, C, T> {
    generator_phantom: PhantomData<G>,
    rgba_phantom: PhantomData<B>,
    viewport_phantom: PhantomData<V>,
    config_phantom: PhantomData<C>,
    complex_phantom: PhantomData<T>,
}

impl<G, B, V, C, T> Plugin for GeneratorPlugin<G, B, V, C, T>
    where G: Generator,
          B: RgbaData + Into<[u8; 4]>,
          V: Viewport,
          C: GeneratorConfigOld,
          T: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app
            .add_state::<AppState>()
            .add_event::<UiEvent<B, V>>()
            .add_event::<GeneratorEvent>()
            .add_systems(Startup,
                         (
                             Self::state_transition_system
                         ).run_if(in_state(AppState::Config)),
            )
            .add_systems(PostStartup,
                         (
                             Self::init_system
                         ).run_if(resource_added::<GeneratorResource<B, T, C>>()),
            )
            .add_systems(
                Update,
                (
                    Self::ui_event_handler,
                    Self::generator_system,
                    Self::ui_system,
                )
                    .run_if(in_state(AppState::Run)),
            )
            .add_systems(
                Update,
                (Self::settings_changed_system)
                    .run_if(resource_exists_and_changed::<SettingsResource>()),
            );
    }
}

impl<G, B, V, C, T> GeneratorPlugin<G, B, V, C, T>
    where
        G: Generator,
        B: RgbaData + Into<[u8; 4]>,
        V: Viewport,
        C: GeneratorConfigOld,
        T: 'static,
// S: 'static + GeneratorConfigOld,
{
    pub fn new() -> Self {
        Self::default()
    }

    fn init_system(
        mut generator: ResMut<GeneratorResource<B, T, C>>,
        mut commands: Commands,
        mut asset: ResMut<Assets<Image>>,
        window: Res<GeneratorWindowSettings>,
    ) {
        generator.as_mut().0.recalculate(true);
        generator.as_mut().0.redraw();
        let image = generator
            .0
            .data()
            .into_par_iter()
            .flat_map(|pixel| pixel.into())
            .collect();
        let image = RgbaImage::from_raw(window.width as u32, window.height as u32, image).unwrap();
        let image = DynamicImage::from(image);
        let handle = asset.add(Image::from_dynamic(image, false));
        commands.insert_resource(ImageRes(handle));

        let settings = generator.0.settings();

        let state = GeneratorState::from(&settings);

        commands.insert_resource(SettingsResource(settings));
        commands.insert_resource(GeneratorStateResource(state));
    }

    fn ui_event_handler(
        mut ui_event: EventReader<UiEvent<B, V>>,
        window: Res<GeneratorWindowSettings>,
        mut asset: ResMut<Assets<Image>>,
        mut commands: Commands,
    )
    {
        for e in ui_event.iter() {
            match &e.0 {
                GeneratorOutputMessage::Loading(is_loading) => {}
                GeneratorOutputMessage::Image(image) => {
                    let image: Vec<_> = image
                        .iter()
                        .flat_map(|row| row
                            .iter()
                            .flat_map(|pixel| [
                                pixel.r().into(),
                                pixel.g().into(),
                                pixel.b().into(),
                                pixel.a().into()
                            ]))
                        .collect();
                    let image = RgbaImage::from_raw(window.width as u32, window.height as u32, image)
                        .unwrap();
                    let image = image::DynamicImage::from(image);
                    let handle = asset.add(Image::from_dynamic(image, false));
                    commands.insert_resource(ImageRes(handle));
                }
                GeneratorOutputMessage::Viewport(viewport) => {}
            }
        }
    }

    fn generator_system(
        mut generator: ResMut<GeneratorResource<B, T, C>>,
        mut rx: EventReader<GeneratorEvent>,
        // mut initialized: Local<bool>,
    ) {
        // if !*initialized {
        //     *initialized = true;
        //     generator.0.as_mut().recalculate(true);
        // }
        for e in rx.iter() {
            match &e.0 {
                GeneratorCommandMessage::Zoom(_) => {}
                GeneratorCommandMessage::Settings(settings) => {
                    generator.as_mut().0.update_settings(settings);
                }
                GeneratorCommandMessage::Reset => {
                    generator.0.reset();
                }
            }
        }
    }

    fn state_transition_system(
        generator: Option<Res<GeneratorResource<B, T, C>>>,
        mut initialized: Local<bool>,
        mut state: ResMut<NextState<AppState>>,
    ) {
        if !*initialized && generator.is_some() {
            *initialized = true;
            state.0 = Some(AppState::Run);
        }
    }

    fn ui_system(
        mut contexts: EguiContexts,
        mut initialized: Local<bool>,
        window_settings: Res<GeneratorWindowSettings>,
        mut image_id: Local<egui::TextureId>,
        image: Option<Res<ImageRes>>,
        settings: Res<GeneratorStateResource>,
    ) {
        if let Some(image) = image {
            if !*initialized {
                *initialized = true;
            }
            *image_id = contexts.add_image(image.0.clone_weak());
        }

        let ctx = contexts.ctx_mut();

        egui::SidePanel::right("right-panel")
            .exact_width(window_settings.sidebar_width)
            .resizable(false)
            .show(ctx, |ui| {
                egui::Grid::new("right-panel-grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Right Panel");
                        ui.end_row();
                        for (l, v) in settings.0.settings.iter() {
                            ui.label(format!("{l} : {v}"));
                            ui.end_row();
                        }
                    });
            });

        let image_area = egui::Area::new("main")
            .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO)
            // .fixed_pos(pos2(0., 0.))
            .enabled(*initialized)
            .show(ctx, |ui| {
                if *initialized {
                    ui.set_max_width(window_settings.width);
                    ui.vertical_centered_justified(|ui| {
                        // Self::draw_hilbert_curve(ui);
                        // Self::draw_dragon_curve(ui);
                        // Self::draw_sierpinski_square_curve(ui);
                        // ui.add(egui::widgets::Image::from_texture(SizedTexture::new(
                        //     *image_id,
                        //     [window_settings.width, window_settings.height],
                        // )));
                    });
                } else {
                    ui.horizontal_centered(|ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(window_settings.width / 2.);
                            ui.label("Loading...");
                        });
                    });
                }
                // Self::draw_dragon_curve(ui);
            })
            .response
            .interact(Sense::click());

        if image_area.clicked_by(PointerButton::Primary) {
            if let Some(pos) = image_area.interact_pointer_pos() {
                if pos.x < window_settings.width && pos.y < window_settings.height {}
            }
        }

        // let lines: Vec<_> = vertices
        //     .par_windows(2)
        //     .map(|v| (v[0], v[1]))
        //     .map(|(p1, p2)| {
        //         egui::Shape::line_segment(
        //             [
        //                 Pos2::new(p1.0 as f32 * 500. + WIDTH / 2., p1.1 as f32 * 500. +
        //                     HEIGHT /
        //                         2.),
        //                 Pos2::new(p2.0 as f32 * 500. + WIDTH / 2., p2.1 as f32 * 500. + HEIGHT /
        //                     2.)
        //             ],
        //             egui::Stroke::new(
        //                 5.,
        //                 egui::Color32::WHITE,
        //             ),
        //         )
        //     }).collect();
        // ui.painter().extend(lines);


        egui::TopBottomPanel::bottom("bottom-panel")
            .resizable(false)
            .exact_height(window_settings.bottom_bar_height)
            .show(ctx, |ui| {
                if !*initialized {
                    ui.add(egui::Spinner::new());
                }
            });
    }

    fn settings_changed_system(
        mut settings: ResMut<SettingsResource>,
    ) {
        dbg!("settings changed");
    }

    fn draw_hilbert_curve(ui: &mut Ui) {
        let size = 256;
        let coords: Vec<_> = (0..size * size).into_par_iter().map(|i| (i, i_to_xy(i, size)))
            .collect();
        let lines: Vec<_> = coords.par_windows(2)
            .map(|v| (v[0], v[1]))
            .map(|((i, p1), (_, p2))| {
                let scale = 1024. / size as f32;
                let stroke_width = 0.5;
                let offset = 1.;
                let offset = offset + stroke_width;
                egui::Shape::line_segment(
                    [
                        Pos2::new(
                            p1.0 as f32 * scale + offset,
                            p1.1 as f32 * scale + offset,
                        ),
                        Pos2::new(
                            p2.0 as f32 * scale + offset,
                            p2.1 as f32 * scale + offset,
                        )
                    ],
                    egui::Stroke::new(
                        stroke_width,
                        egui::Color32::from(Hsva::new(
                            i as f32 / (size * size) as f32,
                            1.,
                            1.,
                            1.,
                        )),
                    ),
                )
            }).collect();
        ui.painter().extend(lines);
    }

    fn draw_dragon_curve(ui: &mut Ui) {
        let center = Pos2::new(1000., 1000.);
        ui.painter().circle(
            center,
            1.,
            egui::Color32::YELLOW, /* stroke */
            egui::Stroke::new(1., egui::Color32::YELLOW),
        );
        let mut turtle = Turtle::new(ui, center);
        let rules = HashMap::from([
            ('x', LStr::from("x+yf+")),
            ('y', LStr::from("-fx-y")),
            ('f', LStr::from("f")),
            ('+', LStr::from("+")),
            ('-', LStr::from("-")),
        ]);
        let start = LStr::from("fx");
        let l_system = LSystem::new(&start, &rules);
        let mut iter = l_system.iter();
        let rules = Rules::default();
        let mut draw = TurtleLSystem::new(&mut turtle, &mut iter, &rules);
        draw.draw(13, 10.);
    }

    fn draw_sierpinski_square_curve(ui: &mut Ui) {
        let center = Pos2::new(1000., 1000.);
        ui.painter().circle(
            center,
            1.,
            egui::Color32::YELLOW, /* stroke */
            egui::Stroke::new(1., egui::Color32::YELLOW),
        );
        let mut turtle = Turtle::new(ui, center);
        let rules = HashMap::from([
            ('x', LStr::from("xf−f+f−xf+f+xf−f+f−x")),
        ]);
        let start = LStr::from("f+xf+f+xf");
        let l_system = LSystem::new(&start, &rules);
        let mut iter = l_system.iter();
        let mut rules = Rules::reversed();
        rules.forward.push('g');
        let mut draw = TurtleLSystem::new(&mut turtle, &mut iter, &rules);
        draw.draw(5, 50.);
    }
}

impl<G, B, V, C, T> Default for GeneratorPlugin<G, B, V, C, T> {
    fn default() -> Self {
        Self {
            generator_phantom: PhantomData,
            rgba_phantom: PhantomData,
            viewport_phantom: PhantomData,
            config_phantom: PhantomData,
            complex_phantom: PhantomData,
            // settings_phantom: PhantomData,
        }
    }
}
