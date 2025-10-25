pub mod settings;

use bevy::prelude::*;
use strum_macros::{Display, EnumIter, EnumString};

use crate::layer::settings::LayerSettings;

#[derive(Clone, EnumString, EnumIter, Display)]
pub enum LayerType {
    Hills,
    Water,
    Mountain,
    Clamp,
    SimpleErosion,
}

#[derive(Clone)]
pub struct Layer {
    pub layer_type: LayerType,
    pub seed: u64,
    pub settings: LayerSettings,
}

#[derive(Event)]
pub struct AddLayer;

impl Layer {
    pub fn new(layer_type: LayerType, seed: u64) -> Self {
        Self {
            layer_type: layer_type.clone(),
            seed,
            settings: LayerSettings::from_layer_type(layer_type),
        }
    }

    pub fn is_blendable(&self) -> bool {
        match self.layer_type {
            LayerType::Clamp => false,
            _ => true,
        }
    }
}
