use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self},
};
use std::str::FromStr;
use strum::IntoEnumIterator;

use crate::{
    ActiveLayers,
    layer::{AddLayer, LayerType, settings::LayerSettings},
    ui::{mountain::mountain_settings, simple_erosion::simple_erosion},
};

mod clamp;
mod mountain;
mod simple_erosion;

pub fn ui(mut contexts: EguiContexts, mut layers: ResMut<ActiveLayers>, mut commands: Commands) {
    egui::Window::new("Layers").show(contexts.ctx_mut().unwrap(), |ui| {
        // Layer groups
        for (index, layer) in layers.0.clone().iter_mut().enumerate() {
            ui.push_id(index, |ui| {
                ui.collapsing(layer.layer_type.to_string(), |ui| {
                    // Select layer type
                    let mut layer_type_value = layer.layer_type.to_string();
                    egui::ComboBox::from_label("Select Type")
                        .selected_text(layer.layer_type.to_string())
                        .show_ui(ui, |ui| {
                            for i in LayerType::iter() {
                                ui.selectable_value(
                                    &mut layer_type_value,
                                    i.to_string(),
                                    i.to_string(),
                                );
                            }
                        });

                    if layer_type_value != layer.layer_type.to_string() {
                        layer.layer_type = LayerType::from_str(layer_type_value.as_str()).unwrap();
                        layer.settings = LayerSettings::from_layer_type(layer.layer_type.clone());
                        layers.0[index] = layer.clone();
                    }

                    // Set seed
                    let mut seed_value = layer.seed.to_string();
                    ui.label("Seed:");
                    let seed = ui.add(egui::TextEdit::singleline(&mut seed_value));
                    if seed.changed() {
                        layer.seed = seed_value.parse::<u64>().unwrap();
                        layers.0[index] = layer.clone();
                    }

                    // Layer type specific settings
                    // TODO: refactor out all this duplicated code
                    match &layer.settings {
                        LayerSettings::Mountain(settings) => {
                            let mut changing = settings.clone();
                            let changed = mountain_settings(ui, &mut changing);
                            if changed {
                                layer.settings = LayerSettings::Mountain(changing);
                                layers.0[index] = layer.clone();
                            }
                        }
                        LayerSettings::Clamp(settings) => {
                            let mut changing = settings.clone();
                            let changed = clamp::clamp_settings(ui, &mut changing);
                            if changed {
                                layer.settings = LayerSettings::Clamp(changing);
                                layers.0[index] = layer.clone();
                            }
                        }
                        LayerSettings::SimpleErosion(settings) => {
                            let mut changing = settings.clone();
                            let changed = simple_erosion(ui, &mut changing);
                            if changed {
                                layer.settings = LayerSettings::SimpleErosion(changing);
                                layers.0[index] = layer.clone();
                            }
                        }
                        _ => (),
                    }
                });
            });
        }

        // Adding layers
        let add_layer_button = egui::Button::new("Add Layer");
        if ui.add(add_layer_button).clicked() {
            commands.trigger(AddLayer);
        }
    });
}
