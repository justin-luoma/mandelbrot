use bevy::prelude::{Deref, Resource};
use crossbeam_channel::{Receiver, Sender};

use fractal_generator_gui::{GeneratorCommandMessage, GeneratorConfigOld, GeneratorOutputMessage, RgbaData, Viewport};

#[derive(Resource)]
pub struct GeneratorUiReceiver<R: RgbaData + Send + Sync, V: Viewport + Send + Sync>(
    pub Receiver<GeneratorOutputMessage<R, V>>
);

#[derive(Resource, Deref)]
pub struct GeneratorRxSender(pub Sender<GeneratorCommandMessage>);