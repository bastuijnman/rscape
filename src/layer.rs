use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, EnumString, EnumIter, Display)]
pub enum LayerType {
    Hills,
    Water,
    Mountain,
}

#[derive(Clone)]
pub enum LayerSettingField {
    LayerType,
}

#[derive(Clone)]
pub struct HillsSettings {
    pub test: f64,
}

#[derive(Clone)]
pub enum LayerSettings {
    Hills(HillsSettings),
}

#[derive(Clone)]
pub struct Layer {
    pub layer_type: LayerType,
    pub seed: u64,
    pub settings: LayerSettings,
}

impl Layer {
    pub fn new(layer_type: LayerType, seed: u64) -> Self {
        Self {
            layer_type,
            seed,
            settings: match layer_type {
                _ => LayerSettings::Hills(HillsSettings { test: 1.0 }),
            },
        }
    }
}
