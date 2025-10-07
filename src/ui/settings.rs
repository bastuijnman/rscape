use crate::{
    layer::{Layer, LayerSettingField, LayerType},
    ui::dropdown::{Dropdown, dropdown},
};

use bevy::prelude::*;
use strum::IntoEnumIterator;

#[derive(Event)]
pub struct SettingChanged<T> {
    pub field: LayerSettingField,
    pub value: T,
}

pub fn settings(layer: Layer) -> impl Bundle {
    match layer.layer_type {
        _ => hills_settings(layer),
    }
}

fn hills_settings(layer: Layer) -> impl Bundle {
    let type_options = LayerType::iter().map(|lt| lt.to_string()).collect();
    (
        Node { ..default() },
        children![dropdown(Dropdown {
            field: LayerSettingField::LayerType,
            options: type_options,
            value: layer.layer_type.to_string(),
            label: "Layer Type:".to_string(),
        })],
    )
}
