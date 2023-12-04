// pub type RgbaData<B> = Vec<B>;

use std::collections::HashMap;
use std::fmt::Debug;

use num_traits::ToPrimitive;

pub trait RgbaData: Sized + Send + Sync + 'static + Into<[u8; 4]> {
    type T: Into<u8>;
    fn r(&self) -> Self::T;
    fn g(&self) -> Self::T;
    fn b(&self) -> Self::T;
    fn a(&self) -> Self::T;
}

pub trait Complex: Send + Sync + 'static {
    type T;
    fn real(&self) -> Self::T;

    fn imaginary(&self) -> Self::T;
}

pub trait Viewport: Send + Sync + 'static {
    type T;
    fn top_left(&self) -> &dyn Complex<T=Self::T>;

    fn bottom_right(&self) -> &dyn Complex<T=Self::T>;

    fn width(&self) -> Self::T;

    fn height(&self) -> Self::T;
}

type BoxedComplex<T> = Box<dyn Complex<T=T>>;

pub trait Generator: GeneratorSettings + Send + Sync + 'static {
    type B;
    type T;
    // type S;

    type C;

    fn new(config: Self::C, max_iterations: u32) -> Self where Self: Sized;

    fn data(&self) -> Vec<Self::B>;

    fn zoom(&mut self, center: (u32, u32), radius: u32) -> (
        BoxedComplex<Self::T>,
        BoxedComplex<Self::T>
    );

    // fn update_settings(&mut self, settings: &[Self::S]);

    // fn settings(&self) -> Vec<Self::S>;

    fn reset(&mut self);

    fn recalculate(&mut self, refresh: bool);

    fn redraw(&mut self);

    fn viewport(&self) -> &dyn Viewport<T=Self::T>;
}

pub struct GeneratorSetting {
    pub label: String,
    pub default: GeneratorValue,
    pub value: GeneratorValue,
    // pub value: Box<dyn Any + Send + Sync + 'static>,
    // fn label(&self) -> String;
    // fn get_default(&self) -> Box<dyn Any>;
    // fn value(&mut self) -> Box<dyn Any>;
}

type Primitive = dyn ToPrimitive + Send + Sync + 'static;
pub type BoxedPrimitive = Box<Primitive>;
pub type Value = BoxedPrimitive;
pub type Start = BoxedPrimitive;
pub type End = BoxedPrimitive;
pub type Step = BoxedPrimitive;

pub enum GeneratorValue {
    Bool(bool),
    Range((Value, Start, End, Option<Step>)),
    Viewport(GeneratorViewport),
}

pub struct GeneratorViewport {
    pub top_left: (BoxedPrimitive, BoxedPrimitive),
    pub bottom_right: (BoxedPrimitive, BoxedPrimitive),
    pub width: BoxedPrimitive,
    pub height: BoxedPrimitive,
}

impl GeneratorViewport {
    pub fn new(
        top_left: (impl ToPrimitive + Send + Sync + 'static, impl ToPrimitive + Send + Sync + 'static),
        bottom_right: (impl ToPrimitive + Send + Sync + 'static, impl ToPrimitive + Send + Sync + 'static),
        width: impl ToPrimitive + Send + Sync + 'static,
        height: impl ToPrimitive + Send + Sync + 'static,
    ) -> Self {
        Self {
            top_left: (Box::new(top_left.0), Box::new(top_left.1)),
            bottom_right: (Box::new(bottom_right.0), Box::new(bottom_right.1)),
            width: Box::new(width),
            height: Box::new(height),
        }
    }
}

impl GeneratorSetting {
    pub fn new(label: String, default: GeneratorValue, value: GeneratorValue) -> Self {
        Self {
            label,
            default,
            value,
        }
    }
}

impl From<&GeneratorSetting> for HashMap<String, String> {
    fn from(value: &GeneratorSetting) -> Self {
        match &value.value {
            GeneratorValue::Bool(bool) => {
                let label = value.label.to_string();
                let value = bool.to_string();

                HashMap::from([
                    (label, value)
                ])
            }
            GeneratorValue::Range((val, _, _, _)) => {
                let label = value.label.to_string();
                let value = val.to_f64().unwrap().to_string();

                HashMap::from([
                    (label, value)
                ])
            }
            GeneratorValue::Viewport(viewport) => {
                let re1 = viewport.top_left.0.to_f64().unwrap().to_string();
                let im1 = viewport.top_left.1.to_f64().unwrap().to_string();
                let re2 = viewport.bottom_right.0.to_f64().unwrap().to_string();
                let im2 = viewport.bottom_right.1.to_f64().unwrap().to_string();
                let width = viewport.width.to_f64().unwrap().to_string();
                let height = viewport.height.to_f64().unwrap().to_string();

                HashMap::from([
                    ("re1".to_string(), re1),
                    ("im1".to_string(), im1),
                    ("re2".to_string(), re2),
                    ("im2".to_string(), im2),
                    ("width".to_string(), width),
                    ("height".to_string(), height),
                ])
            }
        }
    }
}

pub trait GeneratorSettings: Send + Sync + 'static {
    fn settings(&self) -> Vec<GeneratorSetting>;

    fn update_settings(&mut self, settings: &[GeneratorSetting]);
}

pub trait GeneratorConfigOld: Send + Sync + 'static {
    type C;
}

#[derive(Debug, Default, Clone)]
pub struct GeneratorSettingsOld {
    pub hue: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub x1: Option<String>,
    pub y1: Option<String>,
    pub x2: Option<String>,
    pub y2: Option<String>,
    pub exponent: Option<u32>,
    pub iterations: Option<String>,
}

impl GeneratorConfigOld for GeneratorSettingsOld {
    type C = GeneratorSettingsOld;
}

pub enum GeneratorCommandMessage
// where S
{
    Zoom((u32, u32)),
    Settings(Vec<GeneratorSetting>),
    Reset,
}

#[derive(Debug)]
pub enum GeneratorOutputMessage<B, V>
    where B: RgbaData,
          V: Viewport,
{
    Loading(bool),
    Image(Vec<Vec<B>>),
    Viewport(V),
}
