use std::collections::HashMap;
use bevy::prelude::*;
use bevy::tasks::Task;

mod app;
pub mod message;


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

#[derive(Debug, Component)]
pub struct ComputeTask(pub Task<bool>);

#[derive(Event, Debug)]
pub struct LoadingEvent(bool);

#[derive(Debug, Resource, Default)]
pub struct EntityGrid(pub HashMap<(i32, i32), Entity>);

// #[derive(Debug, Resource)]
// pub struct Generator<T>(pub Handle<mandelbrot_gui::Generator<T>>);
