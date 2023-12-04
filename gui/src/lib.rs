pub mod resource;
pub mod message;
pub mod event;
pub mod system;
pub mod plugin;
pub mod generator;
mod hilbert_curve;
pub mod turtle;
pub mod l_system;
pub mod turtle_l_system;

use bevy::prelude::{Component, Event};
use bevy::tasks::Task;


#[derive(Debug, Component)]
pub struct ComputeTask(pub Task<bool>);

#[derive(Event, Debug)]
pub struct LoadingEvent(bool);
