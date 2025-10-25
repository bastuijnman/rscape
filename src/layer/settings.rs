use crate::layer::LayerType;

#[derive(Clone)]
pub struct HillsSettings {}

#[derive(Clone)]
pub struct ClampSettings {
    pub clamp: [f64; 2],
}

#[derive(Clone)]
pub struct SimpleErosionSettings {
    pub iterations: usize,
    pub max_angle: f64,
    pub erosion_factor: f64,
}

#[derive(Clone)]
pub struct MountainSettings {
    pub scale: f64,
}

#[derive(Clone)]
pub enum LayerSettings {
    Hills(HillsSettings),
    Clamp(ClampSettings),
    SimpleErosion(SimpleErosionSettings),
    Mountain(MountainSettings),
}

impl LayerSettings {
    pub fn from_layer_type(layer_type: LayerType) -> LayerSettings {
        match layer_type {
            LayerType::Clamp => LayerSettings::Clamp(ClampSettings { clamp: [0.0, 1.0] }),
            LayerType::SimpleErosion => LayerSettings::SimpleErosion(SimpleErosionSettings {
                iterations: 10,
                max_angle: 0.05,
                erosion_factor: 0.5,
            }),
            LayerType::Mountain => LayerSettings::Mountain(MountainSettings { scale: 0.5 }),
            _ => LayerSettings::Hills(HillsSettings {}),
        }
    }
}
