use bevy_egui::egui::{Slider, Ui};

use crate::layer::settings::ClampSettings;

// Handles the UI for the clamp layer
pub fn clamp_settings(ui: &mut Ui, settings: &mut ClampSettings) -> bool {
    let mut min = settings.clamp[0];
    let mut max = settings.clamp[1];

    let min_ui = ui.add(Slider::new(&mut min, 0.0..=1.0).text("Min:"));
    let max_ui = ui.add(Slider::new(&mut max, 0.0..=1.0).text("Max:"));

    if min_ui.drag_stopped() || max_ui.drag_stopped() {
        settings.clamp = [min, max];
        return true;
    }

    false
}
