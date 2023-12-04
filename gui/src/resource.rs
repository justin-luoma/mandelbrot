use bevy::prelude::{Entity, FromWorld, Image, Resource, World};
use bevy::asset::{AssetServer, Handle};
use fractal_generator_gui::{Generator, GeneratorConfigOld, GeneratorSetting, RgbaData};
use std::collections::HashMap;
use sierpinski_triangle::SierpinskiTriangle;
use crate::plugin::state::GeneratorState;

#[derive(Debug, Resource)]
pub struct Images {
    bevy_icon: Handle<Image>,
    bevy_icon_inverted: Handle<Image>,
    pub mandelbrot: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            bevy_icon: asset_server.load("icon.png"),
            bevy_icon_inverted: asset_server.load("icon_inverted.png"),
            mandelbrot: asset_server.load("mandelbrot.png"),
        }
    }
}

#[derive(Debug, Resource)]
pub struct ImageRes(pub Handle<Image>);

#[derive(Debug, Resource, Default)]
pub struct EntityGrid(pub HashMap<(i32, i32), Entity>);

#[derive(Resource)]
pub struct TriangleGenerator(pub SierpinskiTriangle);

#[derive(Resource)]
pub struct GeneratorResource<B: RgbaData, T, C>(
    pub Box<dyn Generator<B=B, T=T, C=C>>
);

#[derive(Resource)]
pub struct SettingsResource(pub Vec<GeneratorSetting>);

#[derive(Resource, Clone, Copy)]
pub struct GeneratorWindowSettings {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) scale_factor: f64,
    pub(crate) sidebar_width: f32,
    pub(crate) bottom_bar_height: f32,
    pub(crate) title: Option<&'static str>,
}

#[derive(Debug, Resource)]
pub(crate) struct GeneratorStateResource(pub GeneratorState);

impl GeneratorWindowSettings {
    pub fn new(
        title: Option<&'static str>,
        width: f32,
        height: f32,
        scale_factor: f64,
        sidebar_width: f32,
        bottom_bar_height: f32,
    ) -> Self {
        Self {
            title,
            width,
            height,
            scale_factor,
            sidebar_width,
            bottom_bar_height,
        }
    }

    pub fn total_width(&self) -> f32 {
        self.width + self.sidebar_width
    }

    pub fn total_height(&self) -> f32 {
        self.height + self.bottom_bar_height
    }
}
