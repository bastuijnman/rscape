#[derive(Clone)]
pub enum LayerType {
    Hills,
    Water,
    Mountain,
}

impl LayerType {
    pub fn to_string(&self) -> String {
        match &self {
            LayerType::Hills => "Hills".to_string(),
            LayerType::Water => "Water".to_string(),
            LayerType::Mountain => "Mountain".to_string(),
        }
    }
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
