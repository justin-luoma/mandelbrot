use std::ops::Deref;
use num_traits::ToPrimitive;

// pub type RgbaData<B> = Vec<B>;

pub trait RgbaData {
    fn data(&self) -> [f32; 4];
}

pub trait Generator<T, D>
    where T: ToPrimitive {
    fn data(&self) -> D where D: Deref<Target=[T]>;
}

#[derive(Debug, Default, Clone)]
pub struct GeneratorSettings {
    pub hue: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub x1: Option<String>,
    pub y1: Option<String>,
    pub x2: Option<String>,
    pub y2: Option<String>,
    pub exponent: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum GeneratorRxMessage {
    Zoom((u32, u32)),
    Settings(GeneratorSettings),
    Reset,
}

#[derive(Debug)]
pub enum GeneratorTxMessage<R> where R: RgbaData + Send + Sync {
    Loading(bool),
    Image(Vec<Vec<R>>),
}
