use std::collections::HashMap;

use fractal_generator_gui::GeneratorSetting;

#[derive(Debug, Clone)]
pub(crate) struct GeneratorState {
    pub(crate) settings: HashMap<String, String>,
}

impl From<&Vec<GeneratorSetting>> for GeneratorState {
    fn from(value: &Vec<GeneratorSetting>) -> Self {
        Self {
            settings: value
                .iter()
                .map(|setting| HashMap::from(setting))
                .fold(HashMap::new(), |mut map, setting| {
                    map.extend(setting);
                    map
                }),
        }
    }
}
