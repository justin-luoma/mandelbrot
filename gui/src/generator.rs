use std::marker::PhantomData;

use bevy::app::{App, PluginGroup};
use bevy::DefaultPlugins;
use bevy::prelude::{ClearColor, Color, Msaa};
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy_egui::EguiPlugin;
use num_traits::ToPrimitive;

use fractal_generator_gui::{Generator, GeneratorConfigOld, RgbaData, Viewport};

use crate::plugin::GeneratorPlugin;
use crate::resource::{GeneratorResource, GeneratorWindowSettings};

pub struct FractalGenerator<B, V, C, T>
    where
        B: RgbaData,
        V: Viewport,
        C: GeneratorConfigOld,
        T: Send + Sync + 'static + ToPrimitive,
{
    app: App,
    byte_phantom: PhantomData<B>,
    viewport_phantom: PhantomData<V>,
    complex_phantom: PhantomData<C>,
    type_phantom: PhantomData<T>,
}
impl<B, V, C, T> FractalGenerator<B, V, C, T>
    where
        B: RgbaData,
        V: Viewport,
        C: GeneratorConfigOld,
        T: Send + Sync + 'static + ToPrimitive,
{
    pub fn new<G>(window_settings: GeneratorWindowSettings, generator: G) -> Self
        where G: Generator,
              G: Generator<B=B, C=C, T=T>,
    {
        let mut app = App::new();
        app
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(Msaa::Sample4)
            .insert_resource(window_settings)
            .insert_resource(GeneratorResource::<B, T, C>(Box::new(generator)))
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::from((
                            window_settings.total_width(),
                            window_settings.total_height()
                        ))
                            .with_scale_factor_override(window_settings.scale_factor),
                        resizable: true,
                        title: window_settings
                            .title
                            .map_or(
                                "Fractal Generator Gui".to_string(),
                                |t| t.to_string(),
                            ),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                EguiPlugin,
                GeneratorPlugin::<G, B, V, C, T>::new(),
            ))
        ;

        Self {
            app,
            byte_phantom: PhantomData,
            viewport_phantom: PhantomData,
            complex_phantom: PhantomData,
            type_phantom: PhantomData,
        }
    }

    pub fn app<F>(mut self, mut apply_fn: F) -> Self
        where F: FnMut(&mut App),
    {
        apply_fn(&mut self.app);
        self
    }

    pub fn run(&mut self) {
        self.app.run()
    }
}
