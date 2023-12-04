use bevy::prelude::Event;
use fractal_generator_gui::{GeneratorCommandMessage, GeneratorConfigOld, GeneratorOutputMessage, RgbaData, Viewport};

#[derive(Event)]
pub struct UiEvent<R: RgbaData + Send + Sync, V: Viewport + Send + Sync>(
    pub GeneratorOutputMessage<R, V>
);

#[derive(Event)]
pub struct GeneratorEvent(pub GeneratorCommandMessage);
