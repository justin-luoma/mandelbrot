use bevy::prelude::{Deref, Resource};
use crossbeam_channel::{Receiver, Sender};

use mandelbrot_gui::{GeneratorRxMessage, GeneratorTxMessage, RgbaData};

#[derive(Resource)]
pub struct GeneratorUiReceiver<R: RgbaData + Send + Sync>(pub Receiver<GeneratorTxMessage<R>>);

#[derive(Resource, Deref)]
pub struct GeneratorRxSender(pub Sender<GeneratorRxMessage>);