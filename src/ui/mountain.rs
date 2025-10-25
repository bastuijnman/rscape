use bevy_egui::egui::{Slider, Ui};

use crate::layer::settings::MountainSettings;

// Handles the UI for the clamp layer
pub fn mountain_settings(ui: &mut Ui, settings: &mut MountainSettings) -> bool {
    let mut scale = settings.scale;

    let scale_ui = ui.add(Slider::new(&mut scale, 0.0..=1.0).text("Scale:"));

    if scale_ui.drag_stopped() {
        settings.scale = scale;
        return true;
    }

    false
}
