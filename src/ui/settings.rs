use crate::{
    layer::{Layer, LayerSettingField},
    ui::dropdown::{Dropdown, dropdown},
};
use bevy::prelude::*;

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
    let type_options: Vec<String> = vec!["Hills".to_string(), "Mountains".to_string()];
    (
        Node { ..default() },
        children![dropdown(Dropdown {
            field: LayerSettingField::LayerType,
            options: type_options,
            value: layer.layer_type.to_string()
        })],
    )
}
